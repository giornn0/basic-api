use validator::Validate;

use super::errors::Error;

pub fn validate<T>(body: T)->Result<T,Error> where T: Validate{
    body.validate().map_err(|e|{Error::InvalidBody(e)})?;
    Ok(body)
}