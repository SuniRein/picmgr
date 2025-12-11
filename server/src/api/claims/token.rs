use crate::auth::jwt::{Claims, encode_token};
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

#[derive(Serialize, utoipa::ToSchema)]
pub struct Token {
    access_token: AccessToken,
    refresh_token: RefreshToken,
}

impl Token {
    pub fn generate_user_token(user_id: i32) -> Self {
        let now = Utc::now();
        let access_token = AccessToken::new(now, user_id, false);
        let refresh_token = RefreshToken::new(now, user_id, false);

        Self {
            access_token,
            refresh_token,
        }
    }

    pub fn generate_admin_token() -> Self {
        let now = Utc::now();
        let access_token = AccessToken::new(now, 0, true);
        let refresh_token = RefreshToken::new(now, 0, true);

        Self {
            access_token,
            refresh_token,
        }
    }
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct AccessToken(pub String);

#[derive(Serialize, utoipa::ToSchema)]
pub struct RefreshToken(pub String);

const ACCESS_TOKEN_EXPIRY: Duration = Duration::minutes(15);
const REFRESH_TOKEN_EXPIRY: Duration = Duration::days(7);

impl AccessToken {
    pub(super) fn new(now: DateTime<Utc>, sub: i32, is_admin: bool) -> Self {
        let claims = Claims {
            sub,
            exp: (now + ACCESS_TOKEN_EXPIRY).timestamp() as usize,
            is_admin,
            token_use: "access".to_string(),
        };
        Self(encode_token(&claims))
    }
}

impl RefreshToken {
    fn new(now: DateTime<Utc>, sub: i32, is_admin: bool) -> Self {
        let claims = Claims {
            sub,
            exp: (now + REFRESH_TOKEN_EXPIRY).timestamp() as usize,
            is_admin,
            token_use: "refresh".to_string(),
        };
        Self(encode_token(&claims))
    }
}
