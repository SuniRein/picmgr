<script setup lang="ts">
import { ChevronLeft, Download, ImagePlus, Pencil, Plus, Trash } from 'lucide-vue-next';

const route = useRoute();
const router = useRouter();

const images = useImagesStore();
const currentAlbum = useCurrentAlbumStore();

const selectedImage = ref<ReadOnlyImageData | null>(null);

function onPageSizeChange(val: number) {
  images.pageSize = val;
  images.currentPage = 1;
}

async function load() {
  currentAlbum.id = Number(route.params.albumId);
  await Promise.all([
    currentAlbum.fetchAlbumMeta(),
    images.init(),
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
          <Button variant="outline" size="sm">
            <Pencil /> 编辑信息
          </Button>
          <Button size="sm">
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
          { label: '移出相册', icon: Trash, variant: 'destructive' },
        ]"
        @open="selectedImage = img"
      />
    </div>

    <div
      v-else class="
        flex h-64 flex-col items-center justify-center rounded-lg border
        border-dashed
      "
    >
      <ImagePlus class="h-10 w-10 text-muted-foreground/40" />
      <p class="mt-2 text-sm text-muted-foreground">
        相册内还没有图片
      </p>
      <Button variant="link" class="mt-1">
        立即添加图片
      </Button>
    </div>

    <PaginationControls
      v-model:current-page="images.currentPage"
      :page-size="images.pageSize"
      :total-items="images.total"
    />

    <ImageDetailDialog
      :image="selectedImage"
      :url="selectedImage ? images.getImageUrl(selectedImage.meta.id, selectedImage.signature) : ''"
      @update:tags="tags => images.setTags(selectedImage!.meta.id, tags)"
      @close="selectedImage = null"
    />
  </div>
</template>
