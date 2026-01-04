<script setup lang="ts">
import { CheckSquare, X } from 'lucide-vue-next';

interface Props {
  selected?: number;
}
defineProps<Props>();

const emit = defineEmits<{
  selectAll: [];
}>();

const open = defineModel<boolean>('open');
</script>

<template>
  <Transition
    enter-active-class="transition duration-300 ease-out"
    enter-from-class="translate-y-full opacity-0"
    enter-to-class="translate-y-0 opacity-100"
    leave-active-class="transition duration-200 ease-in"
    leave-from-class="translate-y-0 opacity-100"
    leave-to-class="translate-y-full opacity-0"
  >
    <div
      v-if="open"
      class="
        fixed bottom-6 left-1/2 z-50 flex -translate-x-1/2 items-center gap-4
        rounded-full border bg-background px-6 py-3 shadow-lg ring-1 ring-border
      "
    >
      <div class="flex items-center gap-2 border-r pr-4 text-sm font-medium">
        <Badge class="h-6 w-6 rounded-full">
          {{ selected ?? 0 }}
        </Badge>
        <span>已选择</span>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="ghost" size="icon" title="全选当前页" @click="emit('selectAll')">
          <CheckSquare />
        </Button>
      </div>

      <div class="border-l pl-4">
        <Button variant="ghost" size="icon" class="h-8 w-8 rounded-full" @click="open = false">
          <X />
        </Button>
      </div>
    </div>
  </Transition>
</template>
