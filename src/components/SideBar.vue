<template>
  <div class="w-64 h-full sidebar p-4 space-y-2 overflow-y-auto">
    <div v-for="(item, index) in links" :key="index" class="space-y-1">
      <button v-if="item.children" @click="toggleMenu(item.path)"
        class="w-full flex items-center justify-between px-4 py-2 rounded-lg transition-all duration-200" :class="[
          isParentRoute(item.path) ? 'text-white' : 'text-gray-300 hover:bg-gray-700 hover:text-white',
          isAnyChildActive(item) ? 'parent-active' : '' // Add a class when any child is active
        ]">
        <div class="flex items-center space-x-3 truncate">
          <component :is="item.icon" class="w-5 h-5"
            :class="isParentRoute(item.path) ? 'text-white' : 'text-gray-400'" />
          <span class="font-medium truncate text-white" :title="item.label">
            {{ item.label }}
          </span>
        </div>
        <svg :class="openMenus.includes(item.path) ? 'rotate-90' : ''"
          class="w-4 h-4 transform transition-transform duration-200 text-gray-400 " fill="none" stroke="currentColor"
          stroke-width="2" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
        </svg>
      </button>

      <router-link v-else :to="item.path"
        class="flex items-center space-x-3 px-4 py-2 rounded-lg transition-all duration-200" :class="isActiveRoute(item.path)
          ? 'bg-[#A7A6DA] text-white'
          : 'text-gray-300 hover:bg-gray-700 hover:text-white'">
        <component :is="item.icon" class="w-5 h-5 text-white" />
        <span class=" font-medium truncate text-white" :title="item.label">
          {{ item.label }}
        </span>
      </router-link>

      <Transition name="submenu">
        <div v-if="item.children && openMenus.includes(item.path)" class="pl-8 space-y-1 overflow-hidden">
          <router-link v-for="(child, cIndex) in item.children" :key="cIndex" :to="child.path"
            class="flex items-center space-x-2 px-3 py-1 rounded-lg text-sm transition-all duration-200" :class="isActiveRoute(child.path)
              ? 'submenu-item-active'
              : 'submenu-item-inactive hover:bg-gray-700'">
            <component :is="child.icon" class="w-4 h-4" />
            <span class="truncate" :title="child.label">{{ child.label }}</span>
          </router-link>
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import {
  HomeIcon,
  UserIcon,
  Cog6ToothIcon,
  ArrowLeftOnRectangleIcon,
} from '@heroicons/vue/24/outline'

const route = useRoute()

// Define the menu structure
const links = [
  { label: 'Home', path: '/', icon: HomeIcon },
  {
    label: 'Download', icon: UserIcon,
    children: [
      {
        label: 'TruyenCV',
        path: '/download',

      }
    ]
  },
  {
    label: 'About',
    path: '/about',
    icon: Cog6ToothIcon,
    children: [
      { label: 'General', path: '/settings/general', icon: Cog6ToothIcon },
      { label: 'Privacy', path: '/settings/privacy', icon: Cog6ToothIcon },
    ],
  },
  { label: 'Contact', path: '/contact', icon: ArrowLeftOnRectangleIcon },
]

// State for tracking open submenus
const openMenus = ref([])

const toggleMenu = (path) => {
  openMenus.value = openMenus.value.includes(path)
    ? openMenus.value.filter((p) => p !== path)
    : [...openMenus.value, path]
}

// Helper function to determine if a route is active
const isActiveRoute = (path) => route.path === path

// Helper function to determine if a route is a parent route
const isParentRoute = (parentPath) => route.path.startsWith(parentPath)

// Helper function to check if any child of a menu item is active
const isAnyChildActive = (item) => {
  if (item.children) {
    return item.children.some(child => isActiveRoute(child.path));
  }
  return false;
};

// Watch for route changes to automatically open parent menus
watch(
  () => route.path,
  (newPath) => {
    links.forEach((item) => {
      if (item.children && newPath.startsWith(item.path) && !openMenus.value.includes(item.path)) {
        openMenus.value.push(item.path)
      }
    })
  },
  { immediate: true } // Ensure this runs on initial load
)
</script>

<style setup lang="css">
/* Sidebar container */
.sidebar {
  background: linear-gradient(to bottom, #1f2937, #111827);
  box-shadow: 4px 0 15px rgba(0, 0, 0, 0.4);
  border-right: 1px solid rgba(255, 255, 255, 0.05);
  color: white;
  /* Default text color for the sidebar */
}

/* Submenu animation */
.submenu-enter-active,
.submenu-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
}

.submenu-enter-from,
.submenu-leave-to {
  max-height: 0;
  opacity: 0;
  transform: translateY(-5px);
}

.submenu-enter-to,
.submenu-leave-from {
  max-height: 500px;
  opacity: 1;
  transform: translateY(0);
}

/* Icon colors */
.active-icon {
  color: white;
}

.inactive-icon {
  color: white;
  /* Changed to white */
}

/* Submenu items */
.submenu-item-active {
  background-color: #A7A6DA;
  color: white;
  font-weight: 500;
}

.submenu-item-inactive {
  color: white;
  /* Changed to white */
}

/* Truncate utility */
.truncate {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Style for active parent menu item */
.parent-active .text-gray-400 {
  /* Target the icon */
  color: white;
  /* Changed to white */
}

.parent-active {
  /* You can also style the button itself, if you want */
  color: white;
  /* Changed to white */
}
</style>
