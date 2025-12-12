use crate::db::DbPagination;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};
use std::num::NonZeroU32;

const DEFAULT_PAGE: NonZeroU32 = NonZeroU32::new(1).unwrap();
const DEFAULT_PAGE_SIZE: NonZeroU32 = NonZeroU32::new(10).unwrap();
const MAX_PAGE_SIZE: NonZeroU32 = NonZeroU32::new(100).unwrap();

#[derive(Clone, Copy, Deserialize, utoipa::IntoParams)]
pub struct PaginationQuery {
    #[param(value_type = Option<u32>, minimum = 1, default = 1)]
    page: Option<NonZeroU32>,
    #[param(value_type= Option<u32>, minimum = 1, maximum = 100, default = 10)]
    size: Option<NonZeroU32>,
}

impl Debug for PaginationQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{ page: {:?}, size: {:?} }}"#, self.page, self.size)
    }
}

impl PaginationQuery {
    pub fn check(&self) -> Result<PaginationChecked, PageSizeOutOfRange> {
        let page = self.page.unwrap_or(DEFAULT_PAGE);
        let size = self.size.unwrap_or(DEFAULT_PAGE_SIZE);
        if size > MAX_PAGE_SIZE {
            return Err(PageSizeOutOfRange);
        }
        Ok(PaginationChecked { page, size })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PaginationChecked {
    page: NonZeroU32,
    size: NonZeroU32,
}

impl From<PaginationChecked> for DbPagination {
    fn from(PaginationChecked { page, size }: PaginationChecked) -> Self {
        let offset = (page.get() - 1) * size.get();
        Self {
            limit: size.get() as i64,
            offset: offset as i64,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Page Size Out of Range: max allow value is {MAX_PAGE_SIZE}")]
pub struct PageSizeOutOfRange;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    #[schema(value_type = u32)]
    pub current_page: NonZeroU32,
    #[schema(value_type = u32)]
    pub page_size: NonZeroU32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, PaginationChecked { page, size }: PaginationChecked) -> Self {
        Self {
            data,
            current_page: page,
            page_size: size,
        }
    }
}
