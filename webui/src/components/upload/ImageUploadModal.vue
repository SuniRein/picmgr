<script setup lang="ts">
import { AlertCircle, CheckCircle2, Plus, Trash2, X } from 'lucide-vue-next';

const open = defineModel<boolean>('open', { required: true });
const upload = useUploadStore();
const isDragging = ref(false);

const { tasks, idleTasksCount } = storeToRefs(upload);

function handleFileChange(e: Event) {
  const selectedFiles = (e.target as HTMLInputElement).files;
  if (selectedFiles)
    addFiles(Array.from(selectedFiles));
}

function handleDrop(e: DragEvent) {
  isDragging.value = false;
  const droppedFiles = e.dataTransfer?.files;
  if (droppedFiles)
    addFiles(Array.from(droppedFiles));
}

function addFiles(newFiles: File[]) {
  newFiles.forEach(file => upload.addTask(file));
}

async function startUpload() {
  await upload.processQueue();
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent
      class="
        overflow-hidden border-none p-0 shadow-2xl
        sm:max-w-xl
      "
    >
      <div>
        <div class="p-6">
          <DialogHeader class="mb-4 flex flex-row items-center justify-between">
            <div>
              <DialogTitle class="text-xl font-semibold">
                上传队列
              </DialogTitle>
              <DialogDescription>支持多选，单张最大 20MB</DialogDescription>
            </div>
            <Button
              v-if="tasks.some(t => t.status === 'success')"
              variant="outline"
              size="sm"
              class="h-8 text-xs"
              @click="upload.clearCompleted"
            >
              <Trash2 class="mr-1 h-3 w-3" /> 清空已完成
            </Button>
          </DialogHeader>

          <div
            class="
              group relative overflow-hidden rounded-xl border-2 border-dashed border-muted-foreground/20 bg-muted/30
              transition-all
              hover:border-primary/50
            "
            :class="{ 'border-primary bg-primary/5': isDragging }"
            @dragover.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @drop.prevent="handleDrop"
          >
            <Label
              for="picture-upload"
              class="flex cursor-pointer flex-col items-center gap-2 p-8"
            >
              <Plus
                class="
                  h-6 w-6 text-primary transition-transform
                  group-hover:scale-110
                "
              />
              <p class="text-sm text-muted-foreground">点击或拖拽图片到此处</p>
            </Label>
            <Input
              id="picture-upload"
              type="file"
              multiple
              accept="image/*"
              class="hidden"
              @change="handleFileChange"
            />
          </div>

          <ScrollArea v-if="tasks.length > 0" class="mt-6 max-h-80">
            <div class="grid grid-cols-4 gap-3 pr-4">
              <div
                v-for="(item, index) in tasks" :key="index"
                class="group relative aspect-square overflow-hidden rounded-lg border bg-muted"
              >
                <img
                  :src="item.preview"
                  class="h-full w-full object-cover"
                  :class="{ 'opacity-40': item.status === 'uploading' }"
                >

                <div
                  v-if="item.status === 'uploading' || item.status === 'success' || item.status === 'error'"
                  class="absolute inset-0 flex items-center justify-center bg-black/20 backdrop-blur-[1px]"
                >
                  <div
                    v-if="item.status === 'uploading'"
                    class="relative h-12 w-12"
                  >
                    <svg class="h-full w-full -rotate-90">
                      <circle
                        cx="24" cy="24" r="20" stroke="currentColor" stroke-width="4" fill="transparent"
                        class="text-white/30"
                      />
                      <circle
                        cx="24" cy="24" r="20" stroke="currentColor" stroke-width="4" fill="transparent"
                        class="text-primary transition-all duration-300"
                        stroke-dasharray="125.6"
                        :stroke-dashoffset="125.6 - (125.6 * item.progress) / 100"
                      />
                    </svg>
                    <span class="absolute inset-0 flex items-center justify-center text-[10px] font-bold text-white">
                      {{ item.progress }}%
                    </span>
                  </div>
                  <CheckCircle2
                    v-else-if="item.status === 'success'"
                    class="h-8 w-8 rounded-full bg-white text-green-500"
                  />
                  <AlertCircle
                    v-else
                    class="h-8 w-8 rounded-full bg-white text-destructive"
                  />
                </div>

                <Button
                  v-if="item.status !== 'uploading'"
                  variant="destructive"
                  size="icon"
                  class="
                    absolute top-1 right-1 rounded-full bg-background/80 p-1 opacity-0
                    group-hover:opacity-100
                  "
                  @click="upload.removeTask(index)"
                >
                  <X />
                </Button>
              </div>
            </div>
          </ScrollArea>
        </div>

        <DialogFooter
          class="flex items-center justify-end gap-3 bg-muted/50 px-6 py-4"
        >
          <Button variant="ghost" @click="open = false">
            隐藏面板
          </Button>
          <Button
            :disabled="idleTasksCount === 0"
            class="px-8"
            @click="startUpload"
          >
            开始上传 ({{ idleTasksCount }})
          </Button>
        </DialogFooter>
      </div>
    </DialogContent>
  </Dialog>
</template>
