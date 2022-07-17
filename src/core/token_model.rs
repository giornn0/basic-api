use std::sync::Arc;

use serde::{Serialize, Deserialize};

use crate::core::{server_model::Pool,errors::Error};


pub trait FromToken{
    fn from_token(  token: String, db_pool: Arc<Pool>)->Result<Self, Error> where Self: Sized{
        Err(Error::WrongToken)
    }
}


#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct AuthPayload{
    id: i32,
    model: String,
    expiration: String,
}
impl Default for AuthPayload{
    fn default() -> Self {
        AuthPayload { id: 4, model: "testing".to_owned(), expiration: "12/05/2022 15:50".to_owned() }
    }
}
impl FromToken for AuthPayload{
    fn from_token(token: String, db_pool: Arc<Pool>)-> Result<AuthPayload, Error> {
       Ok(AuthPayload::default())
    }
}
