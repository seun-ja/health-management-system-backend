use axum::{Json, extract::State};
use serde::Deserialize;

use crate::{
    AppState, db::personnel_db, error::ApiError, routes::encrypt_password,
    schema::model::MedicalPersonnel,
};

pub async fn create_personnel(
    State(state): State<AppState>,
    Json(personnel): Json<PersonnelCreateRequest>,
) -> Result<(), ApiError> {
    personnel_db::insert_medical_personnel(&state.pool, personnel.to_medical_personnel()?)
        .await
        .map_err(|e| e.into())
}

#[derive(Debug, Deserialize)]
pub struct PersonnelCreateRequest {
    email: String,
    first_name: String,
    last_name: Option<String>,
    specialization: Option<String>,
    password: String,
}

impl PersonnelCreateRequest {
    fn to_medical_personnel(&self) -> Result<MedicalPersonnel, ApiError> {
        Ok(MedicalPersonnel {
            id: uuid::Uuid::now_v7().to_string(),
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            specialization: self.specialization.clone(),
            encrypted_password: encrypt_password(&self.password)?,
        })
    }
}
