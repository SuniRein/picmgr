import api from '@/api';

export interface UserProfile {
  id: number;
  username: string;
  email: string;
  avatarUrl: string | null;
  createdAt: string;
}

const TOKEN_EXPIRY_TOLERANCE = 30 * 1000; // 30 seconds

export const useUserStore = defineStore('user', () => {
  const accessToken = useLocalStorage('access_token', '');
  const refreshToken = useLocalStorage('refresh_token', '');

  const isLoggedIn = computed(() => !!accessToken.value);

  const profile = ref<UserProfile | null>(null);

  async function login(identifier: string, password: string) {
    const response = await api.login({ identifier, password });
    const { access_token, refresh_token } = response.data;
    accessToken.value = access_token;
    refreshToken.value = refresh_token;

    try {
      await fetchProfile();
    }
    catch {
      console.warn('Failed to fetch user profile after login');
    }
  }

  async function fetchProfile() {
    const response = await api.getMe();
    const { id, username, email, avatar_url, created_at } = response.data;
    profile.value = {
      id,
      username,
      email,
      avatarUrl: avatar_url,
      createdAt: created_at,
    };
  }

  async function logout() {
    accessToken.value = '';
    refreshToken.value = '';
    profile.value = null;
  }

  const accessTokenExpTime = computed(() => {
    if (!accessToken.value)
      return 0;
    try {
      const payload = JSON.parse(atob(accessToken.value.split('.')[1]!));
      return payload.exp * 1000;
    }
    catch (e) {
      console.error('access token parse error:', e);
      return 0;
    }
  });

  let refreshPromise: Promise<string | null> | null = null;
  async function getValidAccessToken() {
    if (!accessToken.value || !refreshToken.value)
      return null;
    if (Date.now() < accessTokenExpTime.value - TOKEN_EXPIRY_TOLERANCE)
      return accessToken.value;

    if (!refreshPromise) {
      refreshPromise = (async () => {
        try {
          const response = await api.refreshToken(refreshToken.value);
          const { access_token } = response.data;
          accessToken.value = access_token;
          return access_token;
        }
        catch (e) {
          console.error('refresh token failed:', e);
          return null;
        }
        finally {
          refreshPromise = null;
        }
      })();
    }
    return refreshPromise;
  }

  return {
    isLoggedIn,
    profile,
    login,
    logout,
    fetchProfile,
    getValidAccessToken,
  };
});
