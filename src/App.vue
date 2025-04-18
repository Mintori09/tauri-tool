<script lang="ts" setup>
import DownloadTruyen from './views/MeTruyenChuDownloadView.vue'
import VerticalButtonBar from './components/ButtonBar.vue'
</script>

<template>
    <div class="main-layout">
        <VerticalButtonBar />
        <div class="content-area">
            <DownloadTruyen />
        </div>
    </div>
</template>

<style scoped>
.main-layout {
    display: flex;
    min-height: 100vh;
    background-color: #f0f0f0;
}

.content-area {
    flex-grow: 1;
    padding: 20px;
}
</style>
