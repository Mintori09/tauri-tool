<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event'
import "./style.css"

const baseUrl = ref('')
const downloadedChapters = ref<string[]>([])
const done = ref(false)
async function startDownload() {
  downloadedChapters.value = []
  done.value = false
  console.log(baseUrl)

  await invoke('start_download', { baseUrl: baseUrl.value })
}

onMounted(async () => {

  listen<string>('chapter-downloaded', (event) => {
    downloadedChapters.value.push(event.payload)
  })

  await listen('download-finished', () => {
    done.value = true
  })

  await listen('system-error', (event) => {
    alert('Lỗi hệ thống: ' + event.payload)
  })
})
</script>

<template>
 <div style="padding: 10px;">
    <h1 >Downloader</h1>
    <input
      v-model="baseUrl"
      placeholder="Dán link chương 1 ở metruyenchu"
      class="border p-2 w-full mt-2"
    />
    <button @click="startDownload" class="bg-blue-500 text-white px-4 py-2 mt-2 rounded">
      Bắt đầu tải
    </button>

    <div class="">
      <h2 class="">Tiến trình:</h2>
      <ul>
        <li v-for="(chapter, index) in downloadedChapters" :key="index">
          ✅ {{ chapter }}
        </li>
      </ul>
    </div>

    <div v-if="done" class="mt-4 text-green-600 font-bold">🎉 Tải xong rồi!</div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>