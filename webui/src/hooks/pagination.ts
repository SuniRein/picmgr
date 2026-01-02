interface PaginationOptions {
  initialPageSize: number;
  onPageChange: (page: number, size: number) => void | Promise<void>;
}

export function usePagination(options: PaginationOptions) {
  const currentPage = ref(1);
  const pageSize = ref(options.initialPageSize);

  function resetPagination() {
    currentPage.value = 1;
  }

  watch([currentPage, pageSize], async ([newPage, newSize]) => {
    await options.onPageChange(newPage, newSize);
  });

  return {
    currentPage,
    pageSize,
    resetPagination,
  };
}
