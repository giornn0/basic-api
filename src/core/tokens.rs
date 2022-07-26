use crate::{
    config::{LogModel, Role},
    core::errors::Error,
    utils::server::{reject_error, token_key},
};
use jsonwebtoken::{
    decode, encode, errors::Error as JWTError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use warp::Rejection;

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
    fn get_auth<T: HasSession>(id: i32, log_model: LogModel, role: Role, exp: i64) -> AuthPayload;
    fn to_token(self) -> Result<Token, Rejection>;
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct AuthPayload {
    id: i32,
    log_model: LogModel,
    role: Role,
    exp: i64,
}
impl AuthPayload {
    pub fn default(
        opt_id: Option<i32>,
        opt_log_model: Option<LogModel>,
        opt_role: Option<Role>,
    ) -> AuthPayload {
        AuthPayload {
            id: opt_id.unwrap_or(5),
            log_model: opt_log_model.unwrap_or(LogModel::Worker),
            role: opt_role.unwrap_or(Role::Client),
            exp: 5555,
        }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn log_model(&self) -> LogModel {
        self.log_model
    }
    pub fn role(&self) -> Role {
        self.role
    }
}
impl ToToken for AuthPayload
where
    Token: Serialize,
{
    fn get_auth<T: HasSession>(id: i32, log_model: LogModel, role: Role, exp: i64) -> AuthPayload {
        AuthPayload {
            id,
            log_model,
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
