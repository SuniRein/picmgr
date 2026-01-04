<script setup lang="ts">
import { AlertCircle, Calendar, Mail, RotateCcw, User } from 'lucide-vue-next';
import { InfoItem } from '@/components/profile';

const user = useUserStore();

const isLoading = ref(false);
const fetchError = ref<string | null>(null);

async function loadProfile() {
  if (!user.isLoggedIn) {
    fetchError.value = '用户未登录';
    return;
  }

  isLoading.value = true;
  fetchError.value = null;
  try {
    await user.fetchProfile();
  }
  catch (err: any) {
    fetchError.value = err.message ?? '获取个人资料失败';
  }
  finally {
    isLoading.value = false;
  }
}

onMounted(async () => {
  if (!user.profile)
    await loadProfile();
});

const createdAt = computed(() => {
  const v = user.profile?.createdAt;
  if (!v)
    return '-';
  try {
    return new Date(v).toLocaleString();
  }
  catch {
    return v;
  }
});
</script>

<template>
  <div class="mx-auto max-w-2xl space-y-6">
    <Card v-if="isLoading" class="overflow-hidden">
      <CardHeader class="space-y-2">
        <div class="h-6 w-24 animate-pulse rounded bg-muted" />
        <div class="h-4 w-40 animate-pulse rounded bg-muted" />
      </CardHeader>
      <CardContent class="space-y-8">
        <div class="flex items-center gap-4">
          <div class="h-16 w-16 animate-pulse rounded-full bg-muted" />
          <div class="space-y-2">
            <div class="h-5 w-32 animate-pulse rounded bg-muted" />
            <div class="h-4 w-20 animate-pulse rounded bg-muted" />
          </div>
        </div>
        <div class="grid grid-cols-2 gap-8">
          <div v-for="i in 3" :key="i" class="space-y-2">
            <div class="h-3 w-12 animate-pulse rounded bg-muted" />
            <div class="h-5 w-28 animate-pulse rounded bg-muted" />
          </div>
        </div>
      </CardContent>
    </Card>

    <Alert
      v-else-if="fetchError || !user.profile"
      variant="destructive"
      class="flex flex-col items-center border-2 px-6 py-12 text-center"
    >
      <div class="mb-6">
        <AlertCircle class="h-12 w-12 stroke-[1.5px] opacity-90" />
      </div>

      <div class="space-y-2">
        <AlertTitle class="text-3xl font-bold tracking-tight">
          获取个人信息失败
        </AlertTitle>
        <AlertDescription
          class="justify-center text-lg opacity-80"
        >
          {{ fetchError || '服务器响应超时，请检查网络连接或重新登录' }}
        </AlertDescription>
      </div>

      <div class="mt-8">
        <Button
          variant="outline"
          class="
            border-destructive text-destructive
            hover:bg-destructive/10
          "
          @click="loadProfile"
        >
          <RotateCcw class="h-4 w-4" /> 重新尝试加载
        </Button>
      </div>
    </Alert>

    <Card v-else class="border-muted/60 shadow-md">
      <CardHeader>
        <CardTitle>个人信息</CardTitle>
        <CardDescription>查看您的账户资料</CardDescription>
      </CardHeader>

      <CardContent class="space-y-6">
        <div class="flex items-center gap-5">
          <div class="h-20 w-20 shrink-0 overflow-hidden rounded-full border-2 border-muted bg-muted shadow-sm">
            <img
              v-if="user.profile.avatarUrl"
              :src="user.profile.avatarUrl"
              class="h-full w-full object-cover"
            >
            <div v-else class="flex h-full w-full items-center justify-center">
              <User class="h-8 w-8 text-muted-foreground/40" />
            </div>
          </div>

          <div class="flex flex-col gap-1">
            <h3 class="text-xl font-bold tracking-tight text-foreground">
              {{ user.profile.username }}
            </h3>
            <p class="font-mono text-base text-muted-foreground/70">
              UID: {{ user.profile.id }}
            </p>
          </div>
        </div>

        <Separator class="opacity-50" />

        <div
          class="
            grid grid-cols-1 gap-x-12 gap-y-8
            md:grid-cols-2
          "
        >
          <InfoItem label="用户名" :value="user.profile.username">
            <template #icon>
              <User class="h-4 w-4 text-muted-foreground/60" />
            </template>
          </InfoItem>

          <InfoItem label="电子邮箱" :value="user.profile.email">
            <template #icon>
              <Mail class="h-4 w-4 text-muted-foreground/60" />
            </template>
          </InfoItem>

          <InfoItem label="注册日期" :value="createdAt">
            <template #icon>
              <Calendar class="h-4 w-4 text-muted-foreground/60" />
            </template>
          </InfoItem>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
