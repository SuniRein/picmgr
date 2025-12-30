<script setup lang="ts">
import type { ImageDataView } from '@/stores/images';
import { Calendar, ExternalLink, FileType, HardDrive, Info, Maximize, Shield } from 'lucide-vue-next';

interface Props {
  image: ImageDataView | null;
  url: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{ close: [] }>();

const isOpen = computed({
  get: () => props.image !== null,
  set: (val: boolean) => {
    if (!val)
      emit('close');
  },
});

const formatSize = (bytes: number) => `${(bytes / 1024).toFixed(1)} KB`;
const formatDate = (dateStr: string) => new Date(dateStr).toLocaleString();
</script>

<template>
  <Dialog v-model:open="isOpen">
    <DialogScrollContent
      class="max-w-6xl overflow-hidden border-none p-0 shadow-2xl"
    >
      <template v-if="image">
        <div
          class="
            flex h-full max-h-[90vh] flex-col
            md:flex-row
          "
        >
          <div
            class="
              relative flex min-h-75 flex-1 items-center justify-center
              overflow-hidden rounded-l-lg bg-muted/40 p-4
            "
          >
            <img
              :src="url" :alt="image.meta.id.toString()"
              class="
                max-h-full max-w-full object-contain shadow-2xl
                transition-transform
              "
            >
          </div>

          <div
            class="
              flex w-full flex-col bg-background
              md:w-95
            "
          >
            <DialogHeader class="border-b p-6">
              <DialogTitle class="flex items-center gap-2">
                <Info class="h-5 w-5 text-primary" />
                资源详情 #{{ image.meta.id }}
              </DialogTitle>
            </DialogHeader>

            <ScrollArea class="flex-1">
              <div class="space-y-8 p-6">
                <section>
                  <h4
                    class="
                      mb-4 text-[11px] font-bold tracking-widest
                      text-muted-foreground/70 uppercase
                    "
                  >
                    Properties
                  </h4>
                  <div class="space-y-4">
                    <div class="flex items-center justify-between text-sm">
                      <span
                        class="flex items-center gap-2 text-muted-foreground"
                      >
                        <Maximize class="h-4 w-4" /> 分辨率
                      </span>
                      <span class="font-mono font-medium">{{ image.meta.width }} × {{ image.meta.height }}</span>
                    </div>

                    <div class="flex items-center justify-between text-sm">
                      <span
                        class="flex items-center gap-2 text-muted-foreground"
                      >
                        <HardDrive class="h-4 w-4" /> 大小
                      </span>
                      <span class="font-medium">{{ formatSize(image.meta.size_bytes) }}</span>
                    </div>

                    <div class="flex items-center justify-between text-sm">
                      <span
                        class="flex items-center gap-2 text-muted-foreground"
                      >
                        <FileType class="h-4 w-4" /> 格式
                      </span>
                      <span class="font-medium uppercase">{{ image.meta.mime_type.split('/')[1] }}</span>
                    </div>

                    <div class="flex items-center justify-between text-sm">
                      <span
                        class="flex items-center gap-2 text-muted-foreground"
                      >
                        <Calendar class="h-4 w-4" /> 创建时间
                      </span>
                      <span class="font-medium">{{ formatDate(image.meta.created_at) }}</span>
                    </div>

                    <div class="flex items-center justify-between text-sm">
                      <span
                        class="flex items-center gap-2 text-muted-foreground"
                      >
                        <Shield class="h-4 w-4" /> 权限
                      </span>
                      <Badge
                        :variant="image.meta.is_public ? 'default' : 'outline'"
                        class="rounded-sm px-1.5 py-0"
                      >
                        {{ image.meta.is_public ? '公开' : '私有' }}
                      </Badge>
                    </div>
                  </div>
                </section>

                <section v-if="image.meta.tags.length">
                  <h4
                    class="
                      mb-3 text-[11px] font-bold tracking-widest
                      text-muted-foreground/70 uppercase
                    "
                  >
                    Tags
                  </h4>
                  <div class="flex flex-wrap gap-1.5">
                    <Badge
                      v-for="t in image.meta.tags" :key="t" variant="secondary"
                      class="
                        bg-secondary/50 font-normal
                        hover:bg-secondary
                      "
                    >
                      #{{ t }}
                    </Badge>
                  </div>
                </section>

                <section v-if="image.meta.exif && Object.keys(image.meta.exif).length">
                  <div class="mb-3 flex items-center gap-2">
                    <h4
                      class="
                        text-[11px] font-bold tracking-widest
                        text-muted-foreground/70 uppercase
                      "
                    >
                      Metadata (EXIF)
                    </h4>
                  </div>
                  <div
                    class="
                      rounded-md border bg-zinc-50 p-3 font-mono text-[11px]
                      leading-relaxed
                      dark:bg-zinc-900/50
                    "
                  >
                    <div
                      v-for="(value, key) in image.meta.exif" :key="key" class="
                        flex justify-between border-b border-zinc-100 py-1
                        last:border-0
                        dark:border-zinc-800
                      "
                    >
                      <span class="text-muted-foreground">{{ key }}</span>
                      <span class="text-foreground italic">{{ value }}</span>
                    </div>
                  </div>
                </section>
              </div>
            </ScrollArea>

            <div class="flex gap-2 border-t bg-muted/10 p-4">
              <Button variant="outline" class="flex-1 gap-2 text-xs" as-child>
                <a :href="url" target="_blank">
                  <ExternalLink class="h-3.5 w-3.5" /> 查看原图
                </a>
              </Button>
            </div>
          </div>
        </div>
      </template>
    </DialogScrollContent>
  </Dialog>
</template>
