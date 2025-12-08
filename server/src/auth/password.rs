use argon2::{
    Argon2, PasswordHasher,
    password_hash::{self, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &str) -> password_hash::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}
