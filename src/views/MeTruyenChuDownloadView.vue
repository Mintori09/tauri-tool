<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog';

const baseUrl = ref('')
const downloadedChapters = ref<string[]>([])
const done = ref(false)
const downloading = ref(false)
const stop = ref(false)
const folderPath = ref('')

async function stopDownload() {
    downloading.value = false
    await invoke('stop_download')
}

async function startDownload() {
    downloadedChapters.value = []
    done.value = false
    downloading.value = true
    stop.value = false
    await invoke('start_download', { baseUrl: baseUrl.value, folderPath: folderPath.value })
}

const selectFolder = async () => {
    const folder = await open({
        directory: true,
        multiple: false,
    })
    if (typeof folder === 'string') {
        folderPath.value = folder
        console.log(folder)
    }
}

onMounted(async () => {
    await listen<string>('chapter-downloaded', (event) => {
        downloadedChapters.value.push(event.payload)
    })

    await listen('download-finished', () => {
        done.value = true
        folderPath.value = ""
    })

    await listen('stop-download', () => {
        stop.value = true
        folderPath.value = ""
    })

    await listen('system-error', (event) => {
        alert('Lỗi hệ thống: ' + event.payload)
        folderPath.value = ""
    })
})
</script>

<template>
    <div class="container">
        <h1 class="text-2xl font-bold mb-4">📥 Trình tải truyện</h1>

        <input v-model="baseUrl" placeholder="Dán link chương 1 ở metruyenchu" class="input-field" />

        <div class="button-group">
            <button class="btn btn-start" @click="startDownload">🚀 Bắt đầu tải</button>
            <button class="btn btn-stop" @click="stopDownload" v-if="downloading && !done">
                ⛔ Dừng lại
            </button>
        </div>

        <div class="mt-4">
            <button class="btn btn-folder" @click="selectFolder">📁 Chọn thư mục</button>
        </div>

        <p v-if="folderPath.length" class="folder-path">
            📂 {{ folderPath }}
        </p>

        <div class="mt-6">
            <h2 class="text-lg font-semibold mb-2">📄 Tiến trình:</h2>
            <ul class="chapter-list">
                <li v-for="(chapter, index) in downloadedChapters" :key="index">
                    ✅ {{ chapter }}
                </li>
            </ul>
        </div>

        <div v-if="done" class="done-message">🎉 Tải xong rồi!</div>
        <div v-if="stop" class="stop-message">⛔ Đã dừng tải</div>
    </div>
</template>

<style scoped>
.container {
    max-width: 700px;
    margin: 0 auto;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: #ffffff;
    border-radius: 12px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
}

.input-field {
    width: 100%;
    padding: 12px 16px;
    border-radius: 8px;
    border: 1px solid #ccc;
    font-size: 1rem;
    margin-bottom: 1rem;
}

.button-group {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
}

.btn {
    padding: 10px 20px;
    border-radius: 8px;
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
    transition: 0.2s;
    border: none;
}

.btn-start {
    background-color: #3b82f6;
    color: white;
}

.btn-stop {
    background-color: #ef4444;
    color: white;
}

.btn-folder {
    background-color: #10b981;
    color: white;
    margin-top: 1rem;
}

.btn:hover {
    filter: brightness(1.05);
}

.folder-path {
    margin-top: 1rem;
    font-style: italic;
    color: #555;
    text-align: center;
}

.chapter-list {
    text-align: left;
    margin-top: 1rem;
    padding-left: 0;
    list-style: none;
}

.chapter-list li {
    margin-bottom: 6px;
    padding: 6px;
    background-color: #f3f4f6;
    border-radius: 6px;
}

.done-message,
.stop-message {
    margin-top: 1.5rem;
    font-weight: bold;
    font-size: 1.1rem;
}

.done-message {
    color: #16a34a;
}

.stop-message {
    color: #dc2626;
}

/* Responsive */
@media (max-width: 600px) {
    .input-field {
        width: 100%;
    }

    .button-group {
        flex-direction: column;
        width: 100%;
    }

    .btn {
        width: 100%;
    }
}
</style>
