use serde::{Deserialize, Serialize};

#[cfg(not(feature = "server"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub hash: String,
    pub file_path: String,
    pub file_size: i64,
}
#[cfg(feature = "server")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Image {
    pub id: String,
    pub hash: String,
    pub file_path: String,
    pub file_size: i64,
}

pub const DEFAULT_PAGE_SIZE: u32 = 30;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination<T> {
    pub data: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub total_page: u32,
    pub page_size: u32,
}
impl<T> Pagination<T> {
    pub fn with_page_size(data: Vec<T>, total: u32, page: u32, page_size: u32) -> Self {
        let total_page = f64::ceil(total as f64 / page_size as f64) as u32;
        Self {
            data,
            total,
            page,
            total_page,
            page_size,
        }
    }
    pub fn new(data: Vec<T>, total: u32, page: u32) -> Self {
        Self::with_page_size(data, total, page, DEFAULT_PAGE_SIZE)
    }

    pub fn with_count(data: Vec<T>, count: (i64,), page: u32) -> Self {
        Self::new(data, count.0 as u32, page)
    }
}
