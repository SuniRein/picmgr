use crate::{api::pagination::PageSizeOutOfRange, image::ImageParseError};
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

    #[error(transparent)]
    PageSizeOutOfRange(#[from] PageSizeOutOfRange),

    #[error(transparent)]
    ImageParseError(#[from] ImageParseError),

    #[error("Internal Server Error")]
    #[from(sqlx::Error)]
    InternalServerError,
}

macro_rules! to_internal_server_error {
    ($($err_ty:ty,)*) => {
        $(
            impl From<$err_ty> for ApiError {
                fn from(_: $err_ty) -> Self {
                    ApiError::InternalServerError
                }
            }
        )*
    };
}

to_internal_server_error!(
    sqlx::Error,
    argon2::password_hash::Error,
    std::io::Error,
    axum::http::Error,
    image::ImageError,
);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            error: self.to_string(),
        });
        let status_code = match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::EmptyField(_) | ApiError::InvalidEmailFormat => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            ApiError::UsernameConflict | ApiError::EmailConflict => StatusCode::CONFLICT,
            ApiError::WrongCredentials | ApiError::InvalidToken => StatusCode::UNAUTHORIZED,
            ApiError::PermissionDenied => StatusCode::FORBIDDEN,
            ApiError::PageSizeOutOfRange(_) => StatusCode::BAD_REQUEST,
            ApiError::ImageParseError(e) => match e {
                ImageParseError::UnsupportedFormat => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                ImageParseError::ParseError => StatusCode::BAD_REQUEST,
            },
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };
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
