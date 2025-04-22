<template>
  <nav class="bg-gray-900 p-1 shadow-md">
    <ul class="flex space-x-6 text-sm font-inter">
      <li>
        <router-link to="/" class="text-white hover:text-blue-400 transition-colors">
          Home
        </router-link>
      </li>
      <li>
        <router-link to="/about" class="text-white hover:text-blue-400 transition-colors">
          About
        </router-link>
      </li>
      <li>
        <router-link to="/contact" class="text-white hover:text-blue-400 transition-colors">
          Contact
        </router-link>
      </li>
      <li>
        <router-link to="/download" class="text-white hover:text-blue-400 transition-colors">
          DownloadMEtruyenchu
        </router-link>
      </li>
    </ul>
  </nav>
</template>

<script setup>
// Simple responsive dark navbar using Tailwind
</script>

<style scoped>
/* Đảm bảo rằng bạn đã cài font Inter trong dự án của mình */
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;600&display=swap');

.font-inter {
  font-family: 'Inter', sans-serif;
}
</style>
