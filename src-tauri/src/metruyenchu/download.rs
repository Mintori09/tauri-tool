use regex::Regex;
use scraper::{Html, Selector};
use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Emitter, Manager};
use tokio::task::JoinSet;

const NTHREAD: usize = 10;

fn get_url_metruyenchu(url: &str, num: i32) -> String {
    let re = Regex::new(r"(https://metruyencv\.com/truyen/.*/chuong-)\d+").unwrap();
    re.replace(url, |caps: &regex::Captures| format!("{}{}", &caps[1], num))
        .into_owned()
}

async fn fetch_content_metruyenchu(
    url: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let resp = reqwest::get(&url).await;

    match resp {
        Ok(response) => {
            let resp_text = response.text().await?;
            let document = Html::parse_document(&resp_text);
            let selector = Selector::parse("div.break-words").unwrap();
            let content = document
                .select(&selector)
                .next()
                .map(|node| node.inner_html())
                .unwrap_or_else(|| "Không tìm thấy nội dung!".to_string());

            let content = Regex::new(r"(?i)<br\s*/?><br\s*/?>")
                .unwrap()
                .replace_all(&content, "\n");

            let content = Regex::new(r"(?s)<div.*?>.*?</div>")
                .unwrap()
                .replace_all(&content, "");

            let first_line = content.lines().next().unwrap_or("");
            let title = Regex::new(r"(Chapter )(\d+)")
                .unwrap()
                .replace(first_line, |caps: &regex::Captures| {
                    format!("{}{:03}", &caps[1], caps[2].parse::<i32>().unwrap_or(0))
                })
                .to_string();

            println!("Đang lưu chương: {}", title);
            let mut path = PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".into()));
            path.push("Desktop/Novel");
            fs::create_dir_all(&path)?;

            path.push(format!("{}.txt", title));
            let mut file = File::create(&path)?;
            file.write_all(content.as_bytes())?;
            Ok(title)
        }
        Err(_) => Err("Failed to fetch the URL or parse the content.".into()),
    }
}


pub async fn download_truyencv(
    app_handle: AppHandle,
    base_url: &str,
) -> Result<(), Box<dyn Error>> {
    let index = Arc::new(Mutex::new(1));
    let mut set = JoinSet::new();
    let mut consecutive_errors = 0;
    print!("{}",base_url);

    loop {
        while set.len() < NTHREAD {
            let mut num = index.lock().unwrap();
            let chap = *num;
            *num += 1;
            drop(num);

            let url = get_url_metruyenchu(base_url, chap);
            let handle = app_handle.clone();

            set.spawn(async move {
                let result = fetch_content_metruyenchu(url).await;
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
                Ok(Err(e)) => {
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
