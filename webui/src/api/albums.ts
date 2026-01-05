import type { ImageData } from './images';
import type { PaginationParams, PaginationResponse } from './pagination';
import { api } from './base';

export interface AlbumMeta {
  id: number;
  owner_id?: number;

  name: string;
  description: string;

  is_public: boolean;

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

export async function getAlbumDetails(albumId: number) {
  const response = await api.get<AlbumMeta>(`/albums/${albumId}`);
  return response;
}

export async function getImagesInAlbum(albumId: number, params: PaginationParams, signal?: AbortSignal) {
  const response = await api.get<PaginationResponse<ImageData>>(`/albums/${albumId}/images`, { params, signal });
  return response;
}

export async function getImageCountInAlbum(albumId: number) {
  const response = await api.get<{ count: number }>(`/albums/${albumId}/images/count`);
  return response;
}

export interface CreateAlbumPayload {
  name: string;
  description: string;
  is_public: boolean;
}

export async function createAlbum(payload: CreateAlbumPayload) {
  return api.post<{ id: number }>('/albums', payload);
}

export interface UpdateAlbumPayload {
  name?: string;
  description?: string;
  is_public: boolean;
}

export async function updateAlbum(albumId: number, payload: UpdateAlbumPayload) {
  return api.patch<AlbumMeta>(`/albums/${albumId}`, payload);
}

export async function deleteAlbum(albumId: number) {
  await api.delete(`/albums/${albumId}`);
}
