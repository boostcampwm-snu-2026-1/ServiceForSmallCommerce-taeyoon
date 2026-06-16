use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::{Analysis, AnalysisResult, AnalysisStatus};
use crate::error::AppResult;

/// 분석 작업 영속성 포트.
#[async_trait]
pub trait AnalysisRepository: Send + Sync {
    /// 새 분석 작업을 status=pending 으로 생성한다.
    /// `my_url` = 내 제품 URL, `urls` = 경쟁사 URL 목록.
    async fn create(
        &self,
        user_id: Uuid,
        my_url: &str,
        urls: &[String],
        review_limit: i32,
    ) -> AppResult<Analysis>;

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Analysis>>;

    /// 사용자별 분석 목록을 최신순으로 페이지네이션해 반환한다.
    /// 반환: (해당 페이지 항목, 전체 개수).
    async fn list_by_user(
        &self,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> AppResult<(Vec<Analysis>, i64)>;

    async fn update_status(&self, id: Uuid, status: AnalysisStatus) -> AppResult<()>;

    /// 결과를 저장하고 status=completed, completed_at=now 로 갱신한다.
    async fn set_result(&self, id: Uuid, result: &AnalysisResult) -> AppResult<()>;

    /// 에러를 저장하고 status=failed, completed_at=now 로 갱신한다.
    async fn set_error(&self, id: Uuid, error: &str) -> AppResult<()>;

    /// 이번 달(date_trunc month) 생성된 분석 건수.
    async fn count_this_month(&self, user_id: Uuid) -> AppResult<i64>;
}
