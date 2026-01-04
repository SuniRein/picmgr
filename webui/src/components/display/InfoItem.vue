<script setup lang="ts">
defineProps<{
  label: string;
  value?: string | number | null;
}>();
</script>

<template>
  <div class="flex items-start gap-3">
    <div v-if="$slots.icon" class="mt-0.5 shrink-0 text-muted-foreground">
      <slot name="icon" />
    </div>

    <div class="flex flex-col gap-1.5">
      <span class="text-sm font-bold tracking-widest text-muted-foreground/60 uppercase">
        {{ label }}
      </span>
      <span class="text-base leading-none font-semibold text-foreground/90">
        {{ value || '-' }}
      </span>
    </div>
  </div>
</template>
