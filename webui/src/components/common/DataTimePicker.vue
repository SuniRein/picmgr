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
  const datetime = modelValue.toISOString();
  date.value = datetime.slice(0, 10);
  time.value = datetime.slice(11, 19);
}

function updateValue() {
  if (!date.value) {
    emits('update:modelValue', null);
    return;
  }

  const finalTime = time.value || '00:00:00';
  const newDate = new Date(`${date.value}T${finalTime}Z`);

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
