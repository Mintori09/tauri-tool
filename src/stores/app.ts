import { defineStore } from 'pinia'

interface AppState {
  theme: 'light' | 'dark'
  isInitialized: boolean
}

export const useAppStore = defineStore('app', {
  state: (): AppState => ({
    theme: 'light',
    isInitialized: false,
  }),

  actions: {
    setTheme(theme: 'light' | 'dark') {
      this.theme = theme
      document.documentElement.classList.toggle('dark', theme === 'dark')
    },

    initialize() {
      if (this.isInitialized) return

      // Initialize app state here
      this.isInitialized = true
    },
  },

  persist: true,
})