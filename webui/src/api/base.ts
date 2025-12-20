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
