import type { ImageData } from './images';
import type { PaginationParams, PaginationResponse } from './pagination';
import { api } from './base';

export async function getTrashedImages(params: PaginationParams, signal?: AbortSignal) {
  return await api.get<PaginationResponse<ImageData>>('/trash/images', { params, signal });
}

export async function getTrashedImageCount() {
  return await api.get<{ count: number }>('/trash/images/count');
}

export async function trashImage(id: number) {
  await api.delete(`/images/${id}`);
}

export async function restoreImage(id: number) {
  await api.post(`/trash/images/${id}/restore`);
}

export async function deleteTrashedImage(id: number) {
  return await api.delete(`/trash/images/${id}`);
}

export async function deleteAllTrashedImages() {
  return await api.delete<{ count: number }>('/trash/images');
}
