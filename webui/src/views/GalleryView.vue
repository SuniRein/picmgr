<script setup lang="ts">
import type { ImageSignature } from '@/api';
import { CheckSquare, Download, FolderPlus, Pencil, Plus, Trash2, Unlock } from 'lucide-vue-next';

const images = useImagesStore();

function onPageSizeChange(val: number) {
  images.pageSize = val;
  images.currentPage = 1;
}

const selectedImage = ref<ReadOnlyImageData | null>(null);
const editedImage = ref<ReadOnlyImageData | null>(null);

function handleDownload(id: number, signature: ImageSignature) {
  const url = images.getImageUrl(id, signature);
  const link = document.createElement('a');
  link.href = url;
  link.download = `image_${id}`;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}

const isUploadModalOpen = ref(false);

const currentPageIds = computed(() => images.items.map(item => item.meta.id));
const multiSelect = reactive(useMultiSelect(currentPageIds));

const isLightboxOpen = ref(false);
const lightboxIndex = ref(0);
const lightboxIds = ref([] as number[]);

function openLightBox() {
  isLightboxOpen.value = true;
  lightboxIndex.value = 0;
  lightboxIds.value = Array.from(multiSelect.items);
}

async function load() {
  await images.init('global');
}
await load();
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-end justify-between">
      <div class="space-y-1">
        <div class="flex items-center gap-3">
          <h2 class="text-2xl font-bold tracking-tight">
            我的图库
          </h2>
        </div>
        <p class="text-sm text-muted-foreground">
          管理和浏览您的所有图片资产
        </p>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="outline" size="sm" :disabled="multiSelect.enabled" @click="multiSelect.start">
          <CheckSquare /> 批量管理
        </Button>

        <Button size="sm" class="ml-2" @click="isUploadModalOpen = true">
          <Plus /> 上传图片
        </Button>
      </div>
    </div>

    <Separator />

    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <ImageFilter />
        <span class="text-sm text-muted-foreground">
          共 <span class="font-medium text-foreground">{{ images.total }}</span> 张图片
        </span>
      </div>
      <div class="flex items-center gap-2">
        <RefreshButton :loading="images.isLoading" @click="images.refresh" />
        <PageSizeSelector :page-size="images.pageSize" @update:page-size="onPageSizeChange" />
      </div>
    </div>

    <div
      class="
        grid grid-cols-2 gap-2
        md:grid-cols-3
        lg:grid-cols-4
        xl:grid-cols-5
      "
    >
      <ImageCard
        v-for="img in images.items"
        :key="img.meta.id"
        :url="images.getThumbnailUrl(img.meta.id, 'medium', img.signature)"
        :actions="[
          { label: '下载', icon: Download, handler: () => handleDownload(img.meta.id, img.signature) },
          { label: '设为公开', icon: Unlock },
          { label: '编辑图片', icon: Pencil, handler: () => editedImage = img },
          { label: '移入相册', icon: FolderPlus },
          { label: '删除', icon: Trash2, variant: 'destructive', handler: () => images.trashImage(img.meta.id) },
        ]"
        :selection-mode="multiSelect.enabled"
        :selected="multiSelect.items.has(img.meta.id)"
        @open="selectedImage = img"
        @toggle-select="val => multiSelect.toggleSelect(img.meta.id, val)"
      >
        <template #extra-info>
          <span
            v-if="img.meta.is_public"
            class="flex items-center justify-center gap-1 px-1 text-xs text-muted-foreground/90"
          >
            <Unlock class="h-3 w-3" /> 公开
          </span>
        </template>
      </ImageCard>
    </div>

    <PaginationControls
      v-model:current-page="images.currentPage"
      :page-size="images.pageSize"
      :total-items="images.total"
    />

    <ImageUploadModal v-model:open="isUploadModalOpen" />

    <ImageDetailModal
      :image="selectedImage"
      :url="selectedImage ? images.getImageUrl(selectedImage.meta.id, selectedImage.signature) : ''"
      @update:tags="tags => images.setTags(selectedImage!.meta.id, tags)"
      @close="selectedImage = null"
    />

    <ImageEditor
      :src="editedImage ? images.getImageUrl(editedImage.meta.id, editedImage.signature) : ''"
      @close="editedImage = null"
    />

    <ImageMultiSelectBar
      v-model:open="multiSelect.enabled"
      :selected="multiSelect.items.size"
      @select-all="multiSelect.selectAll"
      @play="openLightBox"
    />

    <ImageLightBox
      v-model:open="isLightboxOpen"
      v-model:index="lightboxIndex"
      :ids="lightboxIds"
    />
  </div>
</template>
