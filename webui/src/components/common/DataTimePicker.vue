<script setup lang="ts">
interface Props {
  modelValue: Date | null;
}

const { modelValue } = defineProps<Props>();
const emits = defineEmits<{ 'update:modelValue': [Date | null] }>();

const date = ref('');
const time = ref('');

watch(() => modelValue, syncFromProps, { immediate: true });

function syncFromProps() {
  if (!modelValue || Number.isNaN(modelValue.getTime())) {
    date.value = '';
    time.value = '';
    return;
  }

  const y = modelValue.getFullYear();
  const m = (modelValue.getMonth() + 1).toString().padStart(2, '0');
  const d = modelValue.getDate().toString().padStart(2, '0');

  const h = modelValue.getHours().toString().padStart(2, '0');
  const min = modelValue.getMinutes().toString().padStart(2, '0');
  const s = modelValue.getSeconds().toString().padStart(2, '0');

  date.value = `${y}-${m}-${d}`;
  time.value = `${h}:${min}:${s}`;
}

function updateValue() {
  if (!date.value) {
    emits('update:modelValue', null);
    return;
  }

  const finalTime = time.value || '00:00:00';
  const newDate = new Date(`${date.value}T${finalTime}`);

  if (!Number.isNaN(newDate.getTime())) {
    emits('update:modelValue', newDate);
  }
  else {
    emits('update:modelValue', null);
  }
}
</script>

<template>
  <div class="flex gap-2">
    <Input
      v-model="date"
      type="date"
      class="
        flex-3 appearance-none bg-background text-center
        [&::-webkit-calendar-picker-indicator]:hidden
        [&::-webkit-calendar-picker-indicator]:appearance-none
      "
      @blur="updateValue"
    />
    <Input
      v-model="time"
      type="time"
      step="1"
      class="
        flex-2 appearance-none bg-background text-center
        [&::-webkit-calendar-picker-indicator]:hidden
        [&::-webkit-calendar-picker-indicator]:appearance-none
      "
      @blur="updateValue"
    />
  </div>
</template>
