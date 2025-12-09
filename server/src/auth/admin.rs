use std::env;
use std::sync::LazyLock;

static ADMIN_USER: LazyLock<String> =
    LazyLock::new(|| env::var("ADMIN_USER").unwrap_or_else(|_| "admin".to_string()));

static ADMIN_PASSWORD: LazyLock<String> =
    LazyLock::new(|| env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string()));

pub fn verify_admin(username: &str, password: &str) -> bool {
    username == ADMIN_USER.as_str() && password == ADMIN_PASSWORD.as_str()
}
