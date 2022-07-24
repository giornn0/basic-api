use crate::{
    core::pagination::{Page, Paginated, Paginator},
    utils::database::reject_db_error, config::DBPool,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use warp::Rejection;

use super::model::{NewOrganization, Organization, OrganizationQueries, UpdateOrganization};
use crate::schema::organizations::dsl::{id as Id, organizations as Table};

pub fn get_organization(
    id: i32,
    conn: &DBPool,
) -> Result<Organization, Rejection> {
    Table.find(id).get_result(conn).map_err(reject_db_error)
}
pub fn create_organization(
    data: NewOrganization,
    conn: &DBPool,
) -> Result<Organization, Rejection> {
    data.insert_into(Table)
        .get_result(conn)
        .map_err(reject_db_error)
}
pub fn update_organization(
    data: UpdateOrganization,
    id: i32,
    conn: &DBPool,
) -> Result<Organization, Rejection> {
    diesel::update(Table.filter(Id.eq(id)))
        .set(data)
        .get_result(conn)
        .map_err(reject_db_error)
}
pub fn remove_organization(
    id: i32,
    conn: &DBPool,
) -> Result<usize, Rejection> {
    diesel::delete(Table.filter(Id.eq(id)))
        .execute(conn)
        .map_err(reject_db_error)
}
pub fn get_organization_page(
    queries: OrganizationQueries,
    conn: &DBPool,
) -> Result<Paginated<Vec<Organization>>, Rejection> {
    let count: i64 = Table.count().get_result(conn).map_err(reject_db_error)?;
    let (take, page) = queries.get_page();
    let organizations = Table
        .limit(take)
        .offset((page - 1) * take)
        .load(conn)
        .map_err(reject_db_error)?;
    Ok(Organization::paginate(organizations, page, take, count))
}
