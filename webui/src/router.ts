import { createRouter, createWebHistory } from 'vue-router';

import DashboardLayout from '@/layouts/DashboardLayout.vue';
import GalleryView from '@/views/GalleryView.vue';
import LoginView from '@/views/LoginView.vue';

const R = {
  HOME: 'Home',
  LOGIN: 'Login',
  IMAGES: 'Images',
} as const;

export const P = {
  HOME: { name: R.HOME },
  LOGIN: { name: R.LOGIN },
  IMAGES: { name: R.IMAGES },
} as const;

const routes = [
  { path: '/login', name: R.LOGIN, component: LoginView },
  {
    path: '/',
    component: DashboardLayout,
    children: [
      { path: '', name: R.HOME, redirect: 'images' },
      { path: 'images', name: R.IMAGES, component: GalleryView },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
