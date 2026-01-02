import type { ImageData } from '@/api';
import api from '@/api';

export interface ImageContext {
  albumId?: number;
}

export const useImagesStore = defineStore('images', () => {
  const items = ref<ImageData[]>([]);
  const total = ref(0);
  const isLoading = ref(false);

  const { currentPage, pageSize, resetPagination } = usePagination({ initialPageSize: 20, onPageChange: fetchImages });

  const activeAlbumId = ref<number | null>(null);

  async function setContext(context: ImageContext = {}) {
    resetPagination();
    activeAlbumId.value = context.albumId ?? null;
  }

  async function loadTotalCount() {
    let response;
    if (activeAlbumId.value !== null) {
      response = await api.getImageCountInAlbum(activeAlbumId.value);
    }
    else {
      response = await api.getImagesCount();
    }

    total.value = response.data.count;
  }

  let abortController: AbortController | null = null;
  async function fetchImages() {
    abortController?.abort();
    abortController = new AbortController();

    isLoading.value = true;
    try {
      const params = {
        page: currentPage.value,
        size: pageSize.value,
      };

      let response;
      if (activeAlbumId.value !== null) {
        response = await api.getImagesInAlbum(activeAlbumId.value, params, abortController.signal);
      }
      else {
        response = await api.getImageData(params, abortController.signal);
      }

      items.value = response.data.data;
    }
    catch (err: any) {
      if (err.name === 'CanceledError')
        return;
      console.error('Images fetched error:', err);
    }
    finally {
      isLoading.value = false;
    }
  }

  async function refresh() {
    await Promise.all([
      loadTotalCount(),
      fetchImages(),
    ]);
  }

  function getImageUrl(id: number, signature: ImageData['signature']) {
    return api.getImageUrl(id, signature);
  }

  async function setTags(id: number, tags: string[]) {
    const item = items.value.find(item => item.meta.id === id);
    if (!item) {
      console.warn(`Image with id ${id} not found in store when setting tags.`);
      return;
    }

    await api.setImageTags(id, tags);
    item.meta.tags = tags;
    triggerRef(items);
  }

  return {
    items: readonly(items),
    total: readonly(total),
    isLoading: readonly(isLoading),

    currentPage,
    pageSize,

    setContext,

    loadTotalCount,
    fetchImages,
    refresh,

    getImageUrl,

    setTags,
  };
});

export type ReadOnlyImageData = ReturnType<typeof useImagesStore>['items'][number];
