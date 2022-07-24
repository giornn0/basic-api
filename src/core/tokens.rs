use std::sync::Arc;

use jsonwebtoken::{
    decode, encode, errors::Error as JWTError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use warp::Rejection;
use std::cmp::Eq;
use crate::{
    config::{LogModel, Role},
    core::{errors::Error, server_model::Pool},
    utils::server::{reject_error, token_key},
};

pub trait HasSession {
    fn get_auth(self, log_model: LogModel) -> Result<AuthPayload, Error>;
}
pub trait FromToken {
    fn decode(token: String) -> Result<TokenData<Self>, Error>
    where
        Self: Sized;
    fn from_token(token: String) -> Result<Self, Error>
    where
        Self: Sized;
}

#[derive(Serialize)]
pub struct Token {
    token: String,
}
pub trait ToToken {
    fn get_auth<T: HasSession>(
        id: i32,
        log_model: LogModel,
        name: String,
        role: Role,
        exp: i64,
    ) -> AuthPayload;
    fn to_token(self) -> Result<Token, Rejection>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash,Eq, PartialEq)]
pub struct AuthPayload {
    id: i32,
    log_model: LogModel,
    name: String,
    role: Role,
    exp: i64,
}
impl AuthPayload{
    fn default(id: i32, log_model: LogModel, role: Role)->AuthPayload{
        AuthPayload { id, log_model, name: "pepe".to_owned(), role, exp: 5555 }
    }
    pub fn name(&self)->String{
        (*self).clone().name
    }
}

impl ToToken for AuthPayload
where
    Token: Serialize,
{
    fn get_auth<T: HasSession>(
        id: i32,
        log_model: LogModel,
        name: String,
        role: Role,
        exp: i64,
    ) -> AuthPayload {
        AuthPayload {
            id,
            log_model,
            name,
            role,
            exp,
        }
    }
    fn to_token(self) -> Result<Token, Rejection> {
        let token = encode_model(self).map_err(reject_error)?;
        Ok(Token { token })
    }
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
        .map_err(bad_token)
    }
    fn from_token(bearer: String) -> Result<AuthPayload, Error> {
        let mut token = bearer.split_whitespace();
        let decoded = AuthPayload::decode(String::from(token.nth(1).expect("Token Malformed")))?;
        Ok(decoded.claims)
    }
}

fn bad_token(err: JWTError) -> Error {
    println!("{}", err);
    Error::WrongToken
}
// fn bad_model(err: String) -> Error {
//     println!("{}", err);
//     Error::BadTokenization
// }

fn encode_model<AuthPayload: Serialize>(model: AuthPayload) -> Result<String, Error> {
    encode(
        &Header::new(Algorithm::HS256),
        &model,
        &EncodingKey::from_secret(token_key().as_bytes()),
    )
    .map_err(bad_token)
}
