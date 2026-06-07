use async_trait::async_trait;

use crate::domain::models::Review;
use crate::error::AppResult;

/// 단일 상품의 크롤링 결과.
#[derive(Debug, Clone)]
pub struct ProductReviews {
    pub url: String,
    pub product_name: String,
    pub reviews: Vec<Review>,
}

/// 쿠팡 리뷰 크롤러 포트.
#[async_trait]
pub trait CoupangCrawler: Send + Sync {
    /// 주어진 상품 URL 에서 최대 `limit` 개의 리뷰를 가져온다.
    async fn fetch_reviews(&self, url: &str, limit: u32) -> AppResult<ProductReviews>;
}
