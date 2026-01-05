<script setup lang="ts">
import { AlertCircle, Loader2, Lock, Mail, User } from 'lucide-vue-next';
import api from '@/api';

const isLoading = ref(false);
const error = ref('');

const formData = reactive({
  username: '',
  email: '',
  password: '',
  confirmPassword: '',
});

const passwordsMatch = computed(() => {
  if (!formData.confirmPassword)
    return true;
  return formData.password === formData.confirmPassword;
});

async function onSubmit(event: Event) {
  event.preventDefault();
  error.value = '';

  if (!/^[\w.-]+$/.test(formData.username)) {
    error.value = '用户名只能包含字母、数字、下划线、点或连字符';
    return;
  }
  if (formData.username.length < 3 || formData.username.length > 30) {
    error.value = '用户名长度应为 3 到 30 个字符';
    return;
  }
  if (!/^[\w.%+-]+@[a-z0-9.-]+\.[a-z]{2,}$/i.test(formData.email)) {
    error.value = '无效的邮件格式';
    return;
  }
  if (!passwordsMatch.value) {
    error.value = '两次输入的密码不一致';
    return;
  }
  if (formData.password.length < 6) {
    error.value = '密码长度至少为 6 位';
    return;
  }

  isLoading.value = true;

  try {
    await api.register({
      username: formData.username,
      email: formData.email,
      password: formData.password,
    });
  }
  catch (err: any) {
    const response = err.response;
    if (response) {
      const { data, status } = response;
      if (status === 409) {
        error.value = '用户名或邮箱已被占用';
        return;
      }
      if (status === 422) {
        error.value = `服务端校验错误：${data}`;
        return;
      }
    }
    error.value = err.message ?? '注册失败，请稍后再试';
  }
  finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div class="flex min-h-screen items-center justify-center bg-background px-4">
    <Card class="w-full max-w-md">
      <CardHeader class="space-y-1">
        <CardTitle class="text-2xl">
          创建账号
        </CardTitle>
        <CardDescription>
          请输入下列信息完成注册
        </CardDescription>
      </CardHeader>

      <CardContent>
        <form @submit="onSubmit">
          <FieldSet class="grid gap-4" :disabled="isLoading">
            <Field data-invalid>
              <FieldLabel for="username">
                用户名
              </FieldLabel>
              <div class="relative">
                <User class="pointer-events-none absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                <Input
                  id="username" v-model="formData.username"
                  type="text" placeholder="zhangsan" required autocomplete="off"
                  class="pl-8"
                  :aria-invalid="!!error && !formData.username"
                />
              </div>
            </Field>

            <Field>
              <FieldLabel for="email">
                邮箱
              </FieldLabel>
              <div class="relative">
                <Mail class="pointer-events-none absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                <Input
                  id="email" v-model="formData.email"
                  type="email" placeholder="name@example.com" required autocomplete="off"
                  class="pl-8"
                  :aria-invalid="!!error && !formData.email"
                />
              </div>
            </Field>

            <Field>
              <FieldLabel for="password">
                密码
              </FieldLabel>
              <div class="relative">
                <Lock class="pointer-events-none absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                <Input
                  id="password" v-model="formData.password"
                  type="password" placeholder="••••••••" required
                  class="pl-8"
                  :aria-invalid="!!error && !formData.password"
                />
              </div>
            </Field>

            <Field>
              <Label for="confirmPassword">确认密码</Label>
              <div class="relative">
                <Lock class="pointer-events-none absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                <Input
                  id="confirmPassword" v-model="formData.confirmPassword"
                  type="password" placeholder="请再次输入密码" required
                  class="pl-8"
                  :aria-invalid="!!error && passwordsMatch"
                  :class="{ 'border-2 border-destructive': !passwordsMatch }"
                />
              </div>
            </Field>

            <FieldError v-if="error">
              <Alert variant="destructive" class="py-2">
                <AlertCircle class="h-4 w-4" />
                <AlertDescription> {{ error }} </AlertDescription>
              </Alert>
            </FieldError>

            <Field>
              <Button class="mt-2 w-full" type="submit">
                <Loader2 v-if="isLoading" class="animate-spin" />
                {{ isLoading ? "注册中..." : "立即注册" }}
              </Button>
            </Field>
          </FieldSet>
        </form>
      </CardContent>

      <CardFooter class="flex flex-col gap-4">
        <div class="w-full text-center text-sm text-muted-foreground">
          已有账号？
          <RouterLink
            :to="P.LOGIN" class="
              font-medium text-primary underline-offset-4
              hover:underline
            "
          >
            返回登录
          </RouterLink>
        </div>
      </CardFooter>
    </Card>
  </div>
</template>
