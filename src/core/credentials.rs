use std::sync::Arc;

use crate::{
    schema::credentials::{
        self as credentials,
        dsl::{credentials as Table, email as Email},
    },
    utils::{
        database::{get_pool, reject_db_error},
        traits::{ HashedValue, Send}, passwords::auth_hash,
    }, application::users::service::get_user_payload,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    Insertable, PgConnection,
};
use diesel_derive_enum::DbEnum;
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

use super::{
    errors::Error,
    middlewares::{with_pool, with_valid_json},
    response::{Action, Response},
    server_model::Pool, tokens::{AuthPayload, ToToken, Token},
};

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone)]
pub enum LogModel {
    User,   //'user
    Client, //'client
    Worker, //'worker
}

#[derive(Queryable, Serialize, Deserialize, Debug,Clone)]
pub struct Credential {
    id: i32,
    password: String,
    email: String,
    state: Option<bool>,
    log_model: LogModel,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}
impl Credential {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn password(&self)->String{
        let cloned = (*self).clone();
        String::from(cloned.password)
    }
}
#[derive(Serialize, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 5, max = 255))]
    password: String,
    #[validate(length(min = 2, max = 50))]
    email: String,
}
#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "credentials"]
pub struct NewCredential {
    password: String,
    email: String,
    state: Option<bool>,
    log_model: LogModel,
}
impl HashedValue for NewCredential{}
impl NewCredential {
    pub fn new(unhashed: String, email: String, log_model: LogModel, state: Option<bool>) -> Result<Self, Rejection> {
        let password = NewCredential::hashed_value(unhashed)?;
        Ok(NewCredential {
            password,
            email,
            state,
            log_model,
        })
    }
}

pub trait GetCredential<T: Validate> {
    fn get_credential(&self) -> Result<NewCredential, Rejection>;
}
pub trait GetRegister<T: Validate, I: Insertable<G>, G> {
    fn get_register(&self, credential_id: i32) -> I;
}

pub fn new_credential(
    value: NewCredential,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Credential, Rejection> {
    value
        .insert_into(Table)
        .get_result(conn)
        .map_err(reject_db_error)
}
pub fn unique_credential_mail(
    email: &String,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<(), Rejection> {
    let registers = credentials::table
        .filter(Email.eq_all(email))
        .load::<Credential>(conn)
        .map_err(reject_db_error)?;

    if registers.len() > 0 {
        // the value of the username will automatically be added later
        return Err(custom(Error::Redaction(
            StatusCode::BAD_REQUEST,
            format!("The {} is already used", email),
        )));
    }
    Ok(())
}

fn get_credential(login_payload: LoginPayload, conn:&PooledConnection<ConnectionManager<PgConnection>> )-> Result<Credential, Rejection>{
    let credential: Credential = Table.filter(Email.eq(&login_payload.email)).get_result(conn).map_err(reject_db_error)?;
    if auth_hash(credential.password(), login_payload.password) {
        return Ok(credential)
    }
    Err(custom(Error::Unauthorized))
}

async fn login_handler(
    log_payload: LoginPayload,
    pool: Arc<Pool>,
) -> Result<WithStatus<Json>, Rejection> {
    let conn = get_pool(pool)?;
    let credential = get_credential(log_payload, &conn)?;
    let session: AuthPayload = match credential.log_model{
        LogModel::User=> get_user_payload(credential.id, &conn),
        LogModel::Client=>Err(custom(Error::Redaction(StatusCode::NOT_IMPLEMENTED, "Modelo sin implementar".to_owned()))),
        LogModel::Worker=>Err(custom(Error::Redaction(StatusCode::NOT_IMPLEMENTED, "Modelo sin implementar".to_owned()))),
    }?;
    let token = session.to_token()?;
    Response::send(Action::Logged(token,"Bienvenido"))
}

pub fn login(pool: &Arc<Pool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("login")
        .and(post())
        .and(end())
        .and(with_valid_json())
        .and(with_pool(pool.clone()))
        .and_then(login_handler)
}
