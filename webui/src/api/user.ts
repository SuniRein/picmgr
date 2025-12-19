import { api } from './base';

export interface UserInfo {
  id: number;
  username: string;
  email: string;
  avatar_url: string | null;
  created_at: string;
}

export async function getMe() {
  return await api.get<UserInfo>('/user/me');
}
