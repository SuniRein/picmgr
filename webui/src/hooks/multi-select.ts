export function useMultiSelect(currentIds: MaybeRef<number[]>) {
  const enabled = ref(false);
  const items = refManualReset(() => new Set<number>());

  function start() {
    enabled.value = true;
    items.reset();
  }

  function toggleSelect(id: number, value: boolean) {
    if (value)
      items.value.add(id);
    else
      items.value.delete(id);
    triggerRef(items);
  }

  function selectAll() {
    unref(currentIds).forEach(id => items.value.add(id));
    triggerRef(items);
  }

  return {
    enabled,
    items,
    start,
    toggleSelect,
    selectAll,
  };
}
