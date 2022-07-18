use std::sync::Arc;

use diesel::{r2d2::{PooledConnection, ConnectionManager}, PgConnection};
use warp::{Rejection, reject::{custom, self}};
use crate::core::{server_model::Pool, errors::Error};

fn modify_error(db_error: diesel::r2d2::PoolError)->Rejection{
    println!("{}",db_error);
    custom(Error::FailedPool)
}

pub fn get_pool(pool: Arc<Pool>)->Result<PooledConnection<ConnectionManager<PgConnection>>, Rejection>{
    pool.get().map_err(modify_error)
}

pub fn reject_error(db_error: diesel::result::Error)->Rejection{
    println!("{}",db_error);
    custom(Error::WhileQuerying)
}