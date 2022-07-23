use serde::Serialize;

pub struct Paginated{
    current_page: u32,
    page_items: u32,
    page_count: u32,
    total_items: u32,
}

pub trait Pager<T: Serialize>{
    fn pager()->String;
}

pub struct Pagination{
    page_count: u32,
    take: u32,
}