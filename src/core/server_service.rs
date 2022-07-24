use std::sync::Arc;

use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use crate::core::server_model::Response;
use warp::{reply::Json, Rejection};

use crate::core::server_model::Pool as DbPool;

pub async fn up_server()->Result<Json, Rejection>{
    Ok(warp::reply::json(&Response::default()))
}

pub fn start_db()->Arc<DbPool>{
    let db_url = std::env::var("DATABASE_URL").expect("Missing database credentials!");
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    Arc::new(
        Pool::builder()
            .build(manager)
            .expect("Failed connection to the database.")
    )

}  
pub fn start_simple_db()->DbPool{
    let db_url = std::env::var("DATABASE_URL").expect("Missing database credentials!");
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    Pool::builder()
        .build(manager)
        .expect("Failed connection to the database.")
}  
