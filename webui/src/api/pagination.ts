export interface PaginationParams {
  page: number;
  size: number;
}

export interface PaginationResponse<T> {
  data: T[];
  current_page: number;
  page_size: number;
}
