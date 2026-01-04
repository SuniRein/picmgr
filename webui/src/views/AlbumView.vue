<script setup lang="ts">
import { FolderPlus, Plus } from 'lucide-vue-next';

const router = useRouter();
const albums = useAlbumsStore();

function onPageSizeChange(val: number) {
  albums.pageSize = val;
  albums.currentPage = 1;
}

const isCreateModalOpen = ref(false);

async function refresh() {
  await Promise.all([
    albums.loadTotalCount(),
    albums.fetchAlbums(),
  ]);
}
await refresh();
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-end justify-between">
      <div class="space-y-1">
        <div class="flex items-center gap-3">
          <h2 class="text-2xl font-bold tracking-tight">
            我的相册
          </h2>
        </div>
        <p class="text-sm text-muted-foreground">
          管理和浏览您的所有相册
        </p>
      </div>

      <div class="flex items-center gap-2">
        <Button size="sm" class="ml-2" @click="isCreateModalOpen = true">
          <Plus /> 新建相册
        </Button>
      </div>
    </div>

    <Separator />

    <div class="flex items-center justify-between">
      <div class="text-sm text-muted-foreground">
        共 <span class="font-medium text-foreground">{{ albums.total }}</span> 个相册
      </div>
      <div class="flex items-center gap-2">
        <RefreshButton :loading="albums.isLoading" @click="refresh" />
        <PageSizeSelector :page-size="albums.pageSize" @update:page-size="onPageSizeChange" />
      </div>
    </div>

    <div
      v-if="albums.items.length > 0"
      class="
        grid grid-cols-2 gap-4
        md:grid-cols-3
        lg:grid-cols-4
        xl:grid-cols-5
      "
    >
      <AlbumCard
        v-for="album in albums.items"
        :key="album.id"
        :album="album"
        @click="id => router.push(P.ALBUM_DETAIL(id))"
      />
    </div>

    <div
      v-else-if="!albums.isLoading"
      class="flex h-100 flex-col items-center justify-center rounded-xl border border-dashed bg-muted/10"
    >
      <div class="flex h-20 w-20 items-center justify-center rounded-full bg-muted/50">
        <FolderPlus class="h-10 w-10 text-muted-foreground/60" />
      </div>
      <h3 class="mt-4 text-lg font-medium text-foreground/80">
        暂无相册
      </h3>
      <p class="mt-1 text-sm text-muted-foreground">
        创建一个相册来更好地归类你的图片
      </p>
      <Button class="mt-6" variant="outline" @click="isCreateModalOpen = true">
        <Plus class="h-4 w-4" /> 立即创建
      </Button>
    </div>

    <PaginationControls
      v-model:current-page="albums.currentPage"
      :page-size="albums.pageSize"
      :total-items="albums.total"
    />

    <CreateAlbumModal v-model:open="isCreateModalOpen" />
  </div>
</template>
