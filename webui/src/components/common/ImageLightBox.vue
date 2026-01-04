<script setup lang="ts">
import type { CarouselApi } from '@/components/ui/carousel';
import AutoPlay from 'embla-carousel-autoplay';
import { X } from 'lucide-vue-next';
import api from '@/api';

const { ids } = defineProps<{ ids: number[] }>();

const open = defineModel<boolean>('open');
const index = defineModel<number>('index', { required: true });

const urls = ref([] as string[]);
watch(open, async () => {
  urls.value = await Promise.all(ids.map(async (id) => {
    const { data } = await api.getImage(id);
    return api.getImageUrl(id, data.signature);
  }));
});

const carousel = ref<CarouselApi>();
watch(carousel, (c) => {
  if (!c)
    return;

  index.value = c.selectedScrollSnap();
  c.on('select', () => {
    index.value = c.selectedScrollSnap();
  });
});

useEventListener('keydown', (e) => {
  if (e.key === 'Escape')
    open.value = false;
});
</script>

<template>
  <Transition
    enter-active-class="transition duration-300 ease-out"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition duration-200 ease-in"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      v-if="open"
      class="
        fixed inset-0 z-100 flex flex-col items-center justify-center
        bg-black/95 backdrop-blur-sm
      "
    >
      <div
        class="
          absolute top-0 z-10 flex w-full items-center justify-between p-4
          text-white
        "
      >
        <span class="text-sm font-medium">
          {{ index + 1 }} / {{ ids.length }}
        </span>
        <Button
          variant="ghost" size="icon"
          class="
            text-white
            hover:bg-white/20
          "
          @click="open = false"
        >
          <X />
        </Button>
      </div>

      <Carousel
        class="max-h-[90vh] w-full max-w-[90vw]"
        :opts="{ startIndex: index, loop: true }"
        :plugins="[AutoPlay({ delay: 3000, stopOnInteraction: false })]"
        @init-api="api => carousel = api"
      >
        <CarouselContent>
          <CarouselItem v-for="url in urls" :key="url">
            <div class="flex h-[90vh] items-center justify-center p-2">
              <img
                :src="url"
                class="max-h-full max-w-full object-contain shadow-2xl"
                loading="lazy"
              >
            </div>
          </CarouselItem>
        </CarouselContent>

        <CarouselPrevious
          class="
            left-4 h-12 w-12 border-none bg-white/10 text-white
            hover:bg-white/20
          "
        />
        <CarouselNext
          class="
            right-4 h-12 w-12 border-none bg-white/10 text-white
            hover:bg-white/20
          "
        />
      </Carousel>
    </div>
  </Transition>
</template>
