use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::{
    error::ApiError,
    routes::{encrypt_password, personnel::PersonnelCreateRequest, signup::PatientSignupRequest},
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

impl TryFrom<PatientSignupRequest> for Patient {
    type Error = ApiError;

    fn try_from(value: PatientSignupRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: uuid::Uuid::now_v7().to_string(),
            email: value.email,
            first_name: Some(value.first_name),
            last_name: value.last_name,
            age: value.age,
            class: value.class,
            gender: value.gender,
            alergies: value.alergies,
            emergency_contact: value.emergency_contact,
            encrypted_password: encrypt_password(&value.password)?,
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

impl TryFrom<PersonnelCreateRequest> for MedicalPersonnel {
    type Error = ApiError;

    fn try_from(value: PersonnelCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: uuid::Uuid::now_v7().to_string(),
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            specialization: value.specialization,
            encrypted_password: encrypt_password(&value.password)?,
        })
    }
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
