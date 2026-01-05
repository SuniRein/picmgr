<script setup lang="ts">
import api from '@/api';

const props = defineProps<{
  albumId: number;
  albumName: string;
  pageSize: number;
}>();

const emit = defineEmits<{ finish: [result: TaskResult] }>();

interface TaskResult {
  success: number;
  failed: number;
  alreadyAdded: number;
}

interface ImageItem {
  id: number;
  thumbnailUrl: string;
}

const { currentPage } = usePagination({ initialPageSize: props.pageSize, onPageChange: fetchPage });

const images = ref<ImageItem[]>([]);
const totalItems = ref(0);

const imageIds = useArrayMap(images, item => item.id);
const multiSelected = reactive(useMultiSelect(imageIds));

async function fetchPage(page: number, size: number) {
  try {
    const { data } = await api.getImages({ page, size });
    images.value = data.data.map(image => ({
      id: image.meta.id,
      thumbnailUrl: api.getThumbnailUrl(image.meta.id, 'small', image.signature),
    }));
  }
  catch (e) {
    console.error(e);
  }
}

const open = defineModel<boolean>('open');

whenever(open, async () => {
  multiSelected.start();
  currentPage.value = 1;
  await Promise.all([
    (async () => totalItems.value = (await api.getImageCount()).data.count)(),
    fetchPage(1, props.pageSize),
  ]);
});

const submitting = ref(false);
const progress = reactive({ done: 0, total: 0 });

async function addToAlbum() {
  const ids = Array.from(multiSelected.items);
  if (ids.length === 0)
    return;

  submitting.value = true;
  progress.done = 0;
  progress.total = ids.length;

  const chunkSize = 10;
  try {
    open.value = false;

    const result = {
      success: 0,
      failed: 0,
      alreadyAdded: 0,
    };

    for (let i = 0; i < ids.length; i += chunkSize) {
      const chunk = ids.slice(i, i + chunkSize);
      await Promise.all(chunk.map(
        id => (async () => {
          try {
            await api.addImageToAlbum(props.albumId, id);
            result.success++;
          }
          catch (e: any) {
            if (e.respsonse?.status === 409)
              result.alreadyAdded++;
            else
              result.failed++;
          }
        })(),
      ));
      progress.done += chunk.length;
    }
    emit('finish', result);
  }
  catch (e) {
    console.error(e);
  }
  finally {
    submitting.value = false;
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="w-full max-w-6xl!">
      <DialogHeader>
        <DialogTitle>添加图片到「{{ albumName }}」</DialogTitle>
        <DialogDescription>从图库中选择要添加的图片</DialogDescription>
      </DialogHeader>

      <div class="flex justify-between">
        <div class="flex items-center gap-3">
          <div class="text-sm text-muted-foreground">
            已选择 <span class="font-medium text-primary">{{ multiSelected.items.size }}</span> 张
          </div>
          <Button variant="outline" size="sm" @click="multiSelected.clear">
            清空选择
          </Button>
        </div>

        <div class="flex items-center gap-3">
          <Button variant="outline" size="sm" @click="open = false">
            取消
          </Button>
          <Button :disabled="submitting" @click="addToAlbum">
            <span v-if="submitting">处理中 {{ progress.done }} / {{ progress.total }}</span>
            <span v-else>添加到相册</span>
          </Button>
        </div>
      </div>

      <ScrollArea class="max-h-[60vh]">
        <div
          class="
            grid grid-cols-3 gap-1
            sm:grid-cols-4
            md:grid-cols-5
            lg:grid-cols-6
          "
        >
          <ImageCard
            v-for="img in images"
            :key="img.id"
            :url="img.thumbnailUrl"
            selection-mode
            :selected="multiSelected.items.has(img.id)"
            @toggle-select="selected => multiSelected.toggleSelect(img.id, selected)"
          />
        </div>
      </ScrollArea>

      <PaginationControls
        v-model:current-page="currentPage"
        :total-items="totalItems"
        :page-size="pageSize"
      />
    </DialogContent>
  </Dialog>
</template>
