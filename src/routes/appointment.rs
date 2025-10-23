use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use rand::Rng as _;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    db::{
        appointment::{get_appointment_data, insert_appointment},
        patient::update_patient_appointment,
        personnel::personnels,
    },
    error::ApiError,
    schema::model::Appointment,
};

pub async fn create_appointment(
    State(state): State<AppState>,
    Json(payload): Json<InitiateAppointment>,
) -> Result<AppointmentCreated, ApiError> {
    // TODO: validate JWT
    let personnels = personnels();

    let random_personnel_id = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..personnels.len())
    };

    let personnel = &personnels[random_personnel_id];

    //TODO: use pool transaction instead

    let appointment_id = insert_appointment(
        &state.pool,
        &payload.patient_id,
        &personnel.id.to_string(),
        chrono::NaiveDateTime::parse_from_str(&payload.date, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| ApiError::Other(e.to_string()))?,
    )
    .await?;

    update_patient_appointment(&state.pool, &payload.patient_id, &appointment_id).await?;

    Ok(AppointmentCreated {
        appointment_id,
        doctor_first_name: personnel.first_name.clone(),
        date: payload.date,
    })
}

#[derive(Debug, Deserialize)]
pub struct InitiateAppointment {
    patient_id: String,
    date: String,
}

#[derive(Serialize)]
pub struct AppointmentCreated {
    appointment_id: String,
    doctor_first_name: String,
    date: String,
}

impl IntoResponse for AppointmentCreated {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn get_appointment(
    State(state): State<AppState>,
    Path(appointment_id): Path<String>,
) -> Result<Appointment, ApiError> {
    Ok(get_appointment_data(&state.pool, &appointment_id).await?)
}

// async fn update_appointment(id: i32, status: AppointmentStatus) -> Result<(), ApiError> {
//     // TODO: implement appointment update logic
//     Ok(())
// }

// async fn delete_appointment(id: i32) -> Result<(), ApiError> {
//     // TODO: implement appointment deletion logic
//     Ok(())
// }
