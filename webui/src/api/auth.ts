import { api } from './base';

export interface LoginCredentials {
  username: string;
  password: string;
}

export interface LoginResponse {
  access_token: string;
  refresh_token: string;
}

export async function login(credentials: LoginCredentials) {
  return await api.post<LoginResponse>('/auth/login', credentials);
}

export interface RegisterPayload {
  username: string;
  email: string;
  password: string;
}

export async function register(payload: RegisterPayload) {
  await api.post('/auth/register', payload);
}

export async function refreshToken(refreshToken: string) {
  return await api.post<{ access_token: string }>('/auth/refresh', null, { headers: { Authorization: `Bearer ${refreshToken}` } });
}
