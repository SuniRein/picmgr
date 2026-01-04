import type { Router } from 'vue-router';
import axios from 'axios';

export const api = axios.create({
  baseURL: '/api',
  timeout: 5000,
});

// Attach Authorization header if access token exists
api.interceptors.request.use(async (config) => {
  if (!config.headers.Authorization) {
    const accessToken = await getAccessToken();
    if (accessToken)
      config.headers.Authorization = `Bearer ${accessToken}`;
  }
  return config;
});

async function getAccessToken() {
  const { getValidAccessToken } = useUserStore();
  return await getValidAccessToken();
}

// Redirect to login on 401 responses
let router: Router | null = null;

export function injectRouter(r: Router) {
  router = r;
}

api.interceptors.response.use(
  response => response,
  (error) => {
    const status = error.response?.status;
    if (status === 401 && router) {
      if (router.currentRoute.value.name !== P.LOGIN.name) {
        router.push({ name: P.LOGIN.name });
      }
    }
    return Promise.reject(error);
  },
);
