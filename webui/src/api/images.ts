import type { PaginationParams, PaginationResponse } from './pagination';
import { api } from './base';

export interface ImageData {
  meta: ImageMeta;
  url: string;
}

export interface ImageMeta {
  id: number;
  owner_id?: number;
  category_id?: number;

  size_bytes: number;
  width: number;
  height: number;
  mime_type: string;
  exif?: object;

  is_public: boolean;
  tags: string[];

  created_at: string;
  updated_at: string;
}

export async function getImageData(params: PaginationParams, signal?: AbortSignal) {
  const response = await api.get<PaginationResponse<ImageData>>('/images', { params, signal });
  const baseUrl = api.defaults.baseURL;
  response.data.data = response.data.data.map(item => ({
    ...item,
    url: `${baseUrl}${item.url}`,
  }));
  return response;
}

export async function getImagesCount() {
  return await api.get<{ count: number }>('/images/count');
}
