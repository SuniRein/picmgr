export interface ImageFilter {
  minWidth?: number;
  maxWidth?: number;
  minHeight?: number;
  maxHeight?: number;
  mimeType?: string;
  createdBefore?: Date;
  createdAfter?: Date;
  updatedBefore?: Date;
  updatedAfter?: Date;
  visibility: 'public' | 'private' | 'all';
  tags: string[];
}

function createDefaultFilter(): ImageFilter {
  return {
    visibility: 'all',
    tags: [],
  };
}

export const useImageFilterStore = defineStore('image-filter', () => {
  const filter = ref<ImageFilter>(createDefaultFilter());

  const activeFilterCount = computed(() => {
    const f = filter.value;
    const activeMap = {
      minWidth: f.minWidth !== undefined,
      maxWidth: f.maxWidth !== undefined,
      minHeight: f.minHeight !== undefined,
      maxHeight: f.maxHeight !== undefined,
      mimeTypes: !!f.mimeType?.trim(),
      createdBefore: f.createdBefore !== undefined,
      createdAfter: f.createdAfter !== undefined,
      updatedBefore: f.updatedBefore !== undefined,
      updatedAfter: f.updatedAfter !== undefined,
      visibility: f.visibility !== 'all',
      tags: f.tags.length > 0,
    };
    return Object.values(activeMap).filter(Boolean).length;
  });

  function reset() {
    filter.value = createDefaultFilter();
  }

  return {
    filter,
    activeFilterCount,
    reset,
  };
});
