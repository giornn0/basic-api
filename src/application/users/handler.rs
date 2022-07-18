use std::sync::Arc;

use warp::{reply::{Json, WithStatus}, Rejection,};

use crate::{core::{server_model::Pool, token_model::AuthPayload, response::{Response, Action}}, utils::database::get_pool};
use crate::utils::traits::Send;

use super::{model::{Queries, NewUser}, service::{get_user, create_user}};

pub async fn get_one(
    id: i32,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let user = get_user(id, &conn)?;
    Response::send(Action::Finded(user, ""))
}
pub async fn create_one(data: NewUser,current_user: AuthPayload, pool: Arc<Pool>)->Result<WithStatus<Json>,Rejection>{
    let conn = get_pool(pool)?;
    let new_user = create_user(data, &conn)?;
    Response::send(Action::Created(new_user, "User created succesfully"))
}
pub async fn get_index(
    queries: Queries,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    Ok(warp::reply::json(&AuthPayload::default()))
}
