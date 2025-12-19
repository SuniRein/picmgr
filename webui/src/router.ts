import { createRouter, createWebHistory } from 'vue-router';

import DashboardLayout from '@/layouts/DashboardLayout.vue';
import GalleryView from '@/views/GalleryView.vue';
import LoginView from '@/views/LoginView.vue';
import ProfileView from '@/views/ProfileView.vue';

const R = {
  HOME: 'Home',
  LOGIN: 'Login',
  IMAGES: 'Images',
  PROFILE: 'Profile',
} as const;

export const P = {
  HOME: { name: R.HOME },
  LOGIN: { name: R.LOGIN },
  IMAGES: { name: R.IMAGES },
  PROFILE: { name: R.PROFILE },
} as const;

const routes = [
  { path: '/login', name: R.LOGIN, component: LoginView },
  {
    path: '/',
    component: DashboardLayout,
    children: [
      { path: '', name: R.HOME, redirect: 'images' },
      { path: 'images', name: R.IMAGES, component: GalleryView },
      { path: 'profile', name: R.PROFILE, component: ProfileView },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
