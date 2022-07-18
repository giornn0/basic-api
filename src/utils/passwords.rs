use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

pub fn hash(value: &String) -> Option<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    match argon2.hash_password(value.as_bytes(), &salt) {
        Ok(hashed) => Some(hashed.to_string()),
        Err(_) => None,
    }
}
pub fn auth_hash(hashed: String, compare: String) -> bool {
    match PasswordHash::new(&hashed) {
        Ok(parsed) => Argon2::default()
            .verify_password(compare.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}
