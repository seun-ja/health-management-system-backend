use std::sync::Arc;

pub mod db;
pub mod error;
pub mod routes;
pub mod schema;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub hmac_secret: Arc<String>,
}
