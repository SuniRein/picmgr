import type { ImageData, ImageFilterOption, ImageSignature, ThumbnailSize } from '@/api';
import api from '@/api';

export interface ImageContext {
  albumId?: number;
}

export const useImagesStore = defineStore('images', () => {
  const items = ref<ImageData[]>([]);
  const total = ref(0);
  const isLoading = ref(false);

  const { currentPage, pageSize, resetPagination } = usePagination({ initialPageSize: 20, onPageChange: fetchImages });

  const albumId = ref<number | null>(null);

  async function setContext(context: ImageContext = {}) {
    resetPagination();
    albumId.value = context.albumId ?? null;
  }

  const imageFilter = useImageFilterStore();
  watch(() => imageFilter.filter, refresh);

  function makeFilterOption(): ImageFilterOption {
    const filter = imageFilter.filter;
    return {
      min_width: filter.minWidth,
      max_width: filter.maxWidth,
      min_height: filter.minHeight,
      max_height: filter.maxHeight,
      mime_type: filter.mimeType,
      created_before: filter.createdBefore?.toISOString().slice(0, -1),
      created_after: filter.createdAfter?.toISOString().slice(0, -1),
      updated_before: filter.updatedBefore?.toISOString().slice(0, -1),
      updated_after: filter.updatedAfter?.toISOString().slice(0, -1),
      album_id: albumId.value ?? undefined,
      is_public: filter.visibility === 'all' ? undefined : filter.visibility === 'public',
      tags: filter.tags,
    };
  }

  async function loadTotalCount() {
    let response;
    if (imageFilter.activeFilterCount > 0) {
      response = await api.getFilteredImageCount(makeFilterOption());
    }
    else if (albumId.value !== null) {
      response = await api.getImageCountInAlbum(albumId.value);
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
      if (imageFilter.activeFilterCount > 0) {
        response = await api.getFilteredImages(makeFilterOption(), params, abortController.signal);
      }
      else if (albumId.value !== null) {
        response = await api.getImagesInAlbum(albumId.value, params, abortController.signal);
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

  function getImageUrl(id: number, signature: ImageSignature) {
    return api.getImageUrl(id, signature);
  }

  function getThumbnailUrl(id: number, size: ThumbnailSize, signature: ImageSignature) {
    return api.getThumbnailUrl(id, size, signature);
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
    getThumbnailUrl,

    setTags,
  };
});

export type ReadOnlyImageData = ReturnType<typeof useImagesStore>['items'][number];
