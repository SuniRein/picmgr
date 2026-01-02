<script setup lang="ts">
import { ContextMenuItem } from '@/components/ui/context-menu';
import { DropdownMenuItem } from '@/components/ui/dropdown-menu';

export interface ActionItem {
  label: string;
  icon: Component;
  handler?: () => void;
  variant?: 'destructive';
}

defineProps<{
  type: 'dropdown' | 'context';
  actions: ActionItem[];
}>();

const typeMap = {
  dropdown: DropdownMenuItem,
  context: ContextMenuItem,
};

const classMap = {
  destructive: `
    text-destructive
    hover:bg-destructive/10
  `,
};
</script>

<template>
  <component
    :is="typeMap[type]"
    v-for="item in actions"
    :key="item.label"
    :class="item.variant ? classMap[item.variant] : ''"
    @click="item.handler"
  >
    <component :is="item.icon" /> {{ item.label }}
  </component>
</template>
