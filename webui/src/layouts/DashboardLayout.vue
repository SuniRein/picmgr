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
  <div
    class="
      flex h-screen flex-col bg-background text-foreground
      md:flex-row
    "
  >
    <header
      class="
        flex h-14 items-center justify-between border-b px-4
        md:hidden
      "
    >
      <h1 class="text-lg font-bold">
        PicManager
      </h1>

      <Sheet v-if="user.isLoggedIn">
        <SheetTrigger as-child>
          <Button variant="ghost" size="icon" class="rounded-full">
            <div class="h-8 w-8 rounded-full bg-slate-200" />
          </Button>
        </SheetTrigger>
        <SheetContent side="right" class="w-72">
          <div class="mt-8 space-y-4">
            <div class="flex items-center space-x-3 p-2">
              <div class="h-10 w-10 rounded-full bg-slate-200" />
              <div>
                <p class="font-medium">
                  {{ user.profile?.username }}
                </p>
                <p class="text-xs text-muted-foreground">
                  个人账户
                </p>
              </div>
            </div>
            <Separator />
            <Button variant="ghost" class="w-full justify-start" @click="router.push(P.PROFILE)">
              <User class="mr-2" /> 个人信息
            </Button>
            <Button
              variant="ghost" class="w-full justify-start text-destructive" @click="handleLogout"
            >
              <LogOut class="mr-2" /> 注销
            </Button>
          </div>
        </SheetContent>
      </Sheet>

      <Button v-else variant="outline" size="sm" @click="router.push(P.LOGIN)">
        登录
      </Button>
    </header>

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
                <User class="mr-2" /> 个人信息
              </DropdownMenuItem>
            </RouterLink>

            <DropdownMenuItem variant="destructive" @click="handleLogout">
              <LogOut class="mr-2" /> 注销
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>

        <Button v-else variant="outline" class="w-full" @click="router.push(P.LOGIN)">
          登录
        </Button>
      </div>
    </aside>

    <main class="flex-1 overflow-hidden">
      <ScrollArea
        class="
          h-full p-4
          md:p-6
        "
      >
        <RouterView />
        <div
          class="
            h-16
            md:hidden
          "
        />
      </Scrollarea>
    </main>

    <nav
      class="
        fixed right-0 bottom-0 left-0 z-50 flex h-16 items-center justify-around
        border-t bg-background/80 backdrop-blur-md
        md:hidden
      "
    >
      <RouterLink
        v-for="item in navItems"
        :key="item.name"
        v-slot="{ isActive }"
        :to="item.to"
        class="flex flex-col items-center justify-center space-y-1 px-3"
      >
        <component
          :is="item.icon"
          class="h-5 w-5"
          :class="[isActive ? 'text-primary' : `text-muted-foreground`]"
        />
        <span
          class="text-[10px]"
          :class="[isActive
            ? 'font-medium text-primary' : `text-muted-foreground`]"
        >
          {{ item.name }}
        </span>
      </RouterLink>
    </nav>
  </div>
</template>
