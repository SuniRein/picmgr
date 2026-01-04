<script setup lang="ts">
import { ArchiveRestore, Trash, Trash2 } from 'lucide-vue-next';

const images = useImagesStore();

function onPageSizeChange(val: number) {
  images.pageSize = val;
  images.currentPage = 1;
}

const selectedImage = ref<ReadOnlyImageData | null>(null);

function getRemainingDays(deletedAt: Date): number {
  const now = new Date();
  const diffTime = now.getTime() - deletedAt.getTime();
  const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));
  return Math.max(0, images.TRASHED_IMAGE_SAVE_DAYS - diffDays);
}

const imageToDelete = ref<number | null>(null);
const deleteImageDialogOpen = computed({
  get: () => imageToDelete.value !== null,
  set: (value) => {
    if (!value)
      imageToDelete.value = null;
  },
});
const emptyTrashDialogOpen = ref(false);

async function load() {
  await images.init('trash');
}
await load();
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-end justify-between">
      <div class="space-y-1">
        <div class="flex items-center gap-3">
          <h2 class="text-2xl font-bold tracking-tight">
            回收站
          </h2>
        </div>
        <p class="text-sm text-muted-foreground">
          管理和浏览您已删除的图片
          <span class="text-xs">（图片将在 {{ images.TRASHED_IMAGE_SAVE_DAYS }} 天后被永久删除）</span>
        </p>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="destructive" size="sm" class="ml-2" @click="emptyTrashDialogOpen = true">
          <Trash /> 清空回收站
        </Button>
      </div>
    </div>

    <Separator />

    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
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
      v-if="images.total > 0"
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
        :title="`Image ${img.meta.id}`"
        :url="images.getThumbnailUrl(img.meta.id, 'medium', img.signature)"
        :actions="[
          { label: '恢复', icon: ArchiveRestore, handler: () => images.restoreImage(img.meta.id) },
          { label: '永久删除', icon: Trash2, variant: 'destructive', handler: () => imageToDelete = img.meta.id },
        ]"
        @open="selectedImage = img"
      >
        <template #extra-info>
          <Badge v-if="img.meta.trashed_at" class="opacity-80">
            {{ getRemainingDays(new Date(img.meta.trashed_at)) }} 天
          </Badge>
        </template>
      </ImageCard>
    </div>

    <div v-else class="flex h-64 flex-col items-center justify-center rounded-lg border border-dashed">
      <Trash class="h-10 w-10 text-muted-foreground/40" />
      <p class="mt-2 text-sm text-muted-foreground">
        回收站为空
      </p>
    </div>

    <PaginationControls
      v-model:current-page="images.currentPage"
      :page-size="images.pageSize"
      :total-items="images.total"
    />

    <ImageDetailDialog
      readonly
      :image="selectedImage"
      :url="selectedImage ? images.getImageUrl(selectedImage.meta.id, selectedImage.signature) : ''"
      @close="selectedImage = null"
    />

    <ConfirmDialog
      v-model:open="deleteImageDialogOpen"
      variant="destructive"
      title="永久删除图片？"
      description="图片将从系统中永久删除，删除后将无法恢复。"
      @confirm="() => images.deleteTrashedImage(imageToDelete!)"
    />

    <ConfirmDialog
      v-model:open="emptyTrashDialogOpen"
      variant="destructive"
      title="清空回收站？"
      description="图片将从系统中永久删除，删除后将无法恢复。"
      @confirm="() => images.emptyTrash()"
    />
  </div>
</template>
