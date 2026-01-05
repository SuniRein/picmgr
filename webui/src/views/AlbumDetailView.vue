<script setup lang="ts">
import type { ImageSignature } from '@/api';
import { ChevronLeft, Download, ImageMinus, ImagePlus, Pencil, Plus } from 'lucide-vue-next';
import api from '@/api';
import { downloadImage } from '@/utils/image';

const route = useRoute();
const router = useRouter();

const images = useImagesStore();
const currentAlbum = useCurrentAlbumStore();

const selectedImage = ref<ReadOnlyImageData | null>(null);
const editedImage = ref<ReadOnlyImageData | null>(null);

const editedAlbum = ref<ReadOnlyAlbumMeta | null>(null);

const isAddImagesModalOpen = ref(false);

const imageToRemove = ref<number | null>(null);
const removeDialogOpen = computed({
  get: () => imageToRemove.value !== null,
  set: (val) => {
    if (!val)
      imageToRemove.value = null;
  },
});

async function handleRemove() {
  if (imageToRemove.value) {
    const imageId = imageToRemove.value;
    imageToRemove.value = null;
    await api.removeImageFromAlbum(currentAlbum.id!, imageId);
    await images.refresh();
  }
}

function onPageSizeChange(val: number) {
  images.pageSize = val;
  images.currentPage = 1;
}

function handleDownload(id: number, signature: ImageSignature) {
  const url = images.getImageUrl(id, signature);
  downloadImage(`image-${id}`, url);
}

async function load() {
  currentAlbum.id = Number(route.params.albumId);
  await Promise.all([
    currentAlbum.fetchAlbumMeta(),
    images.init('album'),
  ]);
}
await load();
</script>

<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4">
      <Button
        variant="ghost"
        size="sm"
        class="-ml-2 w-fit text-muted-foreground"
        @click="router.push(P.ALBUMS)"
      >
        <ChevronLeft class="mr-1 h-4 w-4" /> 返回相册列表
      </Button>

      <div class="flex items-end justify-between">
        <div class="space-y-1">
          <div class="flex items-center gap-3">
            <h2 class="text-2xl font-bold tracking-tight">
              {{ currentAlbum.meta?.name ?? '加载中...' }}
            </h2>
            <Badge variant="outline" class="font-normal">
              {{ currentAlbum.meta?.is_public ? '公开' : '私有' }}
            </Badge>
          </div>
          <p class="text-sm text-muted-foreground">
            {{ currentAlbum.meta?.description || '暂无描述' }}
          </p>
        </div>

        <div class="flex items-center gap-2">
          <Button variant="outline" size="sm" @click="editedAlbum = currentAlbum.meta">
            <Pencil /> 编辑信息
          </Button>
          <Button size="sm" @click="isAddImagesModalOpen = true">
            <Plus /> 添加图片
          </Button>
        </div>
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
      v-if="images.items.length > 0"
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
          { label: '编辑图片', icon: Pencil, handler: () => editedImage = img },
          { label: '移出相册', icon: ImageMinus, variant: 'destructive', handler: () => imageToRemove = img.meta.id },
        ]"
        @open="selectedImage = img"
      />
    </div>

    <div v-else class="flex h-64 flex-col items-center justify-center rounded-lg border border-dashed">
      <ImagePlus class="h-10 w-10 text-muted-foreground/40" />
      <p class="mt-2 text-sm text-muted-foreground">
        相册内还没有图片
      </p>
      <Button variant="link" class="mt-1" @click="isAddImagesModalOpen = true">
        立即添加图片
      </Button>
    </div>

    <PaginationControls
      v-model:current-page="images.currentPage"
      :page-size="images.pageSize"
      :total-items="images.total"
    />

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

    <AlbumUpdateModal v-model:album="editedAlbum" @ok="currentAlbum.fetchAlbumMeta" />

    <AlbumAddImagesModal
      v-model:open="isAddImagesModalOpen"
      :album-id="currentAlbum.id!"
      :album-name="currentAlbum.meta!.name"
      :page-size="50"
      @finish="(result) => { images.refresh(); console.log('images added result: ', result); }"
    />

    <ConfirmDialog
      v-model:open="removeDialogOpen"
      variant="destructive"
      title="将图片移除相册？"
      description="移除图片仅取消与相册的关联，图片本身不会被删除。"
      @confirm="handleRemove"
    />
  </div>
</template>
