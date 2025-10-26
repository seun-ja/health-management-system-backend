use crate::schema::model::{Appointment, AppointmentStatus};
use chrono::NaiveDateTime;

pub async fn insert_appointment(
    pool: &sqlx::PgPool,
    patient_id: &str,
    doctor_id: &str,
    date: NaiveDateTime,
) -> Result<String, sqlx::Error> {
    let appointment_id = uuid::Uuid::now_v7().to_string();
    sqlx::query!(
        r#"
        INSERT INTO appointments (id, patient_id, doctor_id, date)
        VALUES ($1, $2, $3, $4)
        "#,
        appointment_id,
        patient_id,
        doctor_id,
        date,
    )
    .execute(pool)
    .await?;

    Ok(appointment_id)
}

pub async fn get_appointment_data(
    pool: &sqlx::PgPool,
    appointment_id: &str,
) -> Result<Appointment, sqlx::Error> {
    let appointment = sqlx::query_as!(
        Appointment,
        r#"
            SELECT
                id,
                patient_id,
                doctor_id,
                date,
                status as "status: AppointmentStatus"
            FROM appointments
            WHERE id = $1
            "#,
        appointment_id
    )
    .fetch_one(pool)
    .await?;

    Ok(appointment)
}

pub async fn update_appointment(
    pool: &sqlx::PgPool,
    appointment_id: &str,
    status: AppointmentStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE appointments
            SET status = $2
            WHERE id = $1
        "#,
        appointment_id,
        status as AppointmentStatus
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}
