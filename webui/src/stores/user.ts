import api from '@/api';

export interface UserProfile {
  id: number;
  username: string;
  email: string;
  avatarUrl: string | null;
  createdAt: string;
}

export const useUserStore = defineStore('user', () => {
  const accessToken = useLocalStorage('access_token', '');
  const refreshToken = useLocalStorage('refresh_token', '');

  const isLoggedIn = computed(() => !!accessToken.value);

  const profile = ref<UserProfile | null>(null);

  async function login(username: string, password: string) {
    const response = await api.login({ username, password });
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

  return {
    accessToken,
    isLoggedIn,
    profile,
    login,
    logout,
    fetchProfile,
  };
});
