use http_api_problem::StatusCode;
use serde::Serialize;
use warp::{reply::{with_status, WithStatus, Json}, Rejection};

use crate::utils::traits::Send;


pub enum Action<'a,T: Serialize> {
    Created(T, &'a str),
    Updated(T, &'a str),
    Removed(&'a str),
    Indexed(T),
    Finded(T, &'a str),
    Logged(T, &'a str),
    Refreshed(T, &'a str),
    Calculated(T, &'a str),
}

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    data: Option<T>,
    message: Option<String>,
}
impl<T: Serialize> Default for Response<T>{
    fn default()->Self{
        Response{data: None, message: Some("Welcome to giornn0's API".to_owned())}
    }
}
impl<T: Serialize> Response<T> {
    fn create(msg: Option<&str>, data: Option<T>) -> Response<T> {
        let message = match msg {
            Some(message)=>Some(message.to_owned()),
            None=> None
        };
        Response { data, message }
    }

    fn parse(res: Action<T>) -> (Response<T>, StatusCode) {
        match res {
            Action::Created(data, msg) => {
                (Response::create(Some(msg), Some(data)), StatusCode::CREATED)
            }
            Action::Logged(data, msg) => {
                (Response::create(Some(msg), Some(data)), StatusCode::ACCEPTED)
            }
            Action::Updated(data, msg) => (Response::create(Some(msg), Some(data)), StatusCode::OK),
            Action::Refreshed(data, msg) => (Response::create(Some(msg), Some(data)), StatusCode::OK),
            Action::Removed(msg) => (Response::create(Some(msg), None), StatusCode::OK),
            Action::Indexed(data) => (Response::create(None, Some(data)), StatusCode::OK),
            Action::Finded(data, msg) => (Response::create(Some(msg), Some(data)), StatusCode::OK),
            Action::Calculated(data, msg) => {
                (Response::create(Some(msg), Some(data)), StatusCode::OK)
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


