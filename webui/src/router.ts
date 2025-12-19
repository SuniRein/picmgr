import { createRouter, createWebHistory } from 'vue-router';

import DashboardLayout from '@/layouts/DashboardLayout.vue';
import GalleryView from '@/views/GalleryView.vue';
import LoginView from '@/views/LoginView.vue';

const routes = [
  { path: '/login', component: LoginView },
  {
    path: '/',
    component: DashboardLayout,
    children: [
      { path: '', redirect: 'images' },
      { path: 'images', component: GalleryView },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
