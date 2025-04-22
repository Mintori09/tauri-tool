import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import AboutView from '../views/AboutView.vue'
import ContactView from '../views/ContactView.vue'
import MeTruyenChuView from '../components/MeTruyenChuDownloadView.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: HomeView
  },
  {
    path: '/about',
    name: 'About',
    component: AboutView
  },
  {
    path: '/contact',
    name: 'Contact',
    component: ContactView
  },
  {
    path: '/download',
    name: 'Metruyenchu',
component: MeTruyenChuView
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
