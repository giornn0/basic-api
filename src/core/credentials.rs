use crate::{
    config::LogModel,
    schema::credentials::{
        self as credentials,
        dsl::{credentials as Table, email as Email},
    },
    utils::{database::reject_db_error, traits::HashedValue},
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    Insertable, PgConnection,
};
use http_api_problem::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;
use warp::{reject::custom, Rejection};

use super::errors::Error;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
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
    pub fn password(&self) -> String {
        let cloned = (*self).clone();
        String::from(cloned.password)
    }
    pub fn log_model(&self) -> LogModel {
        let clone = self.clone();
        clone.log_model
    }
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "credentials"]
pub struct NewCredential {
    password: String,
    email: String,
    state: Option<bool>,
    log_model: LogModel,
}
impl HashedValue for NewCredential {}
impl NewCredential {
    pub fn new(
        unhashed: String,
        email: String,
        log_model: LogModel,
        state: Option<bool>,
    ) -> Result<Self, Rejection> {
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
