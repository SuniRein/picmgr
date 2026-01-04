import type { ImageData, ImageFilterOption, ImageSignature, ThumbnailSize } from '@/api';
import api from '@/api';

type imageContext = 'global' | 'album' | 'trash';

export const useImagesStore = defineStore('images', () => {
  const items = ref<ImageData[]>([]);
  const total = ref(0);
  const isLoading = ref(false);

  const { currentPage, pageSize, resetPagination } = usePagination({ initialPageSize: 20, onPageChange: fetchImages });

  const context = ref<imageContext>('global');

  async function init(newContext?: imageContext) {
    if (newContext)
      context.value = newContext;
    resetPagination();
    await refresh();
  }

  const { id: albumId } = storeToRefs(useCurrentAlbumStore());

  const imageFilter = useImageFilterStore();
  watch(() => imageFilter.filter, () => init);

  function makeFilterOption(): ImageFilterOption {
    const filter = imageFilter.filter;
    return {
      min_width: filter.minWidth,
      max_width: filter.maxWidth,
      min_height: filter.minHeight,
      max_height: filter.maxHeight,
      mime_type: filter.mimeType,
      created_before: filter.createdBefore?.toISOString(),
      created_after: filter.createdAfter?.toISOString(),
      updated_before: filter.updatedBefore?.toISOString(),
      updated_after: filter.updatedAfter?.toISOString(),
      album_id: albumId.value ?? undefined,
      is_public: filter.visibility === 'all' ? undefined : filter.visibility === 'public',
      tags: filter.tags,
    };
  }

  async function loadTotalCount() {
    let response;
    if (context.value === 'trash') {
      response = await api.getTrashedImageCount();
    }
    else if (imageFilter.activeFilterCount > 0) {
      response = await api.getFilteredImageCount(makeFilterOption());
    }
    else if (context.value === 'album') {
      response = await api.getImageCountInAlbum(albumId.value ?? 0);
    }
    else {
      response = await api.getImageCount();
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
      if (context.value === 'trash') {
        response = await api.getTrashedImages(params, abortController.signal);
      }
      else if (imageFilter.activeFilterCount > 0) {
        response = await api.getFilteredImages(makeFilterOption(), params, abortController.signal);
      }
      else if (context.value === 'album') {
        response = await api.getImagesInAlbum(albumId.value ?? 0, params, abortController.signal);
      }
      else {
        response = await api.getImages(params, abortController.signal);
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
    item.meta = (await api.getImage(id)).data.meta; // refresh metadata
    triggerRef(items);
  }

  async function trashImage(id: number) {
    await api.trashImage(id);
    await refresh();
  }

  async function restoreImage(id: number) {
    await api.restoreImage(id);
    await refresh();
  }

  return {
    items: readonly(items),
    total: readonly(total),
    isLoading: readonly(isLoading),

    currentPage,
    pageSize,

    init,
    refresh,
    loadTotalCount,
    fetchImages,

    getImageUrl,
    getThumbnailUrl,

    setTags,

    trashImage,
    restoreImage,
  };
});

export type ReadOnlyImageData = ReturnType<typeof useImagesStore>['items'][number];
