<script setup lang="ts">
import { RefreshCw } from 'lucide-vue-next';

defineProps<{ loading: boolean }>();
const emits = defineEmits<{ click: [] }>();
</script>

<template>
  <Button
    variant="ghost"
    size="icon"
    class="h-8 w-8 rounded-full p-0"
    :disabled="loading"
    @click="emits('click')"
  >
    <RefreshCw :class="{ 'animate-spin': loading }" />
  </Button>
</template>
