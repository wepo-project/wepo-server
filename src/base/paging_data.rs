use serde::{Deserialize, Serialize};

/// 分页数据
#[derive(Debug, Serialize)]
pub struct PagingData<T: Serialize> {
    pub page: i64,
    pub next: bool,
    pub list: Vec<T>,
}

impl<T: Serialize> PagingData<T> {
    pub fn new(page: i64, next: bool, list: Vec<T>) -> Self {
        Self { page, next, list }
    }
}

pub struct PagingDataBuilder<'a> {
    /// 每页个数
    limit: &'static i64,
    /// 当前页数
    page: &'a i64,
}

impl<'a> PagingDataBuilder<'a> {
    pub fn new(size: &'static i64, page: &'a i64) -> Self {
        Self { limit: size, page }
    }
    pub fn set_list<T: Serialize>(&self, list: Vec<T>) -> PagingData<T> {
        let next = &(list.len() as i64) >= self.limit;
        PagingData::new(*self.page, next, list)
    }
}


#[derive(Deserialize, Serialize)]
/// 通用分页请求
pub struct GetPageDTO {
    pub page: i64,
}
