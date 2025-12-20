<script setup lang="ts">
import { Download, FolderPlus, MoreHorizontal, Trash, Unlock } from 'lucide-vue-next';

interface ImageItem {
  url: string;
  title: string;
}

defineProps<ImageItem>();

const emit = defineEmits(['download', 'public', 'move', 'delete']);
</script>

<template>
  <ContextMenu>
    <ContextMenuTrigger>
      <Card
        class="
          group cursor-pointer overflow-hidden border-0 bg-muted/20 shadow-none
          transition-colors
          hover:bg-muted/40
        "
      >
        <CardContent class="p-0">
          <AspectRatio :ratio="4 / 3">
            <img
              :src="url"
              alt="Photo"
              class="
                h-full w-full object-cover transition-all
                hover:scale-105
              "
              loading="lazy"
            >
          </AspectRatio>
        </CardContent>

        <CardFooter class="flex items-center justify-between p-2 text-sm">
          <span class="truncate text-xs font-medium">{{ title }}</span>

          <DropdownMenu>
            <DropdownMenuTrigger as-child>
              <Button
                variant="ghost"
                size="icon"
                class="
                  h-6 w-6 opacity-0 transition-opacity
                  group-hover:opacity-100
                "
              >
                <MoreHorizontal class="h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>

            <DropdownMenuContent align="end">
              <DropdownMenuItem @click="emit('download')">
                下载
              </DropdownMenuItem>
              <DropdownMenuItem @click="emit('public')">
                设为公开
              </DropdownMenuItem>
              <DropdownMenuItem @click="emit('move')">
                移入相册
              </DropdownMenuItem>
              <DropdownMenuItem class="text-red-600" @click="emit('delete')">
                删除
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </CardFooter>
      </Card>
    </ContextMenuTrigger>

    <ContextMenuContent class="select-none">
      <ContextMenuItem @click="emit('download')">
        <Download class="mr-2 h-4 w-4" /> 下载
      </ContextMenuItem>
      <ContextMenuItem @click="emit('public')">
        <Unlock class="mr-2 h-4 w-4" /> 设为公开
      </ContextMenuItem>
      <ContextMenuItem @click="emit('move')">
        <FolderPlus class="mr-2 h-4 w-4" /> 移入相册
      </ContextMenuItem>
      <ContextMenuItem
        class="
          text-red-600
          focus:text-red-600
        "
        @click="emit('delete')"
      >
        <Trash class="mr-2 h-4 w-4" /> 删除
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>
</template>
