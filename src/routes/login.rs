use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{AppState, db::patient::get_patient, error::ApiError, routes::verify_password};

pub async fn post_login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<LoggedUser, ApiError> {
    let patient = get_patient(&state.pool, request.email)
        .await
        .map_err(ApiError::from)?;

    verify_password(
        &patient.encrypted_password,
        &request.password,
        &state.hmac_secret,
        &patient.id,
        &patient.email,
    )
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoggedUser {
    pub token: String,
    // TODO: include more user info
}

impl IntoResponse for LoggedUser {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
