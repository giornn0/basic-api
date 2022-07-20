use std::convert::Infallible;

use http_api_problem::StatusCode;
use serde::Serialize;
use thiserror::Error as ThisError;
use validator::ValidationErrors;
use warp::{
    body::json,
    reject::{MissingHeader, Reject, MethodNotAllowed},
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
    #[error("Error while pooling the database")]
    FailedPool,
    #[error("Error formatting the body")]
    InvalidBody(ValidationErrors),
    #[error("{0}")]
    Redaction(StatusCode,String),
}
impl Reject for Error {}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
    details: Option<String>
}

pub async fn handle_rejections(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message,details) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Unavailable path".to_owned(),None)
    } else if let Some(c_error) = err.find::<Error>() {
        match c_error {
            Error::InvalidRequest => (StatusCode::BAD_REQUEST, c_error.to_string(),None),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, c_error.to_string(),None),
            Error::Forbidden => (StatusCode::FORBIDDEN, c_error.to_string(),None),
            Error::WrongToken => (StatusCode::BAD_REQUEST, c_error.to_string(),None),
            Error::WhileQuerying => (StatusCode::NOT_IMPLEMENTED, c_error.to_string(),None),
            Error::FailedPool => (StatusCode::SERVICE_UNAVAILABLE, c_error.to_string(),None),
            Error::Redaction(status,error)=> (status.to_owned(), error.to_owned(),None),
            Error::InvalidBody(errors)=> (StatusCode::BAD_REQUEST, c_error.to_string(),Some(format!("Errors => {}",errors))),
        }
    } else if let Some(_s_error) = err.find::<MissingHeader>() {
        (
            StatusCode::BAD_REQUEST,
            "The request is missing information".to_owned(),
            None
        )
    } else if let Some(_s_error) = err.find::<MethodNotAllowed>() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Use of wrong method".to_owned(),
            None
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error while trying to fullfill this request".to_owned(),
            None
        )
    };
    let response = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
        details
    });
    println!("{:?}", err);
    Ok(with_status(response, code))
}
