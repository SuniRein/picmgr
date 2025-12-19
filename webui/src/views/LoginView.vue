<script setup lang="ts">
import { Lock, User } from 'lucide-vue-next';

const user = useUserStore();

const name = ref('');
const password = ref('');
const loading = ref(false);
const errorMsg = ref('');

const router = useRouter();

async function handleLogin() {
  errorMsg.value = '';
  if (!name.value || !password.value) {
    errorMsg.value = '请填写用户名和密码';
    return;
  }

  loading.value = true;
  try {
    await user.login(name.value, password.value);
    // Redirect to the main app page after successful login
    router.push('/images');
  }
  catch {
    errorMsg.value = '登录失败，请检查您的用户名和密码';
  }
  finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="flex min-h-screen items-center justify-center bg-background p-6">
    <Card class="w-full max-w-md border-border/70">
      <CardHeader class="space-y-1">
        <CardTitle class="text-2xl">
          登录到 PicManager
        </CardTitle>
        <CardDescription>使用您的账户访问图库与相册</CardDescription>
      </CardHeader>

      <CardContent class="space-y-4">
        <div class="space-y-2">
          <Label class="text-sm leading-none font-medium">用户名</Label>
          <div class="relative">
            <User
              class="
                pointer-events-none absolute top-1/2 left-2 h-4 w-4
                -translate-y-1/2 text-muted-foreground
              "
            />
            <Input
              v-model="name"
              placeholder="username"
              class="pl-8"
              :aria-invalid="!!errorMsg && !name"
            />
          </div>
        </div>

        <div class="space-y-2">
          <Label class="text-sm leading-none font-medium">密码</Label>
          <div class="relative">
            <Lock
              class="
                pointer-events-none absolute top-1/2 left-2 h-4 w-4
                -translate-y-1/2 text-muted-foreground
              "
            />
            <Input
              v-model="password"
              type="password"
              placeholder="••••••••"
              class="pl-8"
              :aria-invalid="!!errorMsg && !password"
            />
          </div>
        </div>

        <p v-if="errorMsg" class="text-sm text-destructive">
          {{ errorMsg }}
        </p>
      </CardContent>

      <CardFooter class="flex flex-col gap-3">
        <Button class="w-full" :disabled="loading" @click="handleLogin">
          <span v-if="!loading">登录</span>
          <span v-else>登录中…</span>
        </Button>
        <p class="text-center text-xs text-muted-foreground">
          没有账号？<a
            class="
              underline-offset-4
              hover:underline
            "
          >点击注册</a>
        </p>
      </CardFooter>
    </Card>
  </div>
</template>
