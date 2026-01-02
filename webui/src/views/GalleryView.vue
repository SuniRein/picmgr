<script setup lang="ts">
import { Download, FolderPlus, Trash, Unlock } from 'lucide-vue-next';

const images = useImagesStore();

function onPageSizeChange(val: number) {
  images.pageSize = val;
  images.currentPage = 1;
}

const selectedImage = ref<ReadOnlyImageData | null>(null);

async function load() {
  images.setContext();
  await images.refresh();
}
await load();
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

      <div class="flex items-center gap-2">
        <RefreshButton :loading="images.isLoading" @click="images.refresh" />
        <PageSizeSelector :page-size="images.pageSize" @update:page-size="onPageSizeChange" />
      </div>
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
        :actions="[
          { label: '下载', icon: Download },
          { label: '设为公开', icon: Unlock },
          { label: '移入相册', icon: FolderPlus },
          { label: '删除', icon: Trash, variant: 'destructive' },
        ]"
        @open="selectedImage = img"
      />
    </div>

    <PaginationControls
      v-model:current-page="images.currentPage"
      :page-size="images.pageSize"
      :total-items="images.total"
    />

    <ImageDetailDialog
      :image="selectedImage"
      :url="selectedImage ? images.getImageUrl(selectedImage.meta.id, selectedImage.signature) : ''"
      @close="selectedImage = null"
    />
  </div>
</template>
