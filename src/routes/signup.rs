use axum::{Json, extract::State};
use serde::Deserialize;

use crate::{AppState, db::patient::create_patient, error::ApiError, schema::model::Patient};

pub async fn post_signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<Json<String>, ApiError> {
    let patient = create_patient(&state.pool, Patient::from_request(request)?)
        .await
        .map_err(ApiError::from)?;

    // TODO: Implement email verification logic
    Ok(Json(patient.id.to_string()))
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: Option<i32>,
    pub class: Option<String>,
    pub gender: Option<String>,
    pub alergies: Option<Vec<String>>,
    pub emergency_contact: Option<String>,
}
