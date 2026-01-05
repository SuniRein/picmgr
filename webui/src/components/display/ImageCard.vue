<script setup lang="ts">
import type { ActionItem } from '@/components/common/ActionList.vue';
import { MoreHorizontal } from 'lucide-vue-next';

defineProps<{
  url: string;
  actions?: ActionItem[];
  selected?: boolean;
  selectionMode?: boolean;
}>();

const emit = defineEmits<{
  open: [];
  toggleSelect: [selected: boolean];
}>();
</script>

<template>
  <ContextMenu>
    <ContextMenuTrigger as-child :disabled="!actions">
      <div
        class="
          group relative cursor-pointer gap-2 overflow-hidden border-0 bg-muted/20 py-0.5 shadow-none transition-colors
          hover:bg-muted/40
        "
        @click="emit('open')"
      >
        <div
          v-if="selectionMode"
          class="absolute top-2 right-2 z-20 flex items-center transition-opacity"
          @click.stop
        >
          <Checkbox
            :model-value="selected"
            class="h-5 w-5 border bg-black/40"
            @update:model-value="(val) => emit('toggleSelect', val as boolean)"
          />
        </div>

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
          v-if="actions"
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
    </ContextMenuTrigger>

    <ContextMenuContent>
      <ActionList v-if="actions" type="context" :actions />
    </ContextMenuContent>
  </ContextMenu>
</template>
