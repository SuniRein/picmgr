<script setup lang="ts">
import { FolderPlus, Plus } from 'lucide-vue-next';
import { AlbumCard, CreateAlbumModal } from '@/components/album';

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

onMounted(async () => {
  await refresh();
});

function openAlbum(_id: number) { }
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-semibold">
          我的相册
        </h2>
        <p class="text-sm text-muted-foreground">
          共 {{ albums.total }} 个相册
        </p>
      </div>

      <div class="flex items-center gap-2">
        <RefreshButton :loading="albums.isLoading" @click="refresh" />
        <PageSizeSelector :page-size="albums.pageSize" @update:page-size="onPageSizeChange" />
        <Button size="sm" class="ml-2" @click="isCreateModalOpen = true">
          <Plus class="h-4 w-4" /> 新建相册
        </Button>
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
        @click="openAlbum"
      />
    </div>

    <div
      v-else-if="!albums.isLoading"
      class="
        flex h-100 flex-col items-center justify-center rounded-xl border
        border-dashed bg-muted/10
      "
    >
      <div
        class="
          flex h-20 w-20 items-center justify-center rounded-full bg-muted/50
        "
      >
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
