use crate::{
    config::{LogModel, Role},
    core::errors::Error,
    utils::server::{reject_error, token_key, token_key_refresh},
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
    fn decode(token: String, key: String) -> Result<TokenData<Self>, Error>
    where
        Self: Sized;
    fn from_token(token: String, ws: Option<bool>) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_refresh(token: String) -> Result<Self, Error>
    where
        Self: Sized;
}

#[derive(Serialize)]
pub struct Token {
    token: String,
}
pub trait ToToken {
    fn get_auth<T: HasSession>(id: i32, log_model: LogModel, role: Role, exp: i64) -> AuthPayload;
    fn to_token(&self, key: String) -> Result<Token, Rejection>;
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
            log_model: opt_log_model.unwrap_or(LogModel::default()),
            role: opt_role.unwrap_or(Role::default()),
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
    fn to_token(&self, key: String) -> Result<Token, Rejection> {
        let token = encode_model(&self, key).map_err(reject_error)?;
        Ok(Token { token })
    }
}

impl FromToken for AuthPayload {
    fn decode(token: String, key: String) -> Result<TokenData<Self>, Error>
    where
        Self: Sized,
    {
        decode::<AuthPayload>(
            &token,
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(bad_token)
    }
    fn from_token(bearer: String, ws: Option<bool>) -> Result<AuthPayload, Error> {
        let mut token = if ws.is_some() && ws.unwrap(){bearer.split("Bearer")} else{bearer.split(" ")};
        let decoded = AuthPayload::decode(
            String::from(token.nth(1).expect("Token Malformed")),
            token_key(),
        )?;
        Ok(decoded.claims)
    }
    fn from_refresh(bearer: String) -> Result<AuthPayload, Error> {
        let mut token = bearer.split_whitespace();
        let decoded = AuthPayload::decode(
            String::from(token.nth(1).expect("Token Malformed")),
            token_key_refresh(),
        )?;
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

fn encode_model<AuthPayload: Serialize>(model: &AuthPayload, key: String) -> Result<String, Error> {
    encode(
        &Header::new(Algorithm::HS256),
        model,
        &EncodingKey::from_secret(key.as_bytes()),
    )
    .map_err(bad_token)
}

#[derive(Serialize)]
pub struct LoginTokens{
    auth: Token,
    refresh: Token,
}
impl LoginTokens{
    pub fn new(auth: Token, refresh: Token)-> Self{
        LoginTokens { auth, refresh }
    }
}