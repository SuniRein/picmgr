<script setup lang="ts">
import { ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight } from 'lucide-vue-next';

const props = defineProps<{
  totalImages: number;
  pageSize: number;
}>();

const currentPage = defineModel<number>('currentPage', { required: true });

const totalPages = computed(() => {
  const pages = Math.ceil(props.totalImages / props.pageSize);
  return pages > 0 ? pages : 1;
});

const jumpPage = ref(String(currentPage.value));
watch(currentPage, val => jumpPage.value = String(val));

function goToPage(page: number) {
  const target = Math.max(1, Math.min(page, totalPages.value));
  currentPage.value = target;
  jumpPage.value = String(target);
}

function handleJump() {
  const page = Number.parseInt(jumpPage.value, 10);
  if (!Number.isNaN(page))
    goToPage(page);
  else
    jumpPage.value = String(currentPage.value);
}
</script>

<template>
  <div
    class="
      flex flex-col items-center justify-between gap-4 border-t pt-4
      sm:flex-row
    "
  >
    <div
      class="
        text-center text-sm text-muted-foreground
        sm:text-left
      "
    >
      第 {{ (currentPage - 1) * pageSize + 1 }}
      到 {{ Math.min(currentPage * pageSize, totalImages) }}
      张，共 {{ totalImages }} 张
    </div>

    <div class="flex items-center space-x-2">
      <div class="flex items-center space-x-1">
        <Button variant="outline" size="icon" class="h-8 w-8" :disabled="currentPage === 1" @click="goToPage(1)">
          <ChevronsLeft class="h-4 w-4" />
        </Button>
        <Button variant="outline" size="icon" class="h-8 w-8" :disabled="currentPage === 1" @click="goToPage(currentPage - 1)">
          <ChevronLeft class="h-4 w-4" />
        </Button>

        <div class="flex items-center gap-2 px-2">
          <Input
            v-model="jumpPage"
            class="h-8 w-12 text-center"
            @keyup.enter="handleJump"
            @blur="handleJump"
          />
          <span class="text-sm text-muted-foreground">/ {{ totalPages }} 页</span>
        </div>

        <Button variant="outline" size="icon" class="h-8 w-8" :disabled="currentPage === totalPages" @click="goToPage(currentPage + 1)">
          <ChevronRight class="h-4 w-4" />
        </Button>
        <Button variant="outline" size="icon" class="h-8 w-8" :disabled="currentPage === totalPages" @click="goToPage(totalPages)">
          <ChevronsRight class="h-4 w-4" />
        </Button>
      </div>
    </div>
  </div>
</template>
