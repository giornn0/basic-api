use dotenv::Error;
use http_api_problem::StatusCode;
use serde::Serialize;
use warp::{reply::{with_status, WithStatus, Json}, Rejection};

use crate::utils::traits::Send;


pub enum Action<'a,T: Serialize> {
    Created(T, &'a str),
    Updated(T, &'a str),
    Removed(&'a str),
    Indexed(T, &'a str),
    Finded(T, &'a str),
    Logged(T, &'a str),
    Calculated(T, &'a str),
}
impl<'a, T: Serialize> Action<'a,T>{
    pub fn send(self) ->Result<WithStatus<Json>, Rejection> {
        let (response, code) = Response::parse(self);
        let reply = warp::reply::json(&response);
        Ok(with_status(reply, code))
    }
}

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    data: Option<T>,
    message: String,
}
impl<T: Serialize> Response<T> {
    fn create(msg: &str, data: Option<T>) -> Response<T> {
        Response { data, message: msg.to_owned() }
    }

    fn parse(res: Action<T>) -> (Response<T>, StatusCode) {
        match res {
            Action::Created(data, msg) => {
                (Response::create(msg, Some(data)), StatusCode::CREATED)
            }
            Action::Logged(data, msg) => {
                (Response::create(msg, Some(data)), StatusCode::ACCEPTED)
            }
            Action::Updated(data, msg) => (Response::create(msg, Some(data)), StatusCode::OK),
            Action::Removed(msg) => (Response::create(msg, None), StatusCode::OK),
            Action::Indexed(data, msg) => (Response::create(msg, Some(data)), StatusCode::OK),
            Action::Finded(data, msg) => (Response::create(msg, Some(data)), StatusCode::OK),
            Action::Calculated(data, msg) => {
                (Response::create(msg, Some(data)), StatusCode::OK)
            }
        }
    }
}
impl<'a,T:Serialize> Send<'a, T> for Response<T>{
    fn send(action: Action<'a,T>)->Result<WithStatus<Json>,Rejection>{
        let (response, code) = Response::parse(action);
        let reply = warp::reply::json(&response);
        Ok(with_status(reply, code))
    }
}


