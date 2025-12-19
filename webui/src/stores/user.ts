import api from '@/api';

export const useUserStore = defineStore('user', () => {
  const accessToken = useLocalStorage('access_token', '');
  const refreshToken = useLocalStorage('refresh_token', '');

  const isLoggedIn = computed(() => !!accessToken.value);

  async function login(username: string, password: string) {
    const response = await api.login({ username, password });
    const { access_token, refresh_token } = response.data;
    accessToken.value = access_token;
    refreshToken.value = refresh_token;
  }

  async function logout() {
    accessToken.value = '';
    refreshToken.value = '';
  }

  return {
    accessToken,
    isLoggedIn,
    login,
    logout,
  };
});
