use async_trait::async_trait;

use crate::domain::models::Insights;
use crate::domain::ports::crawler::ProductReviews;
use crate::error::AppResult;

/// 리뷰 → 인사이트 분석 포트.
#[async_trait]
pub trait AiAnalyzer: Send + Sync {
    /// 여러 상품의 리뷰를 분석해 인사이트를 생성한다.
    async fn analyze(&self, products: &[ProductReviews]) -> AppResult<Insights>;
}
