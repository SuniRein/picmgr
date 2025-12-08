use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Resource Not Found")]
    NotFound,

    #[error("Empty Field: {0}")]
    EmptyField(String),

    #[error("Invalid Email Format")]
    InvalidEmailFormat,

    #[error("Username Already Exists")]
    UsernameConflict,
    #[error("Email Already Exists")]
    EmailConflict,

    #[error("Database Error")]
    Db(#[from] sqlx::Error),

    #[error("Argon2 Error")]
    Argon2,
}

impl From<argon2::password_hash::Error> for ApiError {
    fn from(_: argon2::password_hash::Error) -> Self {
        ApiError::Argon2
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Db(_) | ApiError::Argon2 => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::EmptyField(_) | ApiError::InvalidEmailFormat => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            ApiError::UsernameConflict | ApiError::EmailConflict => StatusCode::CONFLICT,
        };
        let body = Json(ErrorResponse {
            error: self.to_string(),
        });
        (status_code, body).into_response()
    }
}
