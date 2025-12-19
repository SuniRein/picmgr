<script setup lang="ts">
import { HardDrive, Heart, Image, LogOut, Plus, Search, Settings, Trash2 } from 'lucide-vue-next';

const navItems = [
  { name: '全部图片', icon: Image, path: '/images' },
  { name: '我的相册', icon: HardDrive, path: '/albums' },
  { name: '收藏夹', icon: Heart, path: '/favorites' },
  { name: '回收站', icon: Trash2, path: '/trash' },
];

const router = useRouter();
const user = useUserStore();

function handleLogout() {
  user.logout();
  router.push('/login');
}
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
          <RouterLink v-for="item in navItems" :key="item.name" :to="item.path" custom>
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
        <Button variant="outline" class="w-full justify-start">
          <Settings class="mr-2 h-4 w-4" />
          设置
        </Button>
      </div>
    </aside>

    <main class="flex flex-1 flex-col overflow-hidden">
      <header
        class="
          z-10 flex h-16 items-center justify-between border-b bg-background/95
          px-6 backdrop-blur
        "
      >
        <div class="flex w-full max-w-md items-center gap-2">
          <div class="relative w-full">
            <Search
              class="
                pointer-events-none absolute top-2.5 left-2.5 h-4 w-4
                text-muted-foreground
              "
            />
            <Input type="search" placeholder="搜索图片..." class="bg-muted/50 pl-8" />
          </div>
        </div>

        <div class="flex items-center gap-4">
          <template v-if="user.isLoggedIn">
            <Button>
              <Plus class="mr-2 h-4 w-4" /> 上传图片
            </Button>

            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <div class="h-8 w-8 rounded-full bg-slate-200" />
              </DropdownMenuTrigger>

              <DropdownMenuContent align="end">
                <DropdownMenuItem @click="handleLogout">
                  <LogOut class="mr-2 h-4 w-4" /> 注销
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </template>

          <template v-else>
            <RouterLink to="/login">
              <Button variant="outline">
                登录
              </Button>
            </RouterLink>
          </template>
        </div>
      </header>

      <ScrollArea class="min-h-0 flex-1 p-6">
        <RouterView />
      </ScrollArea>
    </main>
  </div>
</template>
