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
