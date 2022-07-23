use crate::{utils::{database::reject_db_error, server::reject_error}, core::{tokens::{AuthPayload, HasSession}, credentials::LogModel, pagination::Pagination}};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection}, dsl::count_star, sql_types::BigInt
};
use warp::Rejection;

use super::model::{NewUser, UpdateUser, User};
use crate::schema::users::dsl::{id as Id, users as Table, credential_id as CredentialId};

pub fn get_user(
    id: i32,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Rejection> {
    Table.find(id).get_result(conn).map_err(reject_db_error)
}
pub fn create_user(
    data: NewUser,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Rejection> {
    data.insert_into(Table)
        .get_result(conn)
        .map_err(reject_db_error)
}
pub fn update_user(
    data: UpdateUser,
    id: i32,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Rejection> {
    diesel::update(Table.filter(Id.eq(id)))
        .set(data)
        .get_result(conn)
        .map_err(reject_db_error)
}
pub fn remove_user(
    id: i32,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<usize, Rejection> {
    diesel::delete(Table.filter(Id.eq(id)))
    .execute(conn)
    .map_err(reject_db_error)
}


pub fn get_user_page(
    page: Option<i64>,
    _take:Option<i64>,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
)->Result<Vec<User>, Rejection>{
    let test: i64 = Table.count().get_result(conn).map_err(reject_db_error)?;
    println!("{}",test);
    let take = _take.unwrap_or(5); 
    Table
        .limit(take)
        .offset((page.unwrap_or(1)-1) * take)
        .load(conn)
        .map_err(reject_db_error)
}

pub fn get_by_credential(credential: i32, conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Rejection> {
    Table.filter(CredentialId.eq(credential)).get_result(conn).map_err(reject_db_error)
}

pub fn get_user_payload(credential_id: i32,conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<AuthPayload, Rejection> {
    let user = get_by_credential(credential_id, conn)?;
    user.get_auth(LogModel::User).map_err(reject_error)
}