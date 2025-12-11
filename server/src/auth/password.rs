use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{self, PasswordHash, SaltString, rand_core::OsRng},
};
use tracing::{error, instrument};

#[instrument(skip(password))]
pub fn hash_password(password: &str) -> password_hash::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .inspect_err(|e| error!(error=?e, "password hashing failed"))?
        .to_string();
    Ok(password_hash)
}

#[instrument(skip(password, password_hash))]
pub fn verify_password(password: &str, password_hash: &str) -> password_hash::Result<bool> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(e) => {
            error!(error=?e, "password verification failed");
            Err(e)
        }
    }
}
