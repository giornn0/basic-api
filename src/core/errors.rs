use std::convert::Infallible;

use http_api_problem::StatusCode;
use serde::Serialize;
use thiserror::Error as ThisError;
use warp::{
    body::json,
    reject::{MissingHeader, Reject},
    reply::{self, with_status},
    Rejection, Reply,
};

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Invalid request parameters, the server could not handle this request")]
    InvalidRequest,
    #[error("Unauthorized action, need for authentication")]
    Unauthorized,
    #[error("Forbidden request, the user cant acces the request source")]
    Forbidden,
    #[error("Invalid token")]
    WrongToken,
    #[error("Error while querying the database")]
    WhileQuerying,
}
impl Reject for Error {}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejections(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Unavailable path".to_owned())
    } else if let Some(c_error) = err.find::<Error>() {
        match c_error {
            Error::InvalidRequest => (StatusCode::BAD_REQUEST, c_error.to_string()),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, c_error.to_string()),
            Error::Forbidden => (StatusCode::FORBIDDEN, c_error.to_string()),
            Error::WrongToken => (StatusCode::BAD_REQUEST, c_error.to_string()),
            Error::WhileQuerying => (StatusCode::NOT_IMPLEMENTED, c_error.to_string()),
        }
    } else if let Some(s_error) = err.find::<MissingHeader>() {
        (
            StatusCode::BAD_REQUEST,
            "The request is missing information".to_owned(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error while trying to fullfill this request".to_owned(),
        )
    };
    let response = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });
    println!("{:?}", err);
    Ok(with_status(response, code))
}
