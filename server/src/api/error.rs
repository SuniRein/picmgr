use crate::{api::pagination::PageSizeOutOfRange, image::parse::ImageParseError};
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

    #[error("Wrong Credentials")]
    WrongCredentials,

    #[error("Invalid Token")]
    InvalidToken,

    #[error("Permission Denied")]
    PermissionDenied,

    #[error("Internal Server Error")]
    ResponseBuildError,

    #[error(transparent)]
    PageSizeOutOfRange(#[from] PageSizeOutOfRange),

    #[error("Database Error")]
    Db(#[from] sqlx::Error),

    #[error("Argon2 Error")]
    Argon2,

    #[error(transparent)]
    ImageParseError(#[from] ImageParseError),

    #[error("Image Storage Error")]
    ImageStorageError(std::io::Error),
}

impl From<argon2::password_hash::Error> for ApiError {
    fn from(_: argon2::password_hash::Error) -> Self {
        ApiError::Argon2
    }
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
            ApiError::WrongCredentials | ApiError::InvalidToken => StatusCode::UNAUTHORIZED,
            ApiError::PermissionDenied => StatusCode::FORBIDDEN,
            ApiError::ResponseBuildError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::PageSizeOutOfRange(_) => StatusCode::BAD_REQUEST,
            ApiError::ImageParseError(ImageParseError::UnsupportedFormat) => {
                StatusCode::UNSUPPORTED_MEDIA_TYPE
            }
            ApiError::ImageParseError(_) => StatusCode::BAD_REQUEST,
            ApiError::ImageStorageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(ErrorResponse {
            error: self.to_string(),
        });
        (status_code, body).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid Token")]
    InvalidToken,
    #[error("Token Expired")]
    ExpiredToken,
    #[error("Administrative Privileges Required")]
    AdminRequired,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AuthError::InvalidToken | AuthError::ExpiredToken => StatusCode::UNAUTHORIZED,
            AuthError::AdminRequired => StatusCode::FORBIDDEN,
        };
        let body = Json(ErrorResponse {
            error: self.to_string(),
        });
        (status_code, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
