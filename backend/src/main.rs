use anyhow::Result;
use coupang_review_ai_backend::{
    adapters::claude::{ClaudeAiAnalyzer, MockAiAnalyzer},
    adapters::coupang::HttpCoupangCrawler,
    adapters::postgres::{analysis_repo::PgAnalysisRepository, user_repo::PgUserRepository},
    application::analysis_service::{AnalysisService, StandardAnalysisService},
    application::auth_service::{AuthService, StandardAuthService},
    application::user_service::{StandardUserService, UserService},
    config::Config,
    domain::ports::ai_analyzer::AiAnalyzer,
    domain::ports::analysis_repository::AnalysisRepository,
    domain::ports::crawler::CoupangCrawler,
    domain::ports::user_repository::UserRepository,
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

    let user_repo: Arc<dyn UserRepository> = Arc::new(PgUserRepository::new(db_pool.clone()));
    let analysis_repo: Arc<dyn AnalysisRepository> =
        Arc::new(PgAnalysisRepository::new(db_pool.clone()));

    let auth_service: Arc<dyn AuthService> = Arc::new(StandardAuthService::new(
        Arc::clone(&user_repo),
        config.jwt_secret.clone(),
        config.jwt_expires_in,
    ));

    let crawler: Arc<dyn CoupangCrawler> =
        Arc::new(HttpCoupangCrawler::new(reqwest::Client::new()));

    let analyzer: Arc<dyn AiAnalyzer> = match &config.claude_api_key {
        Some(key) => Arc::new(ClaudeAiAnalyzer::new(
            reqwest::Client::new(),
            key.clone(),
            config.claude_model.clone(),
        )),
        None => {
            tracing::warn!("CLAUDE_API_KEY 미설정 → MockAiAnalyzer 사용");
            Arc::new(MockAiAnalyzer::new())
        }
    };

    let analysis_service: Arc<dyn AnalysisService> = Arc::new(StandardAnalysisService::new(
        Arc::clone(&analysis_repo),
        crawler,
        analyzer,
    ));
    let user_service: Arc<dyn UserService> = Arc::new(StandardUserService::new(
        Arc::clone(&user_repo),
        Arc::clone(&analysis_repo),
    ));

    let state = AppState::new(
        db_pool,
        config,
        auth_service,
        analysis_service,
        user_service,
    );
    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Server listening on port {}", port);
    axum::serve(listener, app).await?;

    Ok(())
}
