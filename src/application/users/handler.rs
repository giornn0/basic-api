use std::sync::Arc;

use warp::{
    reply::{Json, WithStatus},
    Rejection,
};

use crate::{utils::traits::Send, core::credentials::unique_credential_mail};
use crate::{
    core::{
        credentials::{new_credential, GetCredential, GetRegister},
        response::{Action, Response},
        server_model::Pool,
        tokens::AuthPayload,
    },
    utils::database::get_pool,
};

use super::{
    model::{Queries, UpdateUser, UserRegister},
    service::{create_user, get_user, remove_user, update_user},
};

pub async fn get_one(
    id: i32,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let user = get_user(id, &conn)?;
    Response::send(Action::Finded(user, ""))
}
pub async fn create_one(
    data: UserRegister,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    unique_credential_mail(data.email(), &conn)?;
    let values_credential = data.get_credential()?;
    let new_credential = new_credential(values_credential, &conn)?;
    let new_user = create_user(data.get_register(new_credential.id()), &conn)?;
    Response::send(Action::Created(new_user, "User created succesfully"))
}
pub async fn update_one(
    id: i32,
    data: UpdateUser,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let updated_user = update_user(data, id, &conn)?;
    Response::send(Action::Updated(updated_user, "User updated succesfully"))
}
pub async fn remove_one(
    id: i32,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let removed = remove_user(id, &conn)?;
    Response::<bool>::send(Action::Removed("User removed succesfully"))
}
pub async fn get_index(
    queries: Queries,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    Response::<bool>::send(Action::Removed("User removed succesfully"))
}
