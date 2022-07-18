use serde::Serialize;
use warp::{reply::{WithStatus, Json}, Rejection};

use crate::core::response::Action;

pub trait DefaultMsg{
    fn default_msg(&self)->String{"Default message".to_owned()}
}
pub trait Send<'a, T: Serialize>{
    fn send(action: Action<'a, T>)->Result<WithStatus<Json>,Rejection>{
        Err(warp::reject())
    }
}