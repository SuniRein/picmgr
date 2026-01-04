import type { AlbumMeta } from '@/api/albums';
import api from '@/api';

export const useCurrentAlbumStore = defineStore('current-album', () => {
  const albumId = ref<number | null>(null);
  const albumMeta = ref<AlbumMeta | null>(null);

  async function fetchAlbumMeta() {
    if (albumId.value === null) {
      albumMeta.value = null;
      return;
    }

    const response = await api.getAlbumDetails(albumId.value);
    albumMeta.value = response.data;
  }

  return {
    id: albumId,
    meta: readonly(albumMeta),
    fetchAlbumMeta,
  };
});
