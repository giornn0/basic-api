use crate::core::errors::Error;
use std::num::ParseIntError;
use warp::{reject::custom, Rejection};

pub fn port() -> Result<u16, ParseIntError> {
    std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))
}
pub fn token_key() -> String {
    std::env::var("JWT_KEY")
        .ok()
        .unwrap_or_else(|| "without_secret".to_owned())
}
pub fn token_key_refresh() -> String {
    std::env::var("JWT_KEY_REFRESH")
        .ok()
        .unwrap_or_else(|| "without_secret_refresh_".to_owned())
}
pub fn reject_error(error: Error) -> Rejection {
    custom(error)
}

