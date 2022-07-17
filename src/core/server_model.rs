use diesel::{r2d2::{ConnectionManager, self}, PgConnection};
use serde::Serialize;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize, Clone)]
pub struct Response{
    message: String,
    author: String
}
impl Default for Response{
    fn default() -> Self {
        Response{
            message: "Server working correctly".to_owned(),
            author: "giornn0".to_owned()
        }
    }
}