<script setup lang="ts">
import type { ActionItem } from '@/components/common/ActionList.vue';
import { MoreHorizontal } from 'lucide-vue-next';

defineProps<{
  url: string;
  actions: ActionItem[];
}>();

const emit = defineEmits<{ open: [] }>();
</script>

<template>
  <ContextMenu>
    <ContextMenuTrigger as-child>
      <div
        class="
          group cursor-pointer gap-2 overflow-hidden border-0 bg-muted/20 py-0.5
          shadow-none transition-colors
          hover:bg-muted/40
        "
        @click="emit('open')"
      >
        <div class="relative">
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

          <div class="absolute bottom-1 left-1 flex items-center justify-center">
            <slot name="extra-info" />
          </div>

          <div
            class="absolute right-1 bottom-1 flex items-center justify-center"
          >
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button
                  variant="ghost"
                  size="icon"
                  class="
                    h-6 w-6 bg-muted/50 opacity-0 transition-opacity
                    group-hover:opacity-100
                    hover:bg-muted/70
                  "
                  @click.stop
                >
                  <MoreHorizontal />
                </Button>
              </DropdownMenuTrigger>

              <DropdownMenuContent align="end">
                <ActionList type="dropdown" :actions />
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
      </div>
    </ContextMenuTrigger>

    <ContextMenuContent>
      <ActionList type="context" :actions />
    </ContextMenuContent>
  </ContextMenu>
</template>
