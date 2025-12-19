<script setup lang="ts">
const pageSize = defineModel<number>('pageSize', { required: true });
</script>

<template>
  <div class="flex items-center gap-3">
    <span class="text-sm text-muted-foreground">每页显示</span>
    <Select v-model:model-value="pageSize">
      <SelectTrigger class="w-20">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem v-for="size in [10, 20, 50]" :key="size" :value="size">
          {{ size }}
        </SelectItem>
      </SelectContent>
    </Select>
  </div>
</template>
