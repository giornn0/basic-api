use std::sync::Arc;

use diesel::Connection;
use warp::{
    reply::{Json, WithStatus},
    Rejection, Reply,
};

use crate::{
    core::{errors::Error, helpers::send_with_headers, pagination::get_page_headers},
    utils::{server::reject_error, traits::Send},
};
use crate::{
    core::{
        response::{Action, Response},
        server_model::Pool,
        tokens::AuthPayload,
    },
    utils::database::get_pool,
};

use super::{
    model::{NewOrganization, Organization, OrganizationQueries, UpdateOrganization},
    service::{
        create_organization, get_organization, get_organization_page, remove_organization,
        update_organization,
    },
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
    Response::send(Action::Created(
        new_organization,
        "Organization created succesfully",
    ))
}
pub async fn update_one(
    id: i32,
    data: UpdateOrganization,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;

    let updated_organization = conn
        .transaction::<Organization, Error, _>(|| update_organization(data, id, &conn))
        .map_err(reject_error)?;
    Response::send(Action::Updated(
        updated_organization,
        "Organization updated succesfully",
    ))
}
pub async fn remove_one(
    id: i32,
    current_user: AuthPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    remove_organization(id, &conn)?;
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
