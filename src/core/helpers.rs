use validator::Validate;
use warp::{hyper::HeaderMap, Rejection, reply::Response, Reply};

use super::errors::Error;

pub fn validate<T>(body: T)->Result<T,Error> where T: Validate{
    body.validate().map_err(|e|{Error::InvalidBody(e)})?;
    Ok(body)
}
pub fn send_with_headers(reply: impl Reply, headers: HeaderMap)->Result<Response, Rejection>{
    let mut  response = reply.into_response();
    let current_headers = response.headers_mut();
    current_headers.extend(headers);
    Ok(response)
}
// pub fn set_headers(mut actual_headers: &HeaderMap, headers: HeaderMap)->(){
//     actual_headers.extend(headers)
// }