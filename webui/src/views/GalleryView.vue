<script setup lang="ts">
import type { AcceptableValue } from 'reka-ui';
import { ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight, Download, FolderPlus, MoreHorizontal, Trash, Unlock } from 'lucide-vue-next';

const currentPage = ref(1);
const pageSize = ref(20);
const jumpPage = ref('1');

const totalImages = ref(112);
const totalPages = computed(() => Math.ceil(totalImages.value / pageSize.value));

const currentImageCount = computed(() => {
  if (currentPage.value < totalPages.value)
    return pageSize.value;
  else
    return totalImages.value - pageSize.value * (totalPages.value - 1);
});

const images = computed(() => {
  const imageOffset = (currentPage.value - 1) * pageSize.value;
  return Array.from({ length: currentImageCount.value }, (_, idx) => {
    const i = imageOffset + idx;
    return {
      id: i,
      url: `https://picsum.photos/seed/${i + 100}/600/400`,
      title: `Image-${i + 1}.jpg`,
    };
  });
});

function goToPage(page: number) {
  currentPage.value = page;
  jumpPage.value = String(page);
}

function handleJump() {
  const page = Number.parseInt(jumpPage.value, 10);
  if (!Number.isNaN(page) && page >= 1 && page <= totalPages.value)
    goToPage(page);
  else
    jumpPage.value = String(currentPage.value);
}

function onPageSizeChange(val: AcceptableValue) {
  pageSize.value = val as number;
  currentPage.value = 1;
  jumpPage.value = '1';
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-semibold">
          最近上传
        </h2>
        <p class="text-sm text-muted-foreground">
          共 {{ totalImages }} 张图片
        </p>
      </div>

      <div class="flex items-center gap-3">
        <span class="text-sm text-muted-foreground">每页显示</span>
        <Select :model-value="pageSize" @update:model-value="onPageSizeChange">
          <SelectTrigger class="w-20">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem :value="10">
              10
            </SelectItem>
            <SelectItem :value="20">
              20
            </SelectItem>
            <SelectItem :value="50">
              50
            </SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>

    <div
      class="
        grid grid-cols-2 gap-4
        md:grid-cols-3
        lg:grid-cols-4
        xl:grid-cols-5
      "
    >
      <div v-for="img in images" :key="img.id" class="group relative">
        <ContextMenu>
          <ContextMenuTrigger>
            <Card
              class="
                cursor-pointer overflow-hidden border-0 bg-muted/20 shadow-none
                transition-colors
                hover:bg-muted/40
              "
            >
              <CardContent class="p-0">
                <AspectRatio :ratio="4 / 3">
                  <img
                    :src="img.url"
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
                <span class="truncate text-xs font-medium">{{ img.title }}</span>

                <DropdownMenu>
                  <DropdownMenuTrigger as-child>
                    <Button
                      variant="ghost" size="icon" class="
                        h-6 w-6 opacity-0 transition-opacity
                        group-hover:opacity-100
                      "
                    >
                      <MoreHorizontal class="h-4 w-4" />
                    </Button>
                  </DropdownMenuTrigger>

                  <DropdownMenuContent align="end">
                    <DropdownMenuItem>下载</DropdownMenuItem>
                    <DropdownMenuItem>设为公开</DropdownMenuItem>
                    <DropdownMenuItem>移入相册</DropdownMenuItem>
                    <DropdownMenuItem class="text-red-600">
                      删除
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </CardFooter>
            </Card>
          </ContextMenuTrigger>

          <ContextMenuContent class="select-none">
            <ContextMenuItem><Download class="mr-2 h-4 w-4" /> 下载</ContextMenuItem>
            <ContextMenuItem><Unlock class="mr-2 h-4 w-4" /> 设为公开</ContextMenuItem>
            <ContextMenuItem><FolderPlus class="mr-2 h-4 w-4" /> 移入相册</ContextMenuItem>
            <ContextMenuItem
              class="
                text-red-600
                focus:text-red-600
              "
            >
              <Trash class="mr-2 h-4 w-4" /> 删除
            </ContextMenuItem>
          </ContextMenuContent>
        </ContextMenu>
      </div>
    </div>

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
  </div>
</template>
