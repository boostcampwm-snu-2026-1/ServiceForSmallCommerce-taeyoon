use coupang_review_ai_backend::adapters::claude::MockAiAnalyzer;
use coupang_review_ai_backend::adapters::coupang::MockCoupangCrawler;
use coupang_review_ai_backend::adapters::postgres::analysis_repo::PgAnalysisRepository;
use coupang_review_ai_backend::adapters::postgres::user_repo::PgUserRepository;
use coupang_review_ai_backend::application::analysis_service::{
    AnalysisService, StandardAnalysisService,
};
use coupang_review_ai_backend::application::auth_service::{AuthService, StandardAuthService};
use coupang_review_ai_backend::application::user_service::{StandardUserService, UserService};
use coupang_review_ai_backend::config::Config;
use coupang_review_ai_backend::domain::ports::ai_analyzer::AiAnalyzer;
use coupang_review_ai_backend::domain::ports::analysis_repository::AnalysisRepository;
use coupang_review_ai_backend::domain::ports::crawler::CoupangCrawler;
use coupang_review_ai_backend::domain::ports::user_repository::UserRepository;
use coupang_review_ai_backend::http::{router::build_router, state::AppState};
use sqlx::{postgres::PgPoolOptions, Connection, Executor, PgConnection, PgPool};
use std::net::SocketAddr;
use std::sync::Arc;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{runners::AsyncRunner, ContainerAsync},
};
use tokio::sync::OnceCell;
use uuid::Uuid;

/// 프로세스당 컨테이너를 공유한다.
/// 핸들을 OnceCell에 보관하여 프로세스 종료 시까지 컨테이너가 살아있도록 한다.
/// Ryuk(testcontainers 자동정리 데몬)가 프로세스 종료 후 컨테이너를 제거한다.
static SHARED_CONTAINER: OnceCell<(ContainerAsync<Postgres>, u16)> = OnceCell::const_new();

async fn shared_postgres_port() -> u16 {
    SHARED_CONTAINER
        .get_or_init(|| async {
            let container: ContainerAsync<Postgres> = Postgres::default()
                .start()
                .await
                .expect("Failed to start PostgreSQL container");
            let port = container
                .get_host_port_ipv4(5432)
                .await
                .expect("Failed to get container port");
            (container, port)
        })
        .await
        .1
}

/// 통합 테스트 하네스.
/// 각 인스턴스는 격리된 test_<uuid> DB + 랜덤 포트 Axum 서버를 스핀업한다.
pub struct TestApp {
    pub address: String,
    #[allow(dead_code)]
    pub db_pool: PgPool,
    db_name: String,
    admin_db_url: String,
}

impl TestApp {
    pub async fn spawn() -> Self {
        let port = shared_postgres_port().await;
        let admin_db_url = format!("postgres://postgres:postgres@localhost:{}/postgres", port);
        let db_name = format!("test_{}", Uuid::new_v4().to_string().replace('-', "_"));

        create_test_database(&admin_db_url, &db_name).await;

        let test_db_url = format!(
            "postgres://postgres:postgres@localhost:{}/{}",
            port, db_name
        );
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&test_db_url)
            .await
            .expect("Failed to connect to test database");

        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Failed to run migrations");

        let config = Config {
            database_url: test_db_url,
            jwt_secret: "test-secret-key".to_string(),
            jwt_expires_in: 86400,
            server_port: 0,
            claude_api_key: None,
            claude_model: "claude-sonnet-4-6".to_string(),
        };

        let user_repo: Arc<dyn UserRepository> = Arc::new(PgUserRepository::new(db_pool.clone()));
        let analysis_repo: Arc<dyn AnalysisRepository> =
            Arc::new(PgAnalysisRepository::new(db_pool.clone()));

        let auth_service: Arc<dyn AuthService> = Arc::new(StandardAuthService::new(
            Arc::clone(&user_repo),
            config.jwt_secret.clone(),
            config.jwt_expires_in,
        ));

        // 결정론·무네트워크: Mock 크롤러/분석기 주입.
        let crawler: Arc<dyn CoupangCrawler> = Arc::new(MockCoupangCrawler::new());
        let analyzer: Arc<dyn AiAnalyzer> = Arc::new(MockAiAnalyzer::new());
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
            db_pool.clone(),
            config,
            auth_service,
            analysis_service,
            user_service,
        );
        let app = build_router(state);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind");
        let addr: SocketAddr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        TestApp {
            address: format!("http://{}", addr),
            db_pool,
            db_name,
            admin_db_url,
        }
    }

    /// register 엔드포인트 호출 헬퍼. 응답을 그대로 반환한다.
    #[allow(dead_code)]
    pub async fn register(&self, email: &str, password: &str) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/api/v1/auth/register", self.address))
            .json(&serde_json::json!({ "email": email, "password": password }))
            .send()
            .await
            .expect("register request failed")
    }

    /// register 후 발급된 JWT 토큰을 반환한다.
    #[allow(dead_code)]
    pub async fn register_and_token(&self, email: &str, password: &str) -> String {
        let res = self.register(email, password).await;
        assert_eq!(res.status(), 201, "register should succeed");
        let body: serde_json::Value = res.json().await.unwrap();
        body["token"]
            .as_str()
            .expect("token should exist")
            .to_string()
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        let admin_url = self.admin_db_url.clone();
        let db_name = self.db_name.clone();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build cleanup runtime");
            rt.block_on(drop_test_database(&admin_url, &db_name));
        });
    }
}

async fn create_test_database(admin_url: &str, db_name: &str) {
    let mut conn = PgConnection::connect(admin_url)
        .await
        .expect("Failed to connect to admin DB");
    conn.execute(format!("CREATE DATABASE \"{}\"", db_name).as_str())
        .await
        .expect("Failed to create test database");
}

async fn drop_test_database(admin_url: &str, db_name: &str) {
    let mut conn = match PgConnection::connect(admin_url).await {
        Ok(c) => c,
        Err(_) => return,
    };
    // 활성 연결 강제 종료 후 DB 삭제
    let _ = conn
        .execute(
            format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity \
                 WHERE datname = '{}' AND pid <> pg_backend_pid()",
                db_name
            )
            .as_str(),
        )
        .await;
    let _ = conn
        .execute(format!("DROP DATABASE IF EXISTS \"{}\"", db_name).as_str())
        .await;
}
