<script setup lang="ts">
import { AlertCircle } from 'lucide-vue-next';
import api from '@/api';

const emit = defineEmits<{ ok: [] }>();
const open = defineModel<boolean>('open', { required: true });

const form = reactive({
  name: '',
  description: '',
  is_public: false,
});

const loading = ref(false);
const error = ref('');
const formErrors = reactive({ name: '' });

async function handleSubmit() {
  if (!form.name.trim()) {
    formErrors.name = '相册名不能为空';
    return;
  }

  formErrors.name = '';
  loading.value = true;
  error.value = '';

  try {
    await api.createAlbum({
      ...form,
      name: form.name.trim(),
    });
    open.value = false;
    resetForm();
    emit('ok');
  }
  catch (e: any) {
    if (e.response?.status === 409) {
      formErrors.name = '相册名不能与已存在的相册重复';
      return;
    }
    error.value = e.response?.data?.error || '创建相册失败，请稍后重试。';
  }
  finally {
    loading.value = false;
  }
}

function resetForm() {
  form.name = '';
  form.description = '';
  form.is_public = false;
  error.value = '';
  formErrors.name = '';
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

      <Alert v-if="error" variant="destructive">
        <AlertCircle class="h-4 w-4" />
        <AlertDescription>{{ error }}</AlertDescription>
      </Alert>

      <div class="grid gap-4 py-2">
        <div class="grid gap-2">
          <Label for="name" :class="{ 'text-destructive': formErrors.name }">相册名称</Label>
          <Input
            id="name" v-model="form.name" placeholder="例如：生活随笔"
            :class="{ 'border-destructive': formErrors.name }"
            @input="formErrors.name = ''"
          />
          <p v-if="formErrors.name" class="text-sm font-medium text-destructive">
            {{ formErrors.name }}
          </p>
        </div>
        <div class="grid gap-2">
          <Label for="description">描述 (可选)</Label>
          <Textarea id="description" v-model="form.description" placeholder="简单介绍下这个相册..." />
        </div>
        <div class="flex items-center justify-between rounded-lg border p-3">
          <div class="space-y-0.5">
            <Label>允许公开访问</Label>
            <p class="text-[12px] text-muted-foreground">
              开启后，其他人可以查看此相册
            </p>
          </div>
          <Switch v-model="form.is_public" />
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" :disabled="loading" @click="open = false">
          取消
        </Button>
        <Button :disabled="loading" @click="handleSubmit">
          <span v-if="loading">创建中...</span>
          <span v-else>确认创建</span>
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
