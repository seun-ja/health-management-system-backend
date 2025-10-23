pub mod appointment;
pub mod patient;
pub mod personnel;

pub async fn init_db(db_url: &str) -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    let pool = sqlx::PgPool::connect(db_url).await?;
    Ok(pool)
}
