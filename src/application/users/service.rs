use crate::utils::database::reject_error;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use warp::Rejection;

use super::model::{NewUser, UpdateUser, User};
use crate::schema::users::dsl::{id as Id, users as Table};

pub fn get_user(
    id: i32,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Rejection> {
    Table.find(id).get_result(conn).map_err(reject_error)
}
pub fn create_user(
    data: NewUser,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Rejection> {
    data.insert_into(Table)
        .get_result(conn)
        .map_err(reject_error)
}
pub fn update_user(
    data: UpdateUser,
    id: i32,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Rejection> {
    diesel::update(Table.filter(Id.eq(id)))
        .set(data)
        .get_result(conn)
        .map_err(reject_error)
}
pub fn remove_user(
    id: i32,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<usize, Rejection> {
    diesel::delete(Table.filter(Id.eq(id)))
        .execute(conn)
        .map_err(reject_error)
}
