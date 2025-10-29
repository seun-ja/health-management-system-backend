use axum::{Json, extract::State};
use serde::Deserialize;

use crate::{AppState, db::personnel_db, error::ApiError};

pub async fn create_personnel(
    State(state): State<AppState>,
    Json(personnel): Json<PersonnelCreateRequest>,
) -> Result<(), ApiError> {
    personnel_db::insert_medical_personnel(&state.pool, personnel.try_into()?)
        .await
        .map_err(|e| e.into())
}

#[derive(Debug, Deserialize)]
pub struct PersonnelCreateRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub specialization: Option<String>,
    pub password: String,
}
