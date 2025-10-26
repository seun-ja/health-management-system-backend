use std::borrow::Cow;

use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServer,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid JWT credentials: {0}")]
    InvalidJWTCredentials(String),
    #[error("Wrong TimeStamp")]
    WrongTimeStamp,
    #[error("Invalid Credentials")]
    InvalidCredentials,
    #[error("Invalid Password Hash: {0}")]
    InvalidPasswordHash(String),
    #[error("Failed Hashing Password: {0}")]
    FailedHashingPassword(argon2::password_hash::Error),
    #[error("Error: {0}")]
    Other(String),
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalServer
            | ApiError::WrongTimeStamp
            | ApiError::FailedHashingPassword(_)
            | ApiError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) | ApiError::InvalidPasswordHash(_) | ApiError::NotFound(_) => {
                StatusCode::BAD_REQUEST
            }
            ApiError::InvalidJWTCredentials(_) | ApiError::InvalidCredentials => {
                StatusCode::UNAUTHORIZED
            }
            ApiError::DatabaseError(e) if check_unique_violation(e) => StatusCode::CONFLICT,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

fn check_unique_violation(error: &sqlx::Error) -> bool {
    match error {
        sqlx::Error::Database(e) => e.code() == Some(Cow::from("23505")),
        _ => false,
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // TODO: Log internal server error don't parse error message to user
        (self.status_code(), Json(self.to_string())).into_response()
    }
}
