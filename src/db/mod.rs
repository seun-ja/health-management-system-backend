pub mod appointment_db;
pub mod patient_db;
pub mod personnel_db;

pub async fn init_db(db_url: &str) -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    let pool = sqlx::PgPool::connect(db_url).await?;
    Ok(pool)
}
