<script setup lang="ts">
const open = defineModel<boolean>('open', { required: true });

const form = reactive({
  name: '',
  description: '',
  is_public: false,
});

async function handleSubmit() {
  // TODO: call API to create album
  open.value = false;
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-xl">
      <DialogHeader>
        <DialogTitle>新建相册</DialogTitle>
        <DialogDescription>
          创建一个相册来组织和管理你的图片。
        </DialogDescription>
      </DialogHeader>

      <div class="grid gap-4 py-4">
        <div class="grid gap-2">
          <Label for="name">相册名称</Label>
          <Input id="name" v-model="form.name" placeholder="例如：生活随笔" />
        </div>
        <div class="grid gap-2">
          <Label for="description">描述 (可选)</Label>
          <Textarea id="description" v-model="form.description" placeholder="简单介绍下这个相册..." />
        </div>
        <div class="flex items-center justify-between rounded-lg border p-3">
          <div class="space-y-0.5">
            <Label>允许公开访问</Label>
            <p class="text-[12px] text-muted-foreground">
              开启后，其他人可以通过链接查看此相册
            </p>
          </div>
          <Switch v-model:checked="form.is_public" />
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="open = false">
          取消
        </Button>
        <Button @click="handleSubmit">
          确认创建
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
