#[macro_use]
extern crate diesel;

mod app;
mod core;
mod utils;
mod application;
mod schema;
use app::app;
use crate::core::server_service::start_db;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::dotenv();

    let pool = start_db();

    app(&pool).await;

    Ok(())
}
