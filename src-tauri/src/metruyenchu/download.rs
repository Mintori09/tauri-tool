use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Emitter};
use tokio::task::JoinSet;
lazy_static! {
    static ref STOP_FLAG: AtomicBool = AtomicBool::new(false);
}

const NTHREAD: usize = 10;

// ✅ Static selector dùng lại an toàn với thread
static BREAK_WORDS_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div.break-words").expect("Failed to parse selector"));

fn get_url_metruyenchu(url: &str, num: i32) -> String {
    let re = Regex::new(r"(https://metruyencv\.com/truyen/.*/chuong-)\d+").unwrap();
    re.replace(url, |caps: &regex::Captures| format!("{}{}", &caps[1], num))
        .into_owned()
}

async fn fetch_content_metruyenchu(
    url: String,
    mut folder_path: PathBuf,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let resp = reqwest::get(&url).await;
    match resp {
        Ok(response) => {
            let resp_text = response.text().await?;
            let document = Html::parse_document(&resp_text);

            // ✅ Sử dụng selector từ static
            let selector = &*BREAK_WORDS_SELECTOR;

            let content = if let Some(node) = document.select(selector).next() {
                Ok(node.inner_html())
            } else {
                Err("Failed to find content in HTML")
            }?;

            // Làm sạch HTML
            let content = Regex::new(r"(?i)<br\s*/?><br\s*/?>")?
                .replace_all(&content, "\n")
                .to_string();
            let content = Regex::new(r"(?s)<div.*?>.*?</div>")?
                .replace_all(&content, "")
                .to_string();

            // Xử lý tiêu đề
            let first_line = content.lines().next().unwrap_or("");
            let title = Regex::new(r"(Chapter )(\d+)")?
                .replace(first_line, |caps: &regex::Captures| {
                    format!("{}{:03}", &caps[1], caps[2].parse::<i32>().unwrap_or(0))
                })
                .to_string();

            println!("Đang lưu chương: {}", title);

            // Ghi file
            fs::create_dir_all(&folder_path)?;
            folder_path.push(format!("{}.txt", title));
            let mut file = File::create(&folder_path)?;
            file.write_all(content.as_bytes())?;

            Ok(title)
        }
        Err(_) => Err("Failed to fetch the URL or parse the content.".into()),
    }
}

pub async fn download_truyencv(
    app_handle: AppHandle,
    base_url: &str,
    folder_path: &str,
) -> Result<(), Box<dyn Error>> {
    let index = Arc::new(Mutex::new(1));
    let mut set = JoinSet::new();
    let mut consecutive_errors = 0;

    println!("Bắt đầu tải từ: {}", base_url);

    loop {
        if STOP_FLAG.load(Ordering::SeqCst) {
            let _ = app_handle.emit("stop-download", "Đã dừng tải.");
            STOP_FLAG.store(false, Ordering::SeqCst);
            break;
        }

        while set.len() < NTHREAD {
            let mut num = index.lock().unwrap();
            let chap = *num;
            *num += 1;
            drop(num);

            let url = get_url_metruyenchu(base_url, chap);
            let handle = app_handle.clone();

            let mut path = PathBuf::new();
            path.push(folder_path);
            set.spawn(async move {
                let result = fetch_content_metruyenchu(url, path).await;
                if let Ok(title) = &result {
                    let _ = handle.emit("chapter-downloaded", title);
                }
                result
            });
        }

        if let Some(res) = set.join_next().await {
            match res {
                Ok(Ok(_)) => {
                    consecutive_errors = 0;
                }
                Ok(Err(_)) => {
                    let _ = app_handle.emit("download-finished", "Không tìm thấy chương!");
                    break;
                }
                Err(e) => {
                    let _ = app_handle.emit("system-error", format!("{:?}", e));
                    consecutive_errors += 1;
                    if consecutive_errors >= 5 {
                        let _ = app_handle.emit("system-error", "Đã xảy ra lỗi liên tiếp.");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

fn extract_chapter_number(path: &PathBuf) -> Option<u32> {
    let file_name = path.file_name()?.to_str()?;
    let re = Regex::new(r"(\d+)").unwrap();
    re.captures(file_name)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

pub fn merge_txt() {
    let home = env::var("HOME").unwrap_or_else(|_| ".".into());
    let dir = PathBuf::from(&home).join("Desktop/Novel");

    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(OsStr::to_str) == Some("txt"))
        .collect();

    files.sort_by_key(|path| extract_chapter_number(path).unwrap_or(0));

    for file in &files {
        println!("Found txt file: {:?}", file);
    }

    let all_bytes: Vec<u8> = files
        .into_iter()
        .flat_map(|path| {
            let mut content = fs::read(&path).unwrap_or_else(|_| {
                eprintln!("Failed to read {:?}", path);
                vec![]
            });
            content.extend(b"\n");
            content
        })
        .collect();

    let merge_path = PathBuf::from(&home).join("Desktop/merge.txt");
    let mut output = File::create(&merge_path).expect("Failed to create file");
    if let Err(e) = output.write_all(&all_bytes) {
        eprintln!("Failed to write to file: {}", e);
    } else {
        println!("Successfully merged into {}", merge_path.display());
    }
}

#[tauri::command]
pub async fn start_download(
    app: AppHandle,
    base_url: String,
    folder_path: String,
) -> Result<(), String> {
    download_truyencv(app, &base_url, &folder_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_download() {
    STOP_FLAG.store(true, Ordering::SeqCst);
}
