use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::domain::models::{Plan, User};
use crate::domain::ports::user_repository::UserRepository;
use crate::error::{AppError, AppResult};

/// PostgreSQL 기반 `UserRepository` 구현체.
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// DB row → User 매핑용 중간 구조체.
///
/// 런타임 `query_as`(매크로 아님)를 사용하므로 컴파일 시 DB 연결이 필요 없다.
#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
    plan: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        User {
            id: r.id,
            email: r.email,
            password_hash: r.password_hash,
            plan: Plan::from_db_str(&r.plan),
            created_at: r.created_at,
        }
    }
}

const SELECT_COLUMNS: &str = "id, email, password_hash, plan, created_at";

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(&self, email: &str, password_hash: &str) -> AppResult<User> {
        let row: UserRow = sqlx::query_as(&format!(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING {SELECT_COLUMNS}"
        ))
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            // unique violation (23505) → 이미 가입된 이메일
            if e.as_database_error().and_then(|d| d.code()).as_deref() == Some("23505") {
                AppError::BadRequest("이미 가입된 이메일입니다".into())
            } else {
                AppError::Database(e)
            }
        })?;

        Ok(row.into())
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let row: Option<UserRow> = sqlx::query_as(&format!(
            "SELECT {SELECT_COLUMNS} FROM users WHERE email = $1"
        ))
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let row: Option<UserRow> =
            sqlx::query_as(&format!("SELECT {SELECT_COLUMNS} FROM users WHERE id = $1"))
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map(Into::into))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use testcontainers_modules::{
        postgres::Postgres,
        testcontainers::{runners::AsyncRunner, ContainerAsync},
    };

    /// 격리된 Postgres 컨테이너 + 마이그레이션 적용된 풀을 띄운다.
    /// 컨테이너 핸들을 반환해 테스트 동안 살아있도록 한다.
    async fn setup() -> (ContainerAsync<Postgres>, PgPool) {
        let container = Postgres::default()
            .start()
            .await
            .expect("failed to start postgres container");
        let port = container
            .get_host_port_ipv4(5432)
            .await
            .expect("failed to get container port");
        let url = format!("postgres://postgres:postgres@localhost:{}/postgres", port);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .expect("failed to connect");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("failed to run migrations");

        (container, pool)
    }

    #[tokio::test]
    async fn create_and_find_roundtrip() {
        let (_container, pool) = setup().await;
        let repo = PgUserRepository::new(pool);

        let created = repo
            .create("alice@example.com", "hashed-pw")
            .await
            .expect("create failed");

        assert_eq!(created.email, "alice@example.com");
        assert_eq!(created.password_hash, "hashed-pw");
        assert_eq!(created.plan, Plan::Free);

        let found = repo
            .find_by_email("alice@example.com")
            .await
            .expect("find_by_email failed")
            .expect("user should exist");
        assert_eq!(found.id, created.id);
        assert_eq!(found.email, "alice@example.com");

        let by_id = repo
            .find_by_id(created.id)
            .await
            .expect("find_by_id failed")
            .expect("user should exist");
        assert_eq!(by_id.email, "alice@example.com");

        let missing = repo
            .find_by_email("nobody@example.com")
            .await
            .expect("find_by_email failed");
        assert!(missing.is_none());
    }

    #[tokio::test]
    async fn duplicate_email_returns_bad_request() {
        let (_container, pool) = setup().await;
        let repo = PgUserRepository::new(pool);

        repo.create("dup@example.com", "h1")
            .await
            .expect("first create failed");

        let err = repo
            .create("dup@example.com", "h2")
            .await
            .expect_err("second create should fail");

        assert!(matches!(err, AppError::BadRequest(_)));
    }
}
