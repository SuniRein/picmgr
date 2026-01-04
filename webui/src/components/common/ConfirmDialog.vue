<script setup lang="ts">
interface Props {
  variant?: 'default' | 'destructive';
  title: string;
  description: string;
  confirmText?: string;
  cancelText?: string;
}
defineProps<Props>();

const emit = defineEmits<{ confirm: [] }>();

const open = defineModel<boolean>('open');

function handleConfirm() {
  emit('confirm');
  open.value = false;
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-md">
      <DialogHeader>
        <DialogTitle> {{ title }} </DialogTitle>
        <DialogDescription class="mt-2">
          {{ description }}
        </DialogDescription>
      </DialogHeader>

      <DialogFooter
        class="
          flex gap-2
          sm:justify-end
        "
      >
        <DialogClose as-child>
          <Button variant="outline" size="sm">
            {{ cancelText ?? "取消" }}
          </Button>
        </DialogClose>
        <Button :variant size="sm" @click="handleConfirm">
          {{ confirmText ?? "确认" }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
