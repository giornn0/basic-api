use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use crate::core::{
    middlewares_services::{with_authenticathed, with_pool},
    server_model::Pool,
};

use super::handler::{get_one, get_index};

pub fn users_router(
    db_pool: &Arc<Pool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let root = warp::path("users");
        // .and(with_authenticathed(db_pool))
        // .and(with_pool(db_pool.clone()));
        let get_index = root
        .and(warp::get())
        .and(warp::query())
        .and(warp::path::end())
        .and(with_authenticathed(db_pool))
        .and(with_pool(db_pool.clone()))
        .and_then(get_index);
        let get_one = root
        .and(warp::path::param())
        .and(warp::get())
        .and(warp::path::end())
        .and(with_authenticathed(db_pool))
        .and(with_pool(db_pool.clone()))
        .and_then(get_one);
    get_one.or(get_index)
}
