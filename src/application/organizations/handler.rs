use std::sync::Arc;

use warp::{
    reply::{Json, WithStatus},
    Rejection,
};

use crate::utils::traits::Send;
use crate::{
    core::{
        response::{Action, Response},
        server_model::Pool,
        tokens::AuthPayload,
    },
    utils::database::get_pool,
};

use super::{
    model::{Queries, UpdateOrganization, NewOrganization},
    service::{create_organization, get_organization, remove_organization, update_organization},
};

pub async fn get_one(
    id: i32,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let organization = get_organization(id, &conn)?;
    Response::send(Action::Finded(organization, ""))
}
pub async fn create_one(
    data: NewOrganization,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let new_organization = create_organization(data, &conn)?;
    Response::send(Action::Created(new_organization, "Organization created succesfully"))
}
pub async fn update_one(
    id: i32,
    data: UpdateOrganization,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let updated_organization = update_organization(data, id, &conn)?;
    Response::send(Action::Updated(updated_organization, "Organization updated succesfully"))
}
pub async fn remove_one(
    id: i32,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let removed = remove_organization(id, &conn)?;
    Response::<bool>::send(Action::Removed("Organization removed succesfully"))
}
pub async fn get_index(
    queries: Queries,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    Response::<bool>::send(Action::Removed("Organization removed succesfully(psyche)"))
}
