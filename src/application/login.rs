use std::sync::Arc;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use http_api_problem::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;
use warp::{
    path::end,
    post,
    reject::custom,
    reply::{Json, WithStatus},
    Filter, Rejection, Reply,
};

use crate::{
    config::LogModel,
    core::{
        credentials::Credential,
        errors::Error,
        middlewares::{with_pool, with_valid_json},
        response::{Action, Response},
        server_model::Pool,
        tokens::{AuthPayload, ToToken, LoginTokens},
    },
    utils::{
        database::{get_pool, reject_db_error},
        passwords::auth_hash, server::{token_key, token_key_refresh},
    },
};
use crate::{
    schema::credentials::dsl::{credentials as Table, email as Email},
    utils::traits::Send,
};

use super::users::service::get_user_payload;

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 5, max = 255))]
    password: String,
    #[validate(length(min = 2, max = 50))]
    email: String,
}

fn get_credential(
    login_payload: LoginPayload,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Credential, Rejection> {
    let credential: Credential = Table
        .filter(Email.eq(&login_payload.email))
        .get_result(conn)
        .map_err(reject_db_error)?;
    if auth_hash(credential.password(), login_payload.password) {
        return Ok(credential);
    }
    Err(custom(Error::Unauthorized))
}

async fn login_handler(
    log_payload: LoginPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let credential = get_credential(log_payload, &conn)?;
    let session: AuthPayload = match credential.log_model() {
        LogModel::User => get_user_payload(credential.id(), &conn),
        LogModel::Client => Err(custom(Error::Redaction(
            StatusCode::NOT_IMPLEMENTED,
            "Modelo sin implementar".to_owned(),
        ))),
        LogModel::Worker => Err(custom(Error::Redaction(
            StatusCode::NOT_IMPLEMENTED,
            "Modelo sin implementar".to_owned(),
        ))),
    }?;
    
    let auth = session.to_token(token_key())?;
    let refresh = session.to_token(token_key_refresh())?;

    Response::send(Action::Logged(LoginTokens::new(auth,refresh), "Bienvenido"))
}

pub fn login(pool: &Arc<Pool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("login")
        .and(post())
        .and(end())
        .and(with_valid_json())
        .and(with_pool(pool.clone()))
        .and_then(login_handler)
}
