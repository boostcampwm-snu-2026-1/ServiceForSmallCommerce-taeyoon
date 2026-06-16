use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::domain::models::{Analysis, AnalysisResult, AnalysisStatus};
use crate::domain::ports::analysis_repository::AnalysisRepository;
use crate::error::{AppError, AppResult};

/// PostgreSQL 기반 `AnalysisRepository` 구현체.
pub struct PgAnalysisRepository {
    pool: PgPool,
}

impl PgAnalysisRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// DB row → Analysis 매핑용 중간 구조체.
///
/// 런타임 `query_as`(매크로 아님)를 사용하므로 컴파일 시 DB 연결이 필요 없다.
/// `result` 는 JSONB 컬럼을 `serde_json::Value` 로 받아 별도 변환한다.
#[derive(FromRow)]
struct AnalysisRow {
    id: Uuid,
    user_id: Uuid,
    my_url: Option<String>,
    urls: Vec<String>,
    review_limit: i32,
    status: String,
    result: Option<serde_json::Value>,
    error: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl AnalysisRow {
    fn into_domain(self) -> AppResult<Analysis> {
        let result = match self.result {
            Some(v) => Some(
                serde_json::from_value::<AnalysisResult>(v)
                    .map_err(|e| AppError::Internal(e.into()))?,
            ),
            None => None,
        };
        Ok(Analysis {
            id: self.id,
            user_id: self.user_id,
            my_url: self.my_url,
            urls: self.urls,
            review_limit: self.review_limit,
            status: AnalysisStatus::from_db_str(&self.status),
            result,
            error: self.error,
            created_at: self.created_at,
            completed_at: self.completed_at,
        })
    }
}

const SELECT_COLUMNS: &str =
    "id, user_id, my_url, urls, review_limit, status, result, error, created_at, completed_at";

#[async_trait]
impl AnalysisRepository for PgAnalysisRepository {
    async fn create(
        &self,
        user_id: Uuid,
        my_url: &str,
        urls: &[String],
        review_limit: i32,
    ) -> AppResult<Analysis> {
        let row: AnalysisRow = sqlx::query_as(&format!(
            "INSERT INTO analyses (user_id, my_url, urls, review_limit) \
             VALUES ($1, $2, $3, $4) RETURNING {SELECT_COLUMNS}"
        ))
        .bind(user_id)
        .bind(my_url)
        .bind(urls)
        .bind(review_limit)
        .fetch_one(&self.pool)
        .await?;

        row.into_domain()
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Analysis>> {
        let row: Option<AnalysisRow> = sqlx::query_as(&format!(
            "SELECT {SELECT_COLUMNS} FROM analyses WHERE id = $1"
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(AnalysisRow::into_domain).transpose()
    }

    async fn list_by_user(
        &self,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> AppResult<(Vec<Analysis>, i64)> {
        let offset = (page - 1).max(0) * per_page;

        let rows: Vec<AnalysisRow> = sqlx::query_as(&format!(
            "SELECT {SELECT_COLUMNS} FROM analyses WHERE user_id = $1 \
             ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ))
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM analyses WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        let items = rows
            .into_iter()
            .map(AnalysisRow::into_domain)
            .collect::<AppResult<Vec<_>>>()?;

        Ok((items, total.0))
    }

    async fn update_status(&self, id: Uuid, status: AnalysisStatus) -> AppResult<()> {
        sqlx::query("UPDATE analyses SET status = $1 WHERE id = $2")
            .bind(status.as_str())
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn set_result(&self, id: Uuid, result: &AnalysisResult) -> AppResult<()> {
        let value = serde_json::to_value(result).map_err(|e| AppError::Internal(e.into()))?;
        sqlx::query(
            "UPDATE analyses SET status = 'completed', result = $1, completed_at = now() \
             WHERE id = $2",
        )
        .bind(value)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn set_error(&self, id: Uuid, error: &str) -> AppResult<()> {
        sqlx::query(
            "UPDATE analyses SET status = 'failed', error = $1, completed_at = now() \
             WHERE id = $2",
        )
        .bind(error)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn count_this_month(&self, user_id: Uuid) -> AppResult<i64> {
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM analyses \
             WHERE user_id = $1 AND created_at >= date_trunc('month', now())",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(total.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{
        AnalysisResult, CompetitorWeakness, Complaint, ImprovementPoint, Insights, Positive,
        ProductSummary,
    };
    use sqlx::postgres::PgPoolOptions;
    use std::collections::HashMap;
    use testcontainers_modules::{
        postgres::Postgres,
        testcontainers::{runners::AsyncRunner, ContainerAsync},
    };

    /// 격리된 Postgres 컨테이너 + 마이그레이션 적용된 풀을 띄운다.
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

    /// FK 충족용 user 행을 삽입하고 id 를 반환한다.
    async fn insert_user(pool: &PgPool) -> Uuid {
        let row: (Uuid,) =
            sqlx::query_as("INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id")
                .bind("analysis-owner@example.com")
                .bind("hashed-pw")
                .fetch_one(pool)
                .await
                .expect("insert user failed");
        row.0
    }

    fn sample_result() -> AnalysisResult {
        let mut dist = HashMap::new();
        dist.insert("5".to_string(), 52);
        dist.insert("4".to_string(), 30);
        AnalysisResult {
            products: vec![ProductSummary {
                url: "https://www.coupang.com/vp/products/123".to_string(),
                product_name: "테스트 상품".to_string(),
                total_reviews: 100,
                avg_rating: 4.2,
                rating_distribution: dist,
                is_mine: true,
            }],
            insights: Insights {
                top_complaints: vec![Complaint {
                    text: "배송 포장이 허술하다".to_string(),
                    count: 37,
                    severity: "high".to_string(),
                }],
                top_positives: vec![Positive {
                    text: "가성비가 좋다".to_string(),
                    count: 52,
                }],
                improvement_points: vec![ImprovementPoint {
                    rank: 1,
                    title: "포장 강화".to_string(),
                    detail: "완충재 추가".to_string(),
                }],
                competitor_weaknesses: vec![CompetitorWeakness {
                    title: "AS 응답 느림".to_string(),
                    opportunity: "빠른 CS 강조".to_string(),
                }],
                purchase_drivers: vec!["저렴한 가격".to_string()],
                comparison_summary: Some("내 제품이 경쟁사보다 우수합니다.".to_string()),
            },
        }
    }

    #[tokio::test]
    async fn analysis_lifecycle_roundtrip() {
        let (_container, pool) = setup().await;
        let user_id = insert_user(&pool).await;
        let repo = PgAnalysisRepository::new(pool);

        let my_url = "https://www.coupang.com/vp/products/0".to_string();
        let urls = vec![
            "https://www.coupang.com/vp/products/1".to_string(),
            "https://www.coupang.com/vp/products/2".to_string(),
        ];

        // create → pending
        let created = repo
            .create(user_id, &my_url, &urls, 100)
            .await
            .expect("create failed");
        assert_eq!(created.user_id, user_id);
        assert_eq!(created.my_url.as_deref(), Some(my_url.as_str()));
        assert_eq!(created.urls, urls);
        assert_eq!(created.review_limit, 100);
        assert_eq!(created.status, AnalysisStatus::Pending);
        assert!(created.result.is_none());

        // find_by_id → pending
        let found = repo
            .find_by_id(created.id)
            .await
            .expect("find_by_id failed")
            .expect("should exist");
        assert_eq!(found.status, AnalysisStatus::Pending);
        assert_eq!(found.my_url.as_deref(), Some(my_url.as_str()));

        // update_status → crawling
        repo.update_status(created.id, AnalysisStatus::Crawling)
            .await
            .expect("update_status failed");
        let crawling = repo
            .find_by_id(created.id)
            .await
            .unwrap()
            .expect("should exist");
        assert_eq!(crawling.status, AnalysisStatus::Crawling);

        // set_result → completed + result 일치
        let result = sample_result();
        repo.set_result(created.id, &result)
            .await
            .expect("set_result failed");
        let completed = repo
            .find_by_id(created.id)
            .await
            .unwrap()
            .expect("should exist");
        assert_eq!(completed.status, AnalysisStatus::Completed);
        assert!(completed.completed_at.is_some());
        let got = completed.result.expect("result should exist");
        assert_eq!(got.products.len(), 1);
        assert_eq!(got.products[0].product_name, "테스트 상품");
        assert_eq!(got.products[0].avg_rating, 4.2);
        assert_eq!(got.insights.top_complaints[0].text, "배송 포장이 허술하다");
        assert_eq!(got.insights.top_complaints[0].count, 37);
        assert_eq!(got.products[0].rating_distribution.get("5"), Some(&52));

        // list_by_user → 1건, total=1
        let (items, total) = repo
            .list_by_user(user_id, 1, 20)
            .await
            .expect("list_by_user failed");
        assert_eq!(items.len(), 1);
        assert_eq!(total, 1);
        assert_eq!(items[0].id, created.id);

        // count_this_month → 1
        let count = repo
            .count_this_month(user_id)
            .await
            .expect("count_this_month failed");
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn set_error_marks_failed() {
        let (_container, pool) = setup().await;
        let user_id = insert_user(&pool).await;
        let repo = PgAnalysisRepository::new(pool);

        let my_url = "https://www.coupang.com/vp/products/8".to_string();
        let urls = vec!["https://www.coupang.com/vp/products/9".to_string()];
        let created = repo
            .create(user_id, &my_url, &urls, 50)
            .await
            .expect("create failed");

        repo.set_error(created.id, "크롤링 실패")
            .await
            .expect("set_error failed");

        let failed = repo
            .find_by_id(created.id)
            .await
            .unwrap()
            .expect("should exist");
        assert_eq!(failed.status, AnalysisStatus::Failed);
        assert_eq!(failed.error.as_deref(), Some("크롤링 실패"));
        assert!(failed.completed_at.is_some());
    }

    #[tokio::test]
    async fn list_by_user_empty_returns_zero() {
        let (_container, pool) = setup().await;
        let user_id = insert_user(&pool).await;
        let repo = PgAnalysisRepository::new(pool);

        let (items, total) = repo.list_by_user(user_id, 1, 20).await.unwrap();
        assert!(items.is_empty());
        assert_eq!(total, 0);
    }
}
