import type { AlbumMeta } from '@/api';
import api from '@/api';

export const useAlbumsStore = defineStore('albums', () => {
  const items = ref<AlbumMeta[]>([]);
  const total = ref(0);
  const isLoading = ref(false);

  const { currentPage, pageSize } = usePagination({ initialPageSize: 20, onPageChange: fetchAlbums }); ;

  async function loadTotalCount() {
    const response = await api.getAlbumsCount();
    total.value = response.data.count;
  }

  let abortController: AbortController | null = null;
  async function fetchAlbums() {
    abortController?.abort();
    abortController = new AbortController();

    isLoading.value = true;
    try {
      const response = await api.getAlbums(
        { page: currentPage.value, size: pageSize.value },
        abortController.signal,
      );
      items.value = response.data.data;
    }
    catch (err: any) {
      if (err.name === 'CanceledError')
        return;
      console.error('Albums fetched error:', err);
    }
    finally {
      isLoading.value = false;
    }
  }

  return {
    items: readonly(items),
    total: readonly(total),
    isLoading: readonly(isLoading),

    currentPage,
    pageSize,

    loadTotalCount,
    fetchAlbums,
  };
});

export type AlbumMetaView = ReturnType<typeof useAlbumsStore>['items'][number];
