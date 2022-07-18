use warp::Rejection;
use diesel::{prelude::*, r2d2::{ConnectionManager, PooledConnection}};
use crate::{utils::database::reject_error};

use super::model::{User, NewUser};
use crate::schema::users::dsl::users as Table;

pub fn get_user(id: i32, conn: &PooledConnection<ConnectionManager<PgConnection>>)->Result<User,Rejection>{
    Table.find(id).get_result(conn).map_err(reject_error)
}
pub fn create_user(data: NewUser, conn: &PooledConnection<ConnectionManager<PgConnection>>)->Result<User,Rejection>{
    data.insert_into(Table).get_result(conn).map_err(reject_error)
}