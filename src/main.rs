use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
use health_mgt_system::{
    AppState,
    routes::{
        appointment::{create_appointment, get_appointment},
        health::ping_pong,
        login::post_login,
        signup::post_signup,
    },
};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = health_mgt_system::db::init_db(&db_url).await?;

    let app_state = AppState {
        pool: db_pool,
        hmac_secret: Arc::new(String::from("secret")),
    };

    let server: Router = Router::new()
        .route("/ping", get(ping_pong))
        .route("/login", post(post_login))
        .route("/signup", post(post_signup))
        .route("/appointment", post(create_appointment))
        .route("/appointment", get(get_appointment))
        .with_state(app_state)
        .layer(
            CorsLayer::new()
                .allow_headers(AllowHeaders::any())
                .allow_origin(AllowOrigin::any())
                .allow_methods(AllowMethods::any()),
        );

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
        .serve(server.into_make_service())
        .await?;

    Ok(())
}
