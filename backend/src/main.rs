use anyhow::Result;
use coupang_review_ai_backend::{
    adapters::postgres::user_repo::PgUserRepository,
    application::auth_service::{AuthService, StandardAuthService},
    config::Config,
    http::{router::build_router, state::AppState},
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    let port = config.server_port;

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&db_pool).await?;

    let user_repo = Arc::new(PgUserRepository::new(db_pool.clone()));
    let auth_service: Arc<dyn AuthService> = Arc::new(StandardAuthService::new(
        user_repo,
        config.jwt_secret.clone(),
        config.jwt_expires_in,
    ));

    let state = AppState::new(db_pool, config, auth_service);
    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Server listening on port {}", port);
    axum::serve(listener, app).await?;

    Ok(())
}
