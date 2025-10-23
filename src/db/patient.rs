use sqlx::Error;

use crate::schema::model::Patient;

pub async fn create_patient(pool: &sqlx::PgPool, patient: Patient) -> Result<Patient, Error> {
    sqlx::query!(
        "
        INSERT INTO patients (
            id,
            email,
            first_name,
            last_name,
            age,
            class,
            gender,
            alergies,
            emergency_contact,
            encrypted_password
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        patient.id,
        patient.email,
        patient.first_name,
        patient.last_name,
        patient.age,
        patient.class,
        patient.gender,
        patient.alergies.as_deref(),
        patient.emergency_contact,
        patient.encrypted_password
    )
    .execute(pool)
    .await?;

    Ok(patient)
}

pub async fn get_patient(pool: &sqlx::PgPool, email: String) -> Result<Patient, Error> {
    sqlx::query_as!(
        Patient,
        "
        SELECT
            id,
            email,
            first_name,
            last_name,
            age,
            class,
            gender,
            alergies,
            emergency_contact,
            encrypted_password,
            appointments
        FROM patients
        WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await
}

pub async fn update_patient_appointment(
    pool: &sqlx::PgPool,
    patient_id: &str,
    appointment_id: &str,
) -> Result<(), Error> {
    sqlx::query!(
        "
        UPDATE patients
        SET appointments = appointments || ARRAY[$2]
        WHERE id = $1
        ",
        patient_id,
        appointment_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
