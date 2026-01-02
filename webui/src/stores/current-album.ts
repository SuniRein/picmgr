import type { AlbumMeta } from '@/api/albums';
import api from '@/api';

export const useCurrentAlbumStore = defineStore('currentAlbum', () => {
  const albumId = ref<number | null>(null);
  const albumMeta = ref<AlbumMeta | null>(null);

  function setAlbumId(id: number | null) {
    albumId.value = id;
  }

  async function fetchAlbumMeta() {
    if (albumId.value === null) {
      albumMeta.value = null;
      return;
    }

    const response = await api.getAlbumDetails(albumId.value);
    albumMeta.value = response.data;
  }

  return {
    meta: readonly(albumMeta),
    setAlbumId,
    fetchAlbumMeta,
  };
});
