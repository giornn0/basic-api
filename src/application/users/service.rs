use crate::{
    core::{
        pagination::{Page, Paginated, Paginator},
        tokens::{AuthPayload, HasSession},
    },
    config::{LogModel, DBPool},
    utils::{database::reject_db_error, server::reject_error},
};
use diesel::{
    prelude::*,
};
use warp::Rejection;

use super::model::{NewUser, UpdateUser, User, UserQueries};
use crate::schema::users::dsl::{credential_id as CredentialId, id as Id, users as Table};

pub fn get_user(
    id: i32,
    conn: &DBPool,
) -> Result<User, Rejection> {
    Table.find(id).get_result(conn).map_err(reject_db_error)
}
pub fn create_user(
    data: NewUser,
    conn: &DBPool,
) -> Result<User, Rejection> {
    data.insert_into(Table)
        .get_result(conn)
        .map_err(reject_db_error)
}
pub fn update_user(
    data: UpdateUser,
    id: i32,
    conn: &DBPool,
) -> Result<User, Rejection> {
    diesel::update(Table.filter(Id.eq(id)))
        .set(data)
        .get_result(conn)
        .map_err(reject_db_error)
}
pub fn remove_user(
    id: i32,
    conn: &DBPool,
) -> Result<usize, Rejection> {
    diesel::delete(Table.filter(Id.eq(id)))
        .execute(conn)
        .map_err(reject_db_error)
}

pub fn get_user_page(
    queries: UserQueries,
    conn: &DBPool,
) -> Result<Paginated<Vec<User>>, Rejection> {
    let count: i64 = Table.count().get_result(conn).map_err(reject_db_error)?;
    let (take, page) = queries.get_page();
    let users = Table
        .limit(take)
        .offset((page - 1) * take)
        .load(conn)
        .map_err(reject_db_error)?;
    Ok(User::paginate(users,page,take,count))
}

pub fn get_by_credential(
    credential: i32,
    conn: &DBPool,
) -> Result<User, Rejection> {
    Table
        .filter(CredentialId.eq(credential))
        .get_result(conn)
        .map_err(reject_db_error)
}

pub fn get_user_payload(
    credential_id: i32,
    conn: &DBPool,
) -> Result<AuthPayload, Rejection> {
    let user = get_by_credential(credential_id, conn)?;
    user.get_auth(LogModel::User).map_err(reject_error)
}
