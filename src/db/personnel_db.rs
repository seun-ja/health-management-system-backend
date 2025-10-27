use crate::schema::model::MedicalPersonnel;

pub async fn medical_personnels(pool: &sqlx::PgPool) -> Result<Vec<MedicalPersonnel>, sqlx::Error> {
    sqlx::query_as!(
        MedicalPersonnel,
        r#"
            SELECT id, email, first_name, last_name, specialization, encrypted_password
            FROM medical_personnel
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn medical_personnel(
    pool: &sqlx::PgPool,
    id: &str,
) -> Result<MedicalPersonnel, sqlx::Error> {
    sqlx::query_as!(
        MedicalPersonnel,
        r#"
            SELECT id, email, first_name, last_name, specialization, encrypted_password
            FROM medical_personnel
            WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn insert_medical_personnel(
    pool: &sqlx::PgPool,
    personnel: MedicalPersonnel,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO medical_personnel (id, email, first_name, last_name, specialization, encrypted_password)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        personnel.id,
        personnel.email,
        personnel.first_name,
        personnel.last_name,
        personnel.specialization,
        personnel.encrypted_password
    )
    .execute(pool)
    .await
    .map(|_| ())
}
