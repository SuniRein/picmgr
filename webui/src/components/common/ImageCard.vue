<script setup lang="ts">
import type { ActionItem } from '@/components/common/ActionList.vue';
import { MoreHorizontal } from 'lucide-vue-next';

defineProps<{
  url: string;
  title: string;
  actions: ActionItem[];
}>();

const emit = defineEmits<{ open: [] }>();
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
        <CardContent class="p-0" @click="emit('open')">
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
              <ActionList type="dropdown" :actions />
            </DropdownMenuContent>
          </DropdownMenu>
        </CardFooter>
      </Card>
    </ContextMenuTrigger>

    <ContextMenuContent>
      <ActionList type="context" :actions />
    </ContextMenuContent>
  </ContextMenu>
</template>
