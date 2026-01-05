<script setup lang="ts">
import { Download, FlipHorizontal, FlipVertical, ImageIcon, RotateCw, Undo2 } from 'lucide-vue-next';

const props = defineProps<{ src: string }>();

const emit = defineEmits<{ close: [] }>();

const canvasRef = useTemplateRef('canvasRef');
const img = new Image();

watch(() => props.src, (val) => {
  if (val) {
    img.crossOrigin = 'anonymous';
    img.src = val;
    img.onload = () => applyFilters();
    reset();
  }
});

const open = computed({
  get: () => !!props.src,
  set: (val) => {
    if (!val)
      emit('close');
  },
});

const settings = ref({
  brightness: 100,
  contrast: 100,
  saturate: 100,
  grayscale: 0,
  rotate: 0,
  flipX: 1,
  flipY: 1,
});

function makeSliderValue(label: keyof typeof settings.value) {
  return computed({
    get: () => [settings.value[label]],
    set: (v: number[]) => {
      if (v[0] !== undefined)
        settings.value[label] = v[0];
    },
  });
}

function applyFilters() {
  const canvas = canvasRef.value;
  if (!canvas || !img.complete)
    return;
  const ctx = canvas.getContext('2d');
  if (!ctx)
    return;

  const rad = settings.value.rotate * Math.PI / 180;
  const w = img.width;
  const h = img.height;
  const cos = Math.abs(Math.cos(rad));
  const sin = Math.abs(Math.sin(rad));

  canvas.width = w * cos + h * sin;
  canvas.height = w * sin + h * cos;

  ctx.filter = `
    brightness(${settings.value.brightness}%) 
    contrast(${settings.value.contrast}%) 
    saturate(${settings.value.saturate}%) 
    grayscale(${settings.value.grayscale}%)
  `;

  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.save();

  ctx.translate(canvas.width / 2, canvas.height / 2);
  ctx.rotate(rad);
  ctx.scale(settings.value.flipX, settings.value.flipY);
  ctx.drawImage(img, -w / 2, -h / 2, w, h);

  ctx.restore();
}

watch(settings, applyFilters, { deep: true });

function downloadImage() {
  const link = document.createElement('a');
  link.download = 'edited-image.png';
  link.href = canvasRef.value!.toDataURL();
  link.click();
}

function reset() {
  settings.value = {
    brightness: 100,
    contrast: 100,
    saturate: 100,
    grayscale: 0,
    rotate: 0,
    flipX: 1,
    flipY: 1,
  };
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent
      class="
        flex h-[90vh] w-[95vw] max-w-300! flex-col gap-0 overflow-hidden border-zinc-200 bg-white p-0 text-zinc-900
        shadow-2xl
      "
    >
      <DialogHeader class="shrink-0 border-b border-zinc-100 bg-white/80 p-5 backdrop-blur-md">
        <div class="flex items-center justify-between">
          <div>
            <DialogTitle class="flex items-center gap-2 text-lg font-bold tracking-tight text-zinc-900">
              <div class="rounded-lg bg-zinc-900 p-1.5">
                <ImageIcon class="h-4 w-4 text-white" />
              </div>
              图片编辑器
            </DialogTitle>
            <DialogDescription class="text-xs text-zinc-500">
              简单直观的参数调节与画面变换
            </DialogDescription>
          </div>
        </div>
      </DialogHeader>

      <div
        v-if="open" class="
          flex flex-1 flex-col overflow-hidden
          md:flex-row
        "
      >
        <div class="relative flex flex-1 items-center justify-center overflow-hidden bg-zinc-50 p-8">
          <div
            class="absolute inset-0 z-0 opacity-[0.4]"
            style="background-image: conic-gradient(#e4e4e7 0.25turn, transparent 0.25turn 0.5turn, #e4e4e7 0.5turn 0.75turn, transparent 0.75turn); background-size: 16px 16px;"
          />

          <div class="relative z-10 flex h-full w-full items-center justify-center">
            <div class="rounded-sm bg-white shadow-[0_8px_30px_rgb(0,0,0,0.08)] ring-1 ring-zinc-200">
              <canvas ref="canvasRef" class="max-h-[70vh] max-w-full object-contain" />
            </div>
          </div>

          <div
            class="
              absolute bottom-8 left-1/2 z-50 flex -translate-x-1/2 items-center gap-2 rounded-2xl border
              border-zinc-200 bg-white/90 p-2 shadow-xl backdrop-blur-md
            "
          >
            <Button variant="ghost" size="icon" @click="reset">
              <Undo2 class="h-4 w-4" />
            </Button>
            <div class="mx-1 h-4 w-px bg-zinc-200" />
            <Button variant="ghost" size="sm" @click="downloadImage">
              <Download class="mr-2" /> 导出图片
            </Button>
          </div>
        </div>

        <aside
          class="
            w-full shrink-0 overflow-y-auto border-l border-zinc-100 bg-white p-6
            md:w-72
          "
        >
          <div class="space-y-8">
            <div class="space-y-6">
              <label class="text-[11px] font-bold tracking-widest text-zinc-400 uppercase">图像参数</label>
              <div
                v-for="label in ['brightness', 'contrast', 'saturate', 'grayscale'] as const"
                :key="label"
                class="space-y-3"
              >
                <div class="flex items-center justify-between">
                  <span class="text-xs font-medium text-zinc-600 capitalize">{{ label }}</span>
                  <span class="rounded-md bg-zinc-100 px-2 py-0.5 font-mono text-[11px] font-bold text-zinc-900">{{ settings[label] }}%</span>
                </div>
                <Slider
                  v-model="makeSliderValue(label).value"
                  :max="label === 'grayscale' ? 100 : 200"
                  :step="1"
                  class="cursor-pointer"
                />
              </div>
            </div>

            <div class="space-y-4 border-t border-zinc-100 pt-8">
              <label class="text-[11px] font-bold tracking-widest text-zinc-400 uppercase">构图工具</label>
              <div class="grid grid-cols-3 gap-2">
                <Button
                  variant="outline" size="icon" class="
                    w-full border-zinc-200
                    hover:bg-zinc-50 hover:text-zinc-900
                  " @click="settings.rotate += 90"
                >
                  <RotateCw class="text-zinc-600" />
                </Button>
                <Button
                  variant="outline" size="icon" class="
                    w-full border-zinc-200
                    hover:bg-zinc-50
                  " @click="settings.flipX *= -1"
                >
                  <FlipHorizontal class="text-zinc-600" />
                </Button>
                <Button
                  variant="outline" size="icon" class="
                    w-full border-zinc-200
                    hover:bg-zinc-50
                  " @click="settings.flipY *= -1"
                >
                  <FlipVertical class="text-zinc-600" />
                </Button>
              </div>
            </div>
          </div>
        </aside>
      </div>
    </DialogContent>
  </Dialog>
</template>
