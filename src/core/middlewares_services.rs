use std::{convert::Infallible, sync::Arc};

use warp::{Filter, Rejection, header};

use super::{
    server_model::Pool,
    token_model::{AuthPayload, FromToken},
};

pub fn with_pool(
    db_pool: Arc<Pool>,
) -> impl Filter<Extract = (Arc<Pool>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn with_authenticathed(
    db_pool: &Arc<Pool>,
) -> impl Filter<Extract = (AuthPayload,), Error = Rejection> + Clone {
    header::<String>("authorization")
        .and(with_pool(db_pool.clone()))
        .and_then(|token: String, db_pool: Arc<Pool>| async move {
            AuthPayload::from_token(token, db_pool).map_err(warp::reject::custom)
        })
}
