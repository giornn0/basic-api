use std::num::ParseIntError;

pub fn port() -> Result<u16, ParseIntError> {
    std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))
}
pub fn token_key() -> String {
    std::env::var("JWT_KEY").ok().unwrap_or("without_secret".to_owned())
}
