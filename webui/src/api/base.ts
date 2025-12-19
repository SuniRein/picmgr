import axios from 'axios';

export const api = axios.create({
  baseURL: '/api',
  timeout: 5000,
});

// Attach Authorization header if access token exists
api.interceptors.request.use((config) => {
  const accessToken = localStorage.getItem('access_token');
  if (!config.headers.Authorization && accessToken) {
    config.headers.Authorization = `Bearer ${accessToken}`;
  }
  return config;
});
