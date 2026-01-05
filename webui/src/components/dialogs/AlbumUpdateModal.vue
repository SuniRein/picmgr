<script setup lang="ts">
import { AlertCircle, Loader2 } from 'lucide-vue-next';
import api from '@/api';

const emit = defineEmits<{ ok: [] }>();

const album = defineModel<ReadOnlyAlbumMeta | null>('album', { required: true });

const open = computed({
  get: () => !!album.value,
  set: (val: boolean) => {
    if (!val)
      album.value = null;
  },
});

const form = reactive({
  name: '',
  description: '',
  is_public: false,
});

const loading = ref(false);
const error = ref('');
const formErrors = reactive({ name: '' });

watch(album, (val) => {
  if (val) {
    form.name = val.name;
    form.description = val.description;
    form.is_public = val.is_public;
  }
  else {
    resetForm();
  }
});

async function handleSubmit() {
  if (!album.value)
    return;

  if (!form.name.trim()) {
    formErrors.name = '相册名不能为空';
    return;
  }

  formErrors.name = '';
  loading.value = true;
  error.value = '';

  try {
    await api.updateAlbum(
      album.value.id,
      {
        ...form,
        name: form.name.trim(),
      },
    );
    open.value = false;
    resetForm();
    emit('ok');
  }
  catch (e: any) {
    if (e.response?.status === 409) {
      formErrors.name = '相册名不能与已存在的相册重复';
      return;
    }
    error.value = e.response?.data?.error || '编辑相册失败，请稍后重试。';
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
        <DialogTitle>编辑相册</DialogTitle>
        <DialogDescription>
          修改相册的基本信息，完成后点击保存。
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
          <Loader2 v-if="loading" class="animate-spin" />
          {{ loading ? '保存中...' : '确认保存' }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
