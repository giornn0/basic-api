use std::sync::Arc;

use warp::{
    get,
    path::{end, param},
    post, query, Filter, Rejection, Reply,
};

use crate::core::{
    middlewares::{with_authenticathed, with_pool, with_valid_json},
    server_model::Pool,
};

use super::{
    handler::{get_index, get_one, create_one},
};

pub fn credential_router(
    db_pool: &Arc<Pool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let root = warp::path("credentials");
    // .and(with_authenticathed(db_pool))
    // .and(with_pool(db_pool.clone()));
    let index = root
        .and(get())
        .and(query())
        .and(end())
        .and(with_authenticathed(db_pool))
        .and(with_pool(db_pool.clone()))
        .and_then(get_index);
    let one = root
        .and(param())
        .and(get())
        .and(end())
        .and(with_authenticathed(db_pool))
        .and(with_pool(db_pool.clone()))
        .and_then(get_one);
    let create = root
        .and(post())
        .and(end())
        .and(with_valid_json())
        .and(with_authenticathed(db_pool))
        .and(with_pool(db_pool.clone()))
        .and_then(create_one);
    one.or(index).or(create)
}
