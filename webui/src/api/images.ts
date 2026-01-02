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

export interface ImageSignature {
  exp: number;
  sig: string;
}

export async function getImageData(params: PaginationParams, signal?: AbortSignal) {
  const response = await api.get<PaginationResponse<ImageData>>('/images', { params, signal });
  return response;
}

export async function getImagesCount() {
  return await api.get<{ count: number }>('/images/count');
}

export function getImageUrl(id: number, signature: ImageSignature) {
  const { exp, sig } = signature;
  return `/api/images/${id}/raw/signed?exp=${exp}&sig=${sig}`;
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
