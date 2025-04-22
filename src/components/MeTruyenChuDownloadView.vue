<template>
  <div class="max-w-2xl mx-auto p-6 rounded-xl bg-white shadow-md dark:bg-gray-900 dark:text-white">
    <h1 class="text-2xl font-bold mb-4">📥 Trình tải truyện</h1>

    <input v-model="baseUrl" placeholder="Dán link chương 1 ở metruyenchu"
      class="w-full px-4 py-3 rounded-lg border border-gray-300 text-base mb-4 dark:bg-gray-800 dark:border-gray-600 dark:text-white" />

    <div class="flex flex-wrap gap-3 mb-4">
      <button class="px-5 py-2 rounded-lg font-semibold text-white bg-blue-600 hover:bg-blue-700 transition"
        @click="startDownload">
        🚀 Bắt đầu tải
      </button>

      <button class="px-5 py-2 rounded-lg font-semibold text-white bg-red-500 hover:bg-red-600 transition"
        @click="stopDownload" v-if="downloading && !done">
        ⛔ Dừng lại
      </button>
    </div>

    <button class="px-5 py-2 rounded-lg font-semibold text-white bg-green-600 hover:bg-green-700 transition mb-2"
      @click="selectFolder">
      📁 Chọn thư mục
    </button>

    <p v-if="folderPath.length" class="italic text-sm text-gray-600 dark:text-gray-400 mt-2">
      📂 {{ folderPath }}
    </p>

    <div class="mt-6 w-full">
      <h2 class="text-lg font-semibold mb-2">📄 Tiến trình:</h2>
      <ul class="space-y-2">
        <li v-for="(chapter, index) in downloadedChapters" :key="index"
          class="px-4 py-2 rounded-md bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-white">
          ✅ {{ chapter }}
        </li>
      </ul>
    </div>

    <div v-if="done" class="mt-6 font-bold text-green-600 dark:text-green-400">🎉 Tải xong rồi!</div>
    <div v-if="stop" class="mt-6 font-bold text-red-600 dark:text-red-400">⛔ Đã dừng tải</div>
  </div>
</template>

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
  await invoke('start_download', {
    baseUrl: baseUrl.value,
    folderPath: folderPath.value
  })
}

const selectFolder = async () => {
  const folder = await open({ directory: true, multiple: false })
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
