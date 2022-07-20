use crate::{
    schema::credentials::{
        self as credentials,
        dsl::{credentials as Table, email as Email},
    },
    utils::{database::reject_error, passwords::hash},
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    Insertable, PgConnection,
};
use diesel_derive_enum::DbEnum;
use http_api_problem::StatusCode;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};
use warp::{Rejection, reject::custom};

use super::errors::Error;

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone)]
pub enum LogModel {
    User,   //'user
    Client, //'client
    Worker, //'worker
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
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
    pub fn id(&self)->i32{
        self.id
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
impl NewCredential {
    fn hash_password(unhashed_pass: String) -> String {
        match hash(&unhashed_pass) {
            Some(hash) => hash,
            None => unhashed_pass,
        }
    }
    pub fn new(password: String, email: String, log_model: LogModel, state: Option<bool>) -> Self {
        NewCredential {
            password: NewCredential::hash_password(password),
            email,
            state,
            log_model,
        }
    }
}

pub trait GetCredential<T: Validate> {
    fn get_credential(&self) -> NewCredential;
}
pub trait GetRegister<T: Validate, I: Insertable<G>, G> {
    fn get_register(&self, credential_id: i32) -> I;
}

pub fn new_credential(
    value: NewCredential,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Credential,Rejection> {
    value
        .insert_into(Table)
        .get_result(conn)
        .map_err(reject_error)
}
pub fn unique_credential_mail(email: &String, conn:&PooledConnection<ConnectionManager<PgConnection>>)->Result<(),Rejection>{

    let registers = credentials::table.filter(Email.eq_all(email)).load::<Credential>(conn).map_err(reject_error)?;
        
    if registers.len() > 0 {
        // the value of the username will automatically be added later
       return Err(custom(Error::Redaction(StatusCode::BAD_REQUEST,format!("The {} is already used",email))));
    }

    Ok(())
}