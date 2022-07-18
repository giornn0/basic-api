use warp::Rejection;
use diesel::{prelude::*, r2d2::{ConnectionManager, PooledConnection}};
use crate::{utils::database::reject_error};

use super::model::{Credential, NewCredential};
use crate::schema::credentials::dsl::credentials as Table;

pub fn get_credential(id: i32, conn: &PooledConnection<ConnectionManager<PgConnection>>)->Result<Credential,Rejection>{
    Table.find(id).get_result(conn).map_err(reject_error)
}
pub fn create_credential(mut data: NewCredential, conn: &PooledConnection<ConnectionManager<PgConnection>>)->Result<Credential,Rejection>{
    data.hash_password();
    data.insert_into(Table).get_result(conn).map_err(reject_error)
}