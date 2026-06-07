use async_trait::async_trait;

use crate::domain::models::Review;
use crate::domain::ports::crawler::{CoupangCrawler, ProductReviews};
use crate::error::{AppError, AppResult};

/// 테스트/개발용 결정론적 크롤러.
///
/// 네트워크 호출 없이 url 기반으로 동일 입력 → 동일 출력의 픽스처를 반환한다.
#[derive(Debug, Default, Clone)]
pub struct MockCoupangCrawler;

impl MockCoupangCrawler {
    pub fn new() -> Self {
        Self
    }

    /// url 에서 결정론적으로 상품명을 유도한다.
    fn product_name_for(url: &str) -> String {
        match HttpCoupangCrawler::extract_product_id(url) {
            Some(id) => format!("쿠팡 상품 {id}"),
            None => "쿠팡 상품".to_string(),
        }
    }

    /// 긍/부정이 섞인 고정 한국어 리뷰 목록.
    fn fixture_reviews() -> Vec<Review> {
        vec![
            Review {
                text: "가성비가 좋다".to_string(),
                rating: 5,
            },
            Review {
                text: "배송이 빠르다".to_string(),
                rating: 5,
            },
            Review {
                text: "품질이 만족스럽다".to_string(),
                rating: 4,
            },
            Review {
                text: "포장이 허술하다".to_string(),
                rating: 2,
            },
            Review {
                text: "생각보다 작다".to_string(),
                rating: 2,
            },
            Review {
                text: "그럭저럭 쓸만하다".to_string(),
                rating: 3,
            },
            Review {
                text: "AS 응답이 느리다".to_string(),
                rating: 1,
            },
        ]
    }
}

#[async_trait]
impl CoupangCrawler for MockCoupangCrawler {
    async fn fetch_reviews(&self, url: &str, limit: u32) -> AppResult<ProductReviews> {
        let mut reviews = Self::fixture_reviews();
        reviews.truncate(limit as usize);
        Ok(ProductReviews {
            url: url.to_string(),
            product_name: Self::product_name_for(url),
            reviews,
        })
    }
}

/// 실제 쿠팡 리뷰 엔드포인트를 호출하는 크롤러.
///
/// 주의: 쿠팡의 실제 리뷰 JSON 스키마는 비공개이며 시점에 따라 변할 수 있다.
/// 따라서 `parse_reviews` 는 방어적으로 작성되어 있으며(여러 후보 필드명 탐색,
/// 누락/타입 불일치 시 해당 항목 skip), 통합 시 실제 응답으로 보정이 필요하다.
pub struct HttpCoupangCrawler {
    client: reqwest::Client,
}

impl HttpCoupangCrawler {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// `https://www.coupang.com/vp/products/12345?...` → Some("12345").
    ///
    /// `/vp/products/{id}` 패턴 뒤의 숫자 세그먼트를 추출한다.
    pub fn extract_product_id(url: &str) -> Option<String> {
        let after = url.split("/products/").nth(1)?;
        let id: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
        if id.is_empty() {
            None
        } else {
            Some(id)
        }
    }

    /// 쿠팡 리뷰 JSON 에서 리뷰 목록을 방어적으로 파싱한다.
    ///
    /// 실제 스키마가 불확실하므로 다음을 가정/허용한다:
    /// - 리뷰 배열은 최상위 배열이거나 `reviews`/`data`/`content` 키 아래에 있을 수 있다.
    /// - 각 항목의 본문은 `content`/`reviewContent`/`text` 중 하나.
    /// - 평점은 `rating`/`reviewRating`/`score`(정수 또는 문자열) 중 하나, 없으면 0.
    ///
    /// 파싱 불가한 항목은 조용히 건너뛴다.
    pub fn parse_reviews(json: &serde_json::Value) -> Vec<Review> {
        let array = if let Some(arr) = json.as_array() {
            arr.clone()
        } else {
            ["reviews", "data", "content"]
                .iter()
                .find_map(|k| json.get(*k).and_then(|v| v.as_array()).cloned())
                .unwrap_or_default()
        };

        array.iter().filter_map(Self::parse_one_review).collect()
    }

    fn parse_one_review(item: &serde_json::Value) -> Option<Review> {
        let text = ["content", "reviewContent", "text"]
            .iter()
            .find_map(|k| item.get(*k).and_then(|v| v.as_str()))
            .map(|s| s.to_string())?;
        if text.trim().is_empty() {
            return None;
        }

        let rating = ["rating", "reviewRating", "score"]
            .iter()
            .find_map(|k| item.get(*k))
            .map(Self::coerce_rating)
            .unwrap_or(0);

        Some(Review { text, rating })
    }

