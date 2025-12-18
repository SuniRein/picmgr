<script setup lang="ts">
import { Download, FolderPlus, MoreHorizontal, Trash, Unlock } from 'lucide-vue-next';

// 模拟图片数据
const images = Array.from({ length: 12 }).map((_, i) => ({
  id: i,
  url: `https://picsum.photos/seed/${i + 100}/600/400`, // 使用 Picsum 占位图
  title: `Image-${i + 1}.jpg`,
  size: '2.4 MB',
}));
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h2 class="text-xl font-semibold">
        最近上传
      </h2>

      <p class="text-sm text-muted-foreground">
        {{ images.length }} 张图片
      </p>
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
  </div>
</template>
