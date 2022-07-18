use std::sync::Arc;

use warp::{reply::{Json, WithStatus}, Rejection,};

use crate::{core::{server_model::Pool, token_model::AuthPayload, response::{Response, Action}}, utils::database::get_pool};
use crate::utils::traits::Send;

use super::{model::{Queries, NewCredential}, service::{get_credential, create_credential}};

pub async fn get_one(
    id: i32,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let credential = get_credential(id, &conn)?;
    Response::send(Action::Finded(credential, ""))
}
pub async fn create_one(data: NewCredential,current_user: AuthPayload, pool: Arc<Pool>)->Result<WithStatus<Json>,Rejection>{
    let conn = get_pool(pool)?;
    let new_credential = create_credential(data, &conn)?;
    Response::send(Action::Created(new_credential, "credential created succesfully"))
}
pub async fn get_index(
    queries: Queries,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    Ok(warp::reply::json(&AuthPayload::default()))
}
