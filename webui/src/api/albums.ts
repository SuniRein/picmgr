import type { PaginationParams, PaginationResponse } from './pagination';
import { api } from './base';

export interface AlbumMeta {
  id: number;
  owner_id?: number;

  name: string;
  description: string;

  is_public: string;

  created_at: string;
  updated_at: string;
}

export async function getAlbums(params: PaginationParams, signal?: AbortSignal) {
  const response = await api.get<PaginationResponse<AlbumMeta>>('/albums', { params, signal });
  return response;
}

export async function getAlbumsCount() {
  return await api.get<{ count: number }>('/albums/count');
}
