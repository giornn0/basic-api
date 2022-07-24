use std::sync::Arc;

use warp::{
    reply::{Json, WithStatus},
    Rejection, Reply,
};

use crate::{utils::traits::Send, core::{pagination::get_page_headers, helpers::send_with_headers}};
use crate::{
    core::{
        response::{Action, Response},
        server_model::Pool,
        tokens::AuthPayload,
    },
    utils::database::get_pool,
};

use super::{
    model::{Queries, UpdateOrganization, NewOrganization, OrganizationQueries},
    service::{create_organization, get_organization, remove_organization, update_organization, get_organization_page},
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
    queries: OrganizationQueries,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<impl Reply, Rejection> {
    let conn = get_pool(pool)?;
    let paginated = get_organization_page(queries, &conn)?;
    let headers = get_page_headers(paginated.metadata);
    let reply = Response::send(Action::Indexed(paginated.data))?;
    send_with_headers(reply, headers)
}
