use validator::Validate;
use warp::{hyper::HeaderMap, reply::Response, Rejection, Reply};

use super::errors::Error;

pub fn validate<T>(body: T) -> Result<T, Error>
where
    T: Validate,
{
    body.validate().map_err(Error::InvalidBody)?;
    Ok(body)
}
pub fn send_with_headers(reply: impl Reply, headers: HeaderMap) -> Result<Response, Rejection> {
    let mut response = reply.into_response();
    let current_headers = response.headers_mut();
    current_headers.extend(headers);
    Ok(response)
}
