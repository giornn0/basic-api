use std::sync::Arc;

use crate::core::{errors::Error, server_model::Pool, server_service::start_simple_db};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    result::Error as DbError,
    PgConnection,
};
use warp::{reject::custom, Rejection};

fn modify_error(db_error: diesel::r2d2::PoolError) -> Rejection {
    println!("{}", db_error);
    custom(Error::FailedPool)
}

pub fn get_pool(
    pool: Arc<Pool>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Rejection> {
    pool.get().map_err(modify_error)
}
pub fn get_ws_pool() -> Result<PooledConnection<ConnectionManager<PgConnection>>, Rejection> {
    start_simple_db().get().map_err(modify_error)
}

pub fn reject_db_error(db_error: DbError) -> Rejection {
   custom(Error::from(db_error))
}

pub fn db_error(error: DbError)->Error{
    Error::from(error)
}