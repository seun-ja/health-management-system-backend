use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::{
    error::ApiError,
    routes::{encrypt_password, signup::SignupRequest},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<i32>,
    pub class: Option<String>,
    pub gender: Option<String>,
    pub alergies: Option<Vec<String>>,
    pub emergency_contact: Option<String>,
    pub encrypted_password: String,
    pub appointments: Option<Vec<String>>,
}

impl Patient {
    pub fn from_request(request: SignupRequest) -> Result<Self, ApiError> {
        Ok(Self {
            id: uuid::Uuid::now_v7().to_string(),
            email: request.email,
            first_name: Some(request.first_name),
            last_name: request.last_name,
            age: request.age,
            class: request.class,
            gender: request.gender,
            alergies: request.alergies,
            emergency_contact: request.emergency_contact,
            encrypted_password: encrypt_password(&request.password)?,
            appointments: Some(vec![]),
        })
    }
}

#[derive(Clone)]
pub struct MedicalPersonnel {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub specialization: Option<String>,
    pub encrypted_password: String,
    // TODO: Add availability
}

#[derive(Debug, Clone, Serialize)]
pub struct Appointment {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub date: NaiveDateTime,
    pub status: AppointmentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum AppointmentStatus {
    Pending,
    OnGoing,
    Cancelled,
    NoShow,
    Completed,
}

impl IntoResponse for Appointment {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
