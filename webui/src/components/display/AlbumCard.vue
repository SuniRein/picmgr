<script setup lang="ts">
import type { ActionItem } from '@/components/common/ActionList.vue';
import { Calendar, Folder, Lock, MoreHorizontal, Pencil, Trash2, Unlock } from 'lucide-vue-next';

const { album } = defineProps<{ album: ReadOnlyAlbumMeta }>();

const emit = defineEmits<{
  click: [];
  edit: [];
  delete: [];
}>();

const actions: ActionItem[] = [
  { label: '编辑详情', icon: Pencil, handler: () => emit('edit') },
  { label: '删除相册', icon: Trash2, handler: () => emit('delete'), variant: 'destructive' },
];
</script>

<template>
  <ContextMenu>
    <ContextMenuTrigger>
      <Card
        class="
          group cursor-pointer overflow-hidden border-0 bg-muted/20 shadow-none transition-all
          hover:bg-muted/40
        "
        @click="emit('click')"
      >
        <CardContent class="p-0">
          <AspectRatio
            :ratio="4 / 3"
            class="flex items-center justify-center bg-linear-to-br from-primary/10 to-primary/5"
          >
            <Folder
              class="
                h-16 w-16 text-primary/80 transition-transform
                group-hover:scale-110
              "
            />
          </AspectRatio>
        </CardContent>

        <CardFooter class="flex flex-col items-start p-3 text-sm">
          <div class="flex w-full items-center justify-between">
            <span class="truncate font-semibold">{{ album.name }}</span>

            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button
                  variant="ghost" size="icon"
                  class="
                    h-6 w-6 opacity-0
                    group-hover:opacity-100
                  "
                  @click.stop
                >
                  <MoreHorizontal class="h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <ActionList type="dropdown" :actions />
              </DropdownMenuContent>
            </DropdownMenu>
          </div>

          <div class="mt-1 flex w-full items-center gap-2 text-[11px] text-muted-foreground">
            <span class="flex items-center gap-1">
              <component :is="album.is_public ? Unlock : Lock" class="h-3 w-3" />
              {{ album.is_public ? '公开' : '私有' }}
            </span>
            <span class="flex items-center gap-1">
              <Calendar class="h-3 w-3" />
              {{ new Date(album.updated_at).toLocaleDateString() }}
            </span>
          </div>
        </CardFooter>
      </Card>
    </ContextMenuTrigger>

    <ContextMenuContent>
      <ActionList type="context" :actions />
    </ContextMenuContent>
  </ContextMenu>
</template>
