use serde::Serialize;
use warp::{reply::{WithStatus,Json, Response}, Rejection, reject::custom, Reply, hyper::HeaderMap};

use crate::core::{response::Action, errors::Error};

use super::passwords::hash;

pub trait DefaultMsg{
    fn default_msg(&self)->String{"Default message".to_owned()}
}
pub trait Send<'a, T: Serialize>{
    fn send(action: Action<'a, T>)->Result<WithStatus<Json>,Rejection>;
}
pub trait HashedValue{
    fn hashed_value(unhashed: String)->Result<String, Rejection>{
        if let Some(hash)= hash(&unhashed){
            return Ok(hash)
        }
        Err(custom(Error::InvalidRequest))
    }
}