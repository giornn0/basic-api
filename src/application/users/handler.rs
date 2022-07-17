use std::{convert::Infallible, sync::Arc};

use warp::{reply::Json, Rejection};

use crate::core::{errors::Error, server_model::Pool, token_model::AuthPayload};

use super::model::Queries;

pub async fn get_one(
    id: i32,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    
    Ok(warp::reply::json(&current_user))
    // Err(warp::reject::custom(Error::WhileQuerying))
}
pub async fn get_index(
    queries: Queries,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    Ok(warp::reply::json(&AuthPayload::default()))
}