    /// JSON 값(정수/실수/문자열)을 정수 평점으로 강제 변환한다. 실패 시 0.
    fn coerce_rating(v: &serde_json::Value) -> i32 {
        if let Some(i) = v.as_i64() {
            return i as i32;
        }
        if let Some(f) = v.as_f64() {
            return f.round() as i32;
        }
        if let Some(s) = v.as_str() {
            return s
                .trim()
                .parse::<f64>()
                .map(|f| f.round() as i32)
                .unwrap_or(0);
        }
        0
    }
}

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
     AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36";

#[async_trait]
impl CoupangCrawler for HttpCoupangCrawler {
    async fn fetch_reviews(&self, url: &str, limit: u32) -> AppResult<ProductReviews> {
        let product_id = Self::extract_product_id(url)
            .ok_or_else(|| AppError::BadRequest("유효하지 않은 쿠팡 상품 URL".into()))?;

        // 쿠팡 내부 리뷰 JSON 엔드포인트 (스키마 비공개, 통합 시 보정 필요).
        let endpoint = format!(
            "https://www.coupang.com/vp/product/reviews?productId={product_id}&size={limit}"
        );

        let resp = self
            .client
            .get(&endpoint)
            .header("User-Agent", USER_AGENT)
            .send()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let mut reviews = Self::parse_reviews(&json);
        reviews.truncate(limit as usize);

        let product_name = json
            .get("productName")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("쿠팡 상품 {product_id}"));

        Ok(ProductReviews {
            url: url.to_string(),
            product_name,
            reviews,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn mock_crawler_is_deterministic() {
        let crawler = MockCoupangCrawler::new();
        let url = "https://www.coupang.com/vp/products/12345?vendorItemId=1";

        let a = crawler.fetch_reviews(url, 100).await.unwrap();
        let b = crawler.fetch_reviews(url, 100).await.unwrap();

        assert_eq!(a.url, b.url);
        assert_eq!(a.product_name, b.product_name);
        assert_eq!(a.reviews.len(), b.reviews.len());
        for (x, y) in a.reviews.iter().zip(b.reviews.iter()) {
            assert_eq!(x.text, y.text);
            assert_eq!(x.rating, y.rating);
        }
        assert!(a.product_name.contains("12345"));
    }

    #[tokio::test]
    async fn mock_crawler_respects_limit() {
        let crawler = MockCoupangCrawler::new();
        let url = "https://www.coupang.com/vp/products/1";

        let limited = crawler.fetch_reviews(url, 3).await.unwrap();
        assert_eq!(limited.reviews.len(), 3);
    }

    #[test]
    fn extract_product_id_from_url() {
        assert_eq!(
            HttpCoupangCrawler::extract_product_id(
                "https://www.coupang.com/vp/products/12345?vendorItemId=99&q=abc"
            ),
            Some("12345".to_string())
        );
        assert_eq!(
            HttpCoupangCrawler::extract_product_id("https://www.coupang.com/vp/products/777"),
            Some("777".to_string())
        );
        assert_eq!(
            HttpCoupangCrawler::extract_product_id("https://www.coupang.com/np/search"),
            None
        );
    }

    #[test]
    fn parse_reviews_from_nested_array() {
        let body = json!({
            "productName": "테스트",
            "reviews": [
                { "content": "정말 좋아요", "rating": 5 },
                { "reviewContent": "별로예요", "reviewRating": "2" },
                { "text": "보통", "score": 3.4 },
                { "content": "   ", "rating": 4 },
                { "rating": 5 }
            ]
        });

        let reviews = HttpCoupangCrawler::parse_reviews(&body);
        assert_eq!(reviews.len(), 3);
        assert_eq!(reviews[0].text, "정말 좋아요");
        assert_eq!(reviews[0].rating, 5);
        assert_eq!(reviews[1].text, "별로예요");
        assert_eq!(reviews[1].rating, 2);
        assert_eq!(reviews[2].text, "보통");
        assert_eq!(reviews[2].rating, 3);
    }

    #[test]
    fn parse_reviews_from_top_level_array() {
        let body = json!([
            { "content": "굿", "rating": 5 },
            { "content": "배송 느림", "rating": 1 }
        ]);
        let reviews = HttpCoupangCrawler::parse_reviews(&body);
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[1].rating, 1);
    }
}
