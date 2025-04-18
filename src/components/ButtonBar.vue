<template>
    <div class="vertical-bar">
        <button class="bar-button" @click="handleClick('home')">🏠 Trang chủ</button>
        <button class="bar-button" @click="handleClick('download')">⬇️ Tải xuống</button>
        <button class="bar-button" @click="handleClick('settings')">⚙️ Cài đặt</button>
        <button class="bar-button" @click="handleClick('about')">ℹ️ Giới thiệu</button>
        <button class="bar-button" @click="handleClick('logout')">🚪 Đăng xuất</button>
    </div>
</template>

<script setup>
const handleClick = (section) => {
    console.log("Clicked:", section)
    // emit sự kiện hoặc điều hướng tùy theo app của bạn
}
</script>

<style scoped>
.vertical-bar {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    width: 200px;
    background-color: #f9fafb;
    border-right: 1px solid #e5e7eb;
    padding: 1rem;
    min-height: 100vh;
}

.bar-button {
    padding: 12px 16px;
    margin-bottom: 8px;
    background-color: #ffffff;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    text-align: left;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s, color 0.2s;
}

.bar-button:hover {
    background-color: #e0f2fe;
    color: #0369a1;
}

.bar-button:active {
    background-color: #bae6fd;
}
</style>
