use std::sync::Arc;

use crate::utils::{server::token_key, traits::Send};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use warp::{
    get,
    path::end,
    reply::{Json},
    Filter, Rejection, Reply,
};

use crate::core::server_model::Pool as DbPool;

use super::{
    middlewares::with_refreshed,
    response::{Action, Response},
    tokens::{AuthPayload, ToToken},
};

pub async fn up_server() -> Result<Json, Rejection> {
    Ok(warp::reply::json(&Response::<bool>::default()))
}

pub fn start_db() -> Arc<DbPool> {
    let db_url = std::env::var("DATABASE_URL").expect("Missing database credentials!");
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    Arc::new(
        Pool::builder()
            .build(manager)
            .expect("Failed connection to the database."),
    )
}
pub fn start_simple_db() -> DbPool {
    let db_url = std::env::var("DATABASE_URL").expect("Missing database credentials!");
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    Pool::builder()
        .build(manager)
        .expect("Failed connection to the database.")
}
// pub fn refresh_token(
//     pool: &Arc<DbPool>,
// ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
//     warp::path("refresh")
//         .and(get())
//         .and(end())
//         .and(with_refreshed())
//         .map(|payload: AuthPayload| {
//             let token = payload.to_token(token_key())?;
//             Response::send(Action::Refreshed(token, ""))
//         })
// }
