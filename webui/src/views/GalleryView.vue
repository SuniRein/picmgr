<script setup lang="ts">
import { ImageCard, PageSizeSelector, PaginationControls } from '@/components/gallery';

const totalImages = ref(112);
const currentPage = ref(1);
const pageSize = ref(20);

const images = computed(() => {
  const imageOffset = (currentPage.value - 1) * pageSize.value;
  const count = Math.min(pageSize.value, totalImages.value - imageOffset);

  return Array.from({ length: count }, (_, idx) => {
    const i = imageOffset + idx;
    return {
      id: i,
      url: `https://picsum.photos/seed/${i + 100}/600/400`,
      title: `Image-${i + 1}.jpg`,
    };
  });
});

function onPageSizeChange(val: number) {
  pageSize.value = val;
  currentPage.value = 1;
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-semibold">
          最近上传
        </h2>
        <p class="text-sm text-muted-foreground">
          共 {{ totalImages }} 张图片
        </p>
      </div>

      <PageSizeSelector :page-size @update:page-size="onPageSizeChange" />
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
        v-for="img in images"
        :key="img.id"
        :image="img"
      />
    </div>

    <PaginationControls
      v-model:current-page="currentPage"
      :page-size
      :total-images
    />
  </div>
</template>
