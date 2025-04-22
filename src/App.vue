<template>

  <!-- <Navbar /> -->
  <div id="app" class="flex h-screen">
    <!-- Sidebar -->
    <div class="w-64">
      <Sidebar />
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col">
      <div class="flex-1 overflow-y-auto p-4">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script setup>
import Navbar from './components/Navbar.vue'
import Sidebar from './components/SideBar.vue'
</script>

<style>
body {
  font-family: Inter, sans-serif;
}
</style>
