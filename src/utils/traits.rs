use serde::Serialize;
use warp::{
    reply::{Json, WithStatus},
    Rejection,
};

use crate::core::{errors::Error, response::Action};

use super::passwords::hash;

pub trait DefaultMsg {
    fn default_msg(&self) -> String {
        "Default message".to_owned()
    }
}
pub trait Send<'a, T: Serialize> {
    fn send(action: Action<'a, T>) -> Result<WithStatus<Json>, Rejection>;
}
pub trait HashedValue {
    fn hashed_value(unhashed: String) -> Result<String, Error> {
        if let Some(hash) = hash(&unhashed) {
            return Ok(hash);
        }
        Err(Error::InvalidRequest)
    }
}
