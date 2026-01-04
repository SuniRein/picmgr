import type { PaginationParams, PaginationResponse } from './pagination';
import { api } from './base';

export interface ImageData {
  meta: ImageMeta;
  signature: ImageSignature;
}

export interface ImageMeta {
  id: number;
  owner_id?: number;

  size_bytes: number;
  width: number;
  height: number;
  mime_type: string;
  exif?: Record<string, any>;

  is_public: boolean;
  tags: string[];

  created_at: string;
  updated_at: string;
}

export async function getImage(id: number) {
  return await api.get<ImageData>(`/images/${id}`);
}

export async function getImages(params: PaginationParams, signal?: AbortSignal) {
  return await api.get<PaginationResponse<ImageData>>('/images', { params, signal });
}

export async function getImageCount() {
  return await api.get<{ count: number }>('/images/count');
}

export interface ImageFilterOption {
  min_width?: number;
  max_width?: number;
  min_height?: number;
  max_height?: number;
  mime_type?: string;
  created_before?: string;
  created_after?: string;
  updated_before?: string;
  updated_after?: string;
  is_public?: boolean;
  album_id?: number;
  tags: string[];
}

export async function getFilteredImages(filter: ImageFilterOption, pagination: PaginationParams, signal?: AbortSignal) {
  return await api.post<PaginationResponse<ImageData>>('/images/search', { filter, pagination }, { signal });
}

export async function getFilteredImageCount(filter: ImageFilterOption) {
  return await api.post<{ count: number }>('/images/search/count', filter);
}

export interface ImageSignature {
  exp: number;
  sig: string;
}

export function getImageUrl(id: number, signature: ImageSignature) {
  const { exp, sig } = signature;
  return `/api/images/${id}/raw/signed?exp=${exp}&sig=${sig}`;
}

export type ThumbnailSize = 'small' | 'medium' | 'large';

export function getThumbnailUrl(id: number, size: ThumbnailSize, signature: ImageSignature) {
  const { exp, sig } = signature;
  return `/api/images/${id}/thumbnails/${size}/signed?exp=${exp}&sig=${sig}`;
}

export async function uploadImageRaw(file: File, onProgress?: (percent: number) => void) {
  await api.post('/images/upload/raw', file, {
    headers: {
      'Content-Type': file.type,
    },
    onUploadProgress: (progressEvent) => {
      if (onProgress) {
        const percentCompleted = Math.round(progressEvent.progress ?? 0 * 100);
        onProgress(percentCompleted);
      }
    },
  });
}

export async function setImageTags(id: number, tags: string[]) {
  await api.put(`/images/${id}/tags`, { tags });
}

export async function getTrashedImages(params: PaginationParams, signal?: AbortSignal) {
  return await api.get<PaginationResponse<ImageData>>('/images/trash', { params, signal });
}

export async function getTrashedImageCount() {
  return await api.get<{ count: number }>('/images/trash/count');
}

export async function trashImage(id: number) {
  await api.post(`/images/${id}/trash`);
}

export async function restoreImage(id: number) {
  await api.post(`/images/${id}/restore`);
}
