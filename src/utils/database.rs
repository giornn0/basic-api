use std::sync::Arc;

use DbError::NotFound;
use diesel::{r2d2::{PooledConnection, ConnectionManager}, PgConnection, result::Error as DbError};
use warp::{Rejection, reject::custom};
use crate::core::{server_model::Pool, errors::Error};

fn modify_error(db_error: diesel::r2d2::PoolError)->Rejection{
    println!("{}",db_error);
    custom(Error::FailedPool)
}

pub fn get_pool(pool: Arc<Pool>)->Result<PooledConnection<ConnectionManager<PgConnection>>, Rejection>{
    pool.get().map_err(modify_error)
}

pub fn reject_db_error(db_error: DbError)->Rejection{
    println!("{}",db_error);
    if db_error.eq(&NotFound){
        return custom(Error::DbNotFound)
    }
    custom(Error::WhileQuerying)
}