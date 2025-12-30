<script setup lang="ts">
import { ImageCard, ImageDetailDialog, PageSizeSelector, PaginationControls } from '@/components/gallery';

const images = useImagesStore();

function onPageSizeChange(val: number) {
  images.pageSize = val;
  images.currentPage = 1;
}

const selectedImage = ref<ImageDataView | null>(null);

onMounted(async () => {
  await images.loadTotalCount();
  await images.fetchImages();
});
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-semibold">
          最近上传
        </h2>
        <p class="text-sm text-muted-foreground">
          共 {{ images.total }} 张图片
        </p>
      </div>

      <PageSizeSelector :page-size="images.pageSize" @update:page-size="onPageSizeChange" />
    </div>

    <div
      class="
        grid grid-cols-2 gap-4
        md:grid-cols-3
        lg:grid-cols-4
        xl:grid-cols-5
      "
    >
      <ImageCard
        v-for="img in images.items"
        :key="img.meta.id"
        :title="`Image ${img.meta.id}`"
        :url="images.getImageUrl(img.meta.id, img.signature)"
        @open="selectedImage = img"
      />
    </div>

    <PaginationControls
      v-model:current-page="images.currentPage"
      :page-size="images.pageSize"
      :total-images="images.total"
    />

    <ImageDetailDialog
      :image="selectedImage"
      :url="selectedImage ? images.getImageUrl(selectedImage.meta.id, selectedImage.signature) : ''"
      @close="selectedImage = null"
    />
  </div>
</template>
