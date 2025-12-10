use super::refresh::RefreshClaims;
use crate::auth::jwt::{Claims, encode_token};
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    access_token: String,
    refresh_token: String,
}

const ACCESS_TOKEN_EXPIRY: Duration = Duration::minutes(15);
const REFRESH_TOKEN_EXPIRY: Duration = Duration::days(7);

impl Token {
    pub fn generate_user_token(user_id: i32) -> Self {
        let access_claims = Claims {
            sub: user_id,
            exp: (chrono::Utc::now() + ACCESS_TOKEN_EXPIRY).timestamp() as usize,
            is_admin: false,
            token_use: "access".to_string(),
        };
        let refresh_claims = Claims {
            sub: user_id,
            exp: (chrono::Utc::now() + REFRESH_TOKEN_EXPIRY).timestamp() as usize,
            is_admin: false,
            token_use: "refresh".to_string(),
        };

        let access_token = encode_token(&access_claims);
        let refresh_token = encode_token(&refresh_claims);

        Self {
            access_token,
            refresh_token,
        }
    }

    pub fn generate_admin_token() -> Self {
        let access_claims = Claims {
            sub: 0,
            exp: (chrono::Utc::now() + ACCESS_TOKEN_EXPIRY).timestamp() as usize,
            is_admin: true,
            token_use: "access".to_string(),
        };
        let refresh_claims = Claims {
            sub: 0,
            exp: (chrono::Utc::now() + REFRESH_TOKEN_EXPIRY).timestamp() as usize,
            is_admin: true,
            token_use: "refresh".to_string(),
        };

        let access_token = encode_token(&access_claims);
        let refresh_token = encode_token(&refresh_claims);

        Self {
            access_token,
            refresh_token,
        }
    }

    pub fn refresh(RefreshClaims(claims): &RefreshClaims) -> String {
        let new_access_claims = Claims {
            sub: claims.sub,
            exp: (chrono::Utc::now() + ACCESS_TOKEN_EXPIRY).timestamp() as usize,
            is_admin: claims.is_admin,
            token_use: "access".to_string(),
        };
        encode_token(&new_access_claims)
    }
}
