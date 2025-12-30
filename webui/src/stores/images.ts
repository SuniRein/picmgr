import type { ImageData } from '@/api';
import api from '@/api';

export const useImagesStore = defineStore('images', () => {
  const items = ref<ImageData[]>([]);
  const total = ref(0);
  const isLoading = ref(false);

  const currentPage = ref(1);
  const pageSize = ref(20);

  async function loadTotalCount() {
    const response = await api.getImagesCount();
    total.value = response.data.count;
  }

  let abortController: AbortController | null = null;
  async function fetchImages() {
    abortController?.abort();
    abortController = new AbortController();

    isLoading.value = true;
    try {
      const response = await api.getImageData(
        { page: currentPage.value, size: pageSize.value },
        abortController.signal,
      );
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

  watch([currentPage, pageSize], async () => {
    await fetchImages();
  });

  function getImageUrl(id: number, signature: ImageData['signature']) {
    return api.getImageUrl(id, signature);
  }

  return {
    items: readonly(items),
    total: readonly(total),
    isLoading: readonly(isLoading),

    currentPage,
    pageSize,

    loadTotalCount,
    fetchImages,
    getImageUrl,
  };
});

export type ImageDataView = ReturnType<typeof useImagesStore>['items'][number];
