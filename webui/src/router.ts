import { createRouter, createWebHistory } from 'vue-router';

import GalleryView from '@/views/GalleryView.vue';

const routes = [
  { path: '/', redirect: '/images' },
  { path: '/images', component: GalleryView },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
