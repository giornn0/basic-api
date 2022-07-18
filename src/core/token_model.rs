use std::sync::Arc;

use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error as JWTError, DecodingKey, EncodingKey, Header, Validation, TokenData,
    Algorithm
};
use serde::{Deserialize, Serialize};

use crate::{
    core::{errors::Error, server_model::Pool},
    utils::server::token_key,
};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum Role{
    Admin,
    User,
    Client
}

pub trait FromToken {
    fn decode(&self, token: String)->Result<TokenData<Self>, Error> where Self: Sized;
    fn from_token(token: String, db_pool: Arc<Pool>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Err(Error::WrongToken)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthPayload {
    id: i32,
    model: String,
    name: String,
    role: Role,
    exp: i64,
}
impl Default for AuthPayload {
    fn default() -> Self {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60))
            .expect("valid timestamp")
            .timestamp();
        AuthPayload {
            id: 4,
            model: "testing".to_owned(),
            name: "naming".to_owned(),
            role: Role::Client,
            exp: expiration,
        }
    }
}
impl FromToken for AuthPayload {
    fn decode(&self, token: String)->Result<TokenData<Self>, Error> where Self: Sized{
        decode::<AuthPayload>(
            &token,
            &DecodingKey::from_secret(token_key().as_bytes()),
            &Validation::new(Algorithm::HS256)).map_err(reject_error)
    }
    fn from_token(token: String, db_pool: Arc<Pool>) -> Result<AuthPayload, Error> {
        let test = AuthPayload::default();

        let token1 = encode_model(&test)?;
        let decoded = test.decode(token1)?;
        println!("{:?}", &decoded.claims);
        Ok(test)
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
