use serde::{Deserialize, Serialize};
use warp::{http::HeaderValue, hyper::HeaderMap};

use crate::config::default_pager;

// pub trait
pub trait Paginator {
    fn paginate<T>(data: Vec<T>, page: i64, take: i64, count: i64) -> Paginated<Vec<T>> where T: Serialize,{
        let current_length = data.len();
        Paginated {
            metadata: get_page_data(page, take, count, current_length as i64),
            data,
        }
    }
}
#[derive(Serialize)]
pub struct PageData {
    current_page: i64,
    page_items: i64,
    page_count: i64,
    total_items: i64,
}
#[derive(Serialize)]
pub struct Paginated<T: Serialize> {
    pub metadata: PageData,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct Pager {
    pub page: Option<i64>,
    pub take: Option<i64>,
}

pub trait Page {
    fn get_page(&self) -> (i64, i64) {
        default_pager()
    }
}
fn get_page_data(page: i64, take: i64, total_items: i64, page_items: i64) -> PageData {
    let page_count = if total_items % take > 0 {
        total_items / take + 1
    } else {
        total_items / take
    };
    let current_page = if page > page_count { page_count } else { page };
    PageData {
        current_page,
        page_items,
        page_count,
        total_items,
    }
}
pub fn get_page_headers(metadata: PageData) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("x-current-page", HeaderValue::from(metadata.current_page));
    headers.insert("x-page-items", HeaderValue::from(metadata.page_items));
    headers.insert("x-page-count", HeaderValue::from(metadata.page_count));
    headers.insert("x-total-items", HeaderValue::from(metadata.total_items));
    headers
}
