<script setup lang="ts">
import { ListFilter, Settings2 } from 'lucide-vue-next';

const open = ref(false);

const imageFilter = useImageFilterStore();
const { filter, activeFilterCount } = storeToRefs(imageFilter);

const tags = refManualReset(() => filter.value.tags.join(', '));
const minWidth = refManualReset(() => filter.value.minWidth?.toString() ?? '');
const maxWidth = refManualReset(() => filter.value.maxWidth?.toString() ?? '');
const minHeight = refManualReset(() => filter.value.minHeight?.toString() ?? '');
const maxHeight = refManualReset(() => filter.value.maxHeight?.toString() ?? '');
const mimeType = refManualReset(() => filter.value.mimeType ?? 'all');
const createdBefore = refManualReset(() => filter.value.createdBefore ?? null);
const createdAfter = refManualReset(() => filter.value.createdAfter ?? null);
const updatedBefore = refManualReset(() => filter.value.updatedBefore ?? null);
const updatedAfter = refManualReset(() => filter.value.updatedAfter ?? null);
const visibility = refManualReset(() => filter.value.visibility);

const mimeTypes = [
  { label: 'All', value: 'all' },
  { label: 'JPEG', value: 'image/jpeg' },
  { label: 'PNG', value: 'image/png' },
  { label: 'GIF', value: 'image/gif' },
  { label: 'WEBP', value: 'image/webp' },
];

function syncToFiledValue() {
  [
    tags,
    minWidth,
    maxWidth,
    minHeight,
    maxHeight,
    mimeType,
    createdBefore,
    createdAfter,
    updatedBefore,
    updatedAfter,
    visibility,
  ]
    .forEach(ref => ref.reset());
}

watch(open, (val) => {
  if (val) {
    syncToFiledValue();
  }
});

function parseTags(tags: string) {
  return tags
    .split(',')
    .map(tag => tag.trim())
    .filter(tag => tag.length > 0);
}

function parseNumber(value: string) {
  const trimmed = value.trim();
  if (trimmed === '')
    return undefined;
  const num = Number(trimmed);
  return Number.isNaN(num) ? undefined : num;
}

function save() {
  filter.value = {
    tags: parseTags(tags.value),
    minWidth: parseNumber(minWidth.value),
    maxWidth: parseNumber(maxWidth.value),
    minHeight: parseNumber(minHeight.value),
    maxHeight: parseNumber(maxHeight.value),
    mimeType: mimeType.value !== 'all' ? mimeType.value : undefined,
    createdBefore: createdBefore.value ? new Date(createdBefore.value) : undefined,
    createdAfter: createdAfter.value ? new Date(createdAfter.value) : undefined,
    updatedBefore: updatedBefore.value ? new Date(updatedBefore.value) : undefined,
    updatedAfter: updatedAfter.value ? new Date(updatedAfter.value) : undefined,
    visibility: visibility.value,
  };
}

function reset() {
  imageFilter.reset();
  syncToFiledValue();
}
</script>

<template>
  <Popover v-model:open="open">
    <PopoverTrigger as-child>
      <Button
        variant="ghost"
        size="icon"
        class="relative h-8 w-8 rounded-full p-0 transition-all"
        :class="[
          activeFilterCount > 0
            ? `border-primary bg-primary/5 text-primary`
            : `text-muted-foreground`,
        ]"
      >
        <ListFilter />
        <span
          v-if="activeFilterCount > 0"
          class="
            absolute -top-0.5 -right-0.5 flex h-3 w-3 items-center justify-center rounded-full bg-primary text-[8px]
            text-primary-foreground
          "
        >
          {{ activeFilterCount }}
        </span>
      </Button>
    </PopoverTrigger>

    <PopoverContent align="start" class="flex w-80 p-0">
      <ScrollArea class="max-h-[calc(var(--reka-popover-content-available-height)-4px)] flex-1">
        <div class="space-y-6 p-4">
          <h4 class="mb-4 flex items-center gap-2 border-b pb-2 font-medium">
            <Settings2 class="h-4 w-4" /> 高级筛选
          </h4>

          <div class="mb-4 grid gap-3">
            <div class="space-y-2">
              <Label class="text-xs font-semibold">标签</Label>
              <Input v-model="tags" placeholder="输入标签，用逗号分隔..." />
            </div>

            <div class="space-y-2">
              <Label class="text-xs font-semibold">图片尺寸 (像素)</Label>
              <div class="space-y-1">
                <span class="text-[10px] text-muted-foreground">宽度范围</span>
                <div class="flex items-center gap-1">
                  <Input v-model="minWidth" placeholder="最小" class="h-8 text-xs" />
                  <span class="text-muted-foreground">-</span>
                  <Input v-model="maxWidth" placeholder="最大" class="h-8 text-xs" />
                </div>
              </div>
              <div class="space-y-1">
                <span class="text-[10px] text-muted-foreground">高度范围</span>
                <div class="flex items-center gap-1">
                  <Input
                    v-model="minHeight" placeholder="最小" class="h-8 text-xs"
                  />
                  <span class="text-muted-foreground">-</span>
                  <Input
                    v-model="maxHeight" placeholder="最大" class="h-8 text-xs"
                  />
                </div>
              </div>
            </div>

            <div class="space-y-2">
              <Label class="text-xs font-semibold">文件类型</Label>
              <Select v-model:model-value="mimeType">
                <SelectTrigger class="w-full">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent class="max-h-60 overflow-y-auto" position="item-aligned">
                  <SelectItem v-for="{ label, value } in mimeTypes" :key="value" :value="value">
                    {{ label }}
                  </SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div class="space-y-2">
              <Label class="text-xs font-semibold">创建时间</Label>
              <div class="flex items-center gap-2">
                <span class="text-sm">从</span>
                <DataTimePicker v-model="createdAfter" class="flex-1" />
              </div>
              <div class="flex items-center gap-2">
                <span class="text-sm">到</span>
                <DataTimePicker v-model="createdBefore" class="flex-1" />
              </div>
            </div>

            <div class="space-y-2">
              <Label class="text-xs font-semibold">更新时间</Label>
              <div class="flex items-center gap-2">
                <span class="text-sm">从</span>
                <DataTimePicker v-model="updatedAfter" class="flex-1" />
              </div>
              <div class="flex items-center gap-2">
                <span class="text-sm">到</span>
                <DataTimePicker v-model="updatedBefore" class="flex-1" />
              </div>
            </div>

            <div class="space-y-2">
              <Label class="text-xs font-semibold">可见性</Label>
              <Tabs v-model="visibility" class="w-full">
                <TabsList class="grid h-8 w-full grid-cols-3">
                  <TabsTrigger value="all" class="text-xs">
                    全部
                  </TabsTrigger>
                  <TabsTrigger value="public" class="text-xs">
                    公开
                  </TabsTrigger>
                  <TabsTrigger value="private" class="text-xs">
                    私有
                  </TabsTrigger>
                </TabsList>
              </Tabs>
            </div>
          </div>

          <div class="flex items-center justify-between border-t pt-2">
            <Button variant="outline" size="sm" @click="reset">
              重置
            </Button>
            <Button size="sm" @click="save">
              应用筛选
            </Button>
          </div>
        </div>
      </ScrollArea>
    </PopoverContent>
  </Popover>
</template>
