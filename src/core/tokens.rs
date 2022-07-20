use std::sync::Arc;

use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error as JWTError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use crate::{
    core::{errors::Error, server_model::Pool},
    utils::server::token_key,
};

use super::credentials::LogModel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    User,
    Client,
}

pub trait FromToken {
    fn decode(token: String) -> Result<TokenData<Self>, Error>
    where
        Self: Sized;
    fn from_token(token: String, db_pool: Arc<Pool>) -> Result<Self, Error>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthPayload {
    id: i32,
    log_model: LogModel,
    name: String,
    role: Role,
    exp: i64,
}

impl FromToken for AuthPayload {
    fn decode(token: String) -> Result<TokenData<Self>, Error>
    where
        Self: Sized,
    {
        decode::<AuthPayload>(
            &token,
            &DecodingKey::from_secret(token_key().as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(reject_error)
    }
    fn from_token(token: String, db_pool: Arc<Pool>) -> Result<AuthPayload, Error> {
        let decoded = AuthPayload::decode(token)?;
        Ok(decoded.claims)
    }
}

fn reject_error(err: JWTError) -> Error {
    println!("{}", err);
    Error::WrongToken
}

fn encode_model<T: Serialize>(model: &T) -> Result<String, Error> {
    encode(
        &Header::new(Algorithm::HS256),
        model,
        &EncodingKey::from_secret(token_key().as_bytes()),
    )
    .map_err(reject_error)
}
