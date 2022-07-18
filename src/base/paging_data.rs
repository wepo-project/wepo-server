
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

/// 分页数据
#[derive(Debug, Serialize)]
struct PagingData<T: Serialize> {
    page: i64,
    next: bool,
    list: Vec<T>,
}

impl<T: Serialize> PagingData<T> {
    pub fn new(page: i64, next: bool, list: Vec<T>) -> Self {
        Self { page, next, list }
    }
}

pub struct Paging<'a> {
    /// 每页个数
    limit: &'static i64,
    /// 当前页数
    page: &'a i64,
    /// 偏移
    offset: i64,
}

impl<'a> Paging<'a> {
    pub fn default(page: &'a i64) -> Self {
        /// 每页的数量
        const COUNT_PER_PAGE: i64 = 20;
        Self::new(&COUNT_PER_PAGE, page)
    }
    pub fn new(limit: &'static i64, page: &'a i64) -> Self {
        Self { limit, page, offset: limit * (page - 1) }
    }
    pub fn limit(&self) -> &i64 {
        self.limit
    }
    pub fn offset(&self) -> &i64 {
        &self.offset
    }
    // pub async fn get_data<T: Serialize, F>(&self, func: F) -> Result<HttpResponse, MyError>
    // where
    //     F: Fn(&Self) -> Pin<Box<dyn Future<Output = Result<Vec<T>, MyError>>>>
    // {
    //     let list = func(&self).await?;
    //     let next = &(list.len() as i64) >= self.limit;
    //     Ok(HttpResponse::Ok().json(
    //         PagingData::new(*self.page, next, list)
    //     ))
    // }
    pub fn finish<T: Serialize, E>(&self, list: Vec<T>) -> Result<HttpResponse, E> {
        let next = &(list.len() as i64) >= self.limit;
        Ok(HttpResponse::Ok().json(
            PagingData::new(*self.page, next, list)
        ))
    }
}


#[derive(Deserialize, Serialize)]
/// 通用分页请求
pub struct GetPageDTO {
    pub page: i64,
}
