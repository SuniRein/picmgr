<script setup lang="ts">
import { HardDrive, Heart, Image, LogOut, Trash2, User } from 'lucide-vue-next';

const navItems = [
  { name: '全部图片', icon: Image, to: P.IMAGES },
  { name: '我的相册', icon: HardDrive, to: P.ALBUMS },
  { name: '收藏夹', icon: Heart, to: '' },
  { name: '回收站', icon: Trash2, to: P.TRASH_IMAGES },
];

const router = useRouter();
const user = useUserStore();

function handleLogout() {
  user.logout();
  router.push(P.LOGIN);
}

async function load() {
  if (user.isLoggedIn) {
    await user.fetchProfile();
  }
}
await load();
</script>

<template>
  <div class="flex h-screen bg-background text-foreground">
    <aside
      class="
        hidden w-64 flex-col border-r
        md:flex
      "
    >
      <div class="p-6">
        <h1 class="text-2xl font-bold tracking-tight">
          PicManager
        </h1>
      </div>

      <div class="flex-1 px-4">
        <nav class="space-y-2">
          <RouterLink v-for="item in navItems" :key="item.name" :to="item.to" custom>
            <template #default="{ navigate, isActive }">
              <Button
                :variant="isActive ? 'secondary' : 'ghost'"
                class="w-full justify-start"
                @click="navigate"
              >
                <component :is="item.icon" class="mr-2 h-4 w-4" />
                {{ item.name }}
              </Button>
            </template>
          </RouterLink>
        </nav>
      </div>

      <div class="border-t p-4">
        <DropdownMenu v-if="user.isLoggedIn">
          <DropdownMenuTrigger as-child>
            <Button variant="ghost" class="w-full justify-start px-2 py-6">
              <div class="mr-3 h-8 w-8 rounded-full bg-slate-200" />
              <div class="flex flex-col items-start overflow-hidden">
                <span class="text-sm font-medium">{{ user.profile?.username }}</span>
                <span class="w-full truncate text-xs text-muted-foreground">查看账号详情</span>
              </div>
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent align="end" class="w-56">
            <RouterLink :to="P.PROFILE">
              <DropdownMenuItem>
                <User class="mr-2 h-4 w-4" /> 个人信息
              </DropdownMenuItem>
            </RouterLink>

            <DropdownMenuItem @click="handleLogout">
              <LogOut class="mr-2 h-4 w-4" /> 注销
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>

        <Button v-else variant="outline" class="w-full" @click="router.push(P.LOGIN)">
          登录
        </Button>
      </div>
    </aside>

    <main class="flex flex-1 flex-col overflow-hidden">
      <ScrollArea class="min-h-0 flex-1 p-6">
        <RouterView />
      </ScrollArea>
    </main>
  </div>
</template>
