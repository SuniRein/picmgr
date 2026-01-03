<script setup lang="ts">
import { Download, FolderPlus, Plus, Trash, Unlock } from 'lucide-vue-next';
import { ImageUploadModal } from '@/components/upload';

const images = useImagesStore();

function onPageSizeChange(val: number) {
  images.pageSize = val;
  images.currentPage = 1;
}

const selectedImage = ref<ReadOnlyImageData | null>(null);

const isUploadModalOpen = ref(false);

async function load() {
  images.setContext();
  await images.refresh();
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
        <Button size="sm" class="ml-2" @click="isUploadModalOpen = true">
          <Plus /> 上传图片
        </Button>
      </div>
    </div>

    <Separator />

    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2 text-sm text-muted-foreground">
        <ImageFilter />
        共 <span class="font-medium text-foreground">{{ images.total }}</span> 张图片
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
        :url="images.getThumbnailUrl(img.meta.id, 'medium', img.signature)"
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

    <ImageUploadModal v-model:open="isUploadModalOpen" />

    <ImageDetailDialog
      :image="selectedImage"
      :url="selectedImage ? images.getImageUrl(selectedImage.meta.id, selectedImage.signature) : ''"
      @update:tags="tags => images.setTags(selectedImage!.meta.id, tags)"
      @close="selectedImage = null"
    />
  </div>
</template>
