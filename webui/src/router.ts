import type { RouteRecordRaw } from 'vue-router';
import { createRouter, createWebHistory } from 'vue-router';

import DashboardLayout from '@/layouts/DashboardLayout.vue';
import AlbumDetailView from '@/views/AlbumDetailView.vue';
import AlbumView from '@/views/AlbumView.vue';
import GalleryView from '@/views/GalleryView.vue';
import LoginView from '@/views/LoginView.vue';
import ProfileView from '@/views/ProfileView.vue';
import RegisterView from '@/views/RegisterView.vue';
import TrashImagesView from '@/views/TrashImagesView.vue';

const R = {
  HOME: 'Home',
  LOGIN: 'Login',
  REGISTER: 'Register',
  IMAGES: 'Images',
  ALBUMS: 'Albums',
  ALBUM_DETAIL: 'AlbumDetail',
  TRASH_IMAGES: 'TrashImages',
  PROFILE: 'Profile',
} as const;

export const P = {
  HOME: { name: R.HOME },
  LOGIN: { name: R.LOGIN },
  REGISTER: { name: R.REGISTER },
  IMAGES: { name: R.IMAGES },
  ALBUMS: { name: R.ALBUMS },
  ALBUM_DETAIL: (albumId: number) => ({ name: R.ALBUM_DETAIL, params: { albumId: String(albumId) } }),
  TRASH_IMAGES: { name: R.TRASH_IMAGES },
  PROFILE: { name: R.PROFILE },
} as const;

const routes = [
  { path: '/login', name: R.LOGIN, component: LoginView },
  { path: '/register', name: R.REGISTER, component: RegisterView },
  {
    path: '/',
    component: DashboardLayout,
    children: [
      { path: '', name: R.HOME, redirect: 'images' },
      { path: 'images', name: R.IMAGES, component: GalleryView },
      {
        path: 'albums',
        children: [
          { path: '', name: R.ALBUMS, component: AlbumView },
          { path: ':albumId', name: R.ALBUM_DETAIL, component: AlbumDetailView },
        ],
      },
      { path: 'trash', name: R.TRASH_IMAGES, component: TrashImagesView },
      { path: 'profile', name: R.PROFILE, component: ProfileView },
    ],
  },
] satisfies RouteRecordRaw[];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
