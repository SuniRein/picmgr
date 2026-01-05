<script setup lang="ts">
import { Lock, User } from 'lucide-vue-next';

const user = useUserStore();

const identifier = ref('');
const password = ref('');
const loading = ref(false);
const errorMsg = ref('');

const router = useRouter();

async function handleLogin() {
  errorMsg.value = '';
  if (!identifier.value || !password.value) {
    errorMsg.value = '请填写用户名和密码';
    return;
  }

  loading.value = true;
  try {
    await user.login(identifier.value, password.value);
    // Redirect to the main app page after successful login
    router.push(P.IMAGES);
  }
  catch {
    errorMsg.value = '登录失败，请检查您的用户名和密码';
  }
  finally {
    loading.value = false;
  }
}

function handleGuest() {
  router.push(P.IMAGES);
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
          <Label class="text-sm leading-none font-medium">用户名或邮箱</Label>
          <div class="relative">
            <User class="pointer-events-none absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              v-model="identifier"
              placeholder="username"
              class="pl-8"
              :aria-invalid="!!errorMsg && !identifier"
            />
          </div>
        </div>

        <div class="space-y-2">
          <Label class="text-sm leading-none font-medium">密码</Label>
          <div class="relative">
            <Lock class="pointer-events-none absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
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

      <CardFooter class="flex flex-col gap-4">
        <Button class="w-full" :disabled="loading" @click="handleLogin">
          <span v-if="!loading">登录</span>
          <span v-else>登录中…</span>
        </Button>
        <Button variant="secondary" class="w-full" :disabled="loading" @click="handleGuest">
          访客模式
        </Button>
        <p class="w-full text-center text-sm text-muted-foreground">
          没有账号
          <RouterLink
            :to="P.REGISTER" class="
              font-medium text-primary underline-offset-4
              hover:underline
            "
          >
            点击注册
          </RouterLink>
        </p>
      </CardFooter>
    </Card>
  </div>
</template>
