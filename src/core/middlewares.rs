use std::{convert::Infallible, sync::Arc};

use serde::de::DeserializeOwned;
use validator::Validate;
use warp::{header, reject::custom, Filter, Rejection};

use super::{
    helpers::validate,
    server_model::Pool,
    tokens::{AuthPayload, FromToken},
};

pub fn with_pool(
    db_pool: Arc<Pool>,
) -> impl Filter<Extract = (Arc<Pool>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn with_authenticathed(
    db_pool: &Arc<Pool>,
) -> impl Filter<Extract = (AuthPayload,), Error = Rejection> + Clone {
    header::<String>("authorization")
        .and(with_pool(db_pool.clone()))
        .and_then(|token: String, db_pool: Arc<Pool>| async move {
            AuthPayload::from_token(token, db_pool).map_err(custom)
        })
}
pub fn with_valid_json<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: DeserializeOwned + Validate + Send,
{
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
        .and_then(|value| async move { validate(value).map_err(custom) })
}
