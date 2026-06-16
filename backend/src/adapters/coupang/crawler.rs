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
            is_mine: false,
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
    /// 스크래핑 API 프록시 URL 템플릿(`{url}` 치환). None 이면 직접 호출.
    proxy_url: Option<String>,
}

impl HttpCoupangCrawler {
    pub fn new(client: reqwest::Client, proxy_url: Option<String>) -> Self {
        Self { client, proxy_url }
    }

    /// 쿠팡 리뷰 JSON API(next-api/review) URL 을 만든다.
    ///
    /// 쿠팡 상품 상세에서 실제로 호출하는 엔드포인트로, 평점 요약 포함 정렬 리뷰를 반환한다.
    pub fn build_review_api_url(product_id: &str, limit: u32) -> String {
        format!(
            "https://www.coupang.com/next-api/review?productId={product_id}\
             &page=1&size={limit}&sortBy=ORDER_SCORE_ASC&ratingSummary=true&ratings=&market="
        )
    }

    /// 프록시 템플릿이 있으면 타겟 URL 을 URL-encode 해 `{url}` 에 치환한다.
    /// 없으면 타겟 URL 을 그대로 반환(직접 호출).
    pub fn resolve_request_url(&self, target: &str) -> String {
        match &self.proxy_url {
            Some(tmpl) if tmpl.contains("{url}") => tmpl.replace("{url}", &percent_encode(target)),
            // 템플릿에 {url} 가 없으면 쿼리로 덧붙인다(?url= 또는 &url=).
            Some(tmpl) => {
                let sep = if tmpl.contains('?') { '&' } else { '?' };
                format!("{tmpl}{sep}url={}", percent_encode(target))
            }
            None => target.to_string(),
        }
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
        // 1) 빠른 경로: 최상위 배열 또는 알려진 키.
        if let Some(arr) = json.as_array() {
            let r: Vec<Review> = arr.iter().filter_map(Self::parse_one_review).collect();
            if !r.is_empty() {
                return r;
            }
        }
        for k in [
            "reviews",
            "data",
            "content",
            "reviewList",
            "items",
            "contents",
        ] {
            if let Some(arr) = json.get(k).and_then(|v| v.as_array()) {
                let r: Vec<Review> = arr.iter().filter_map(Self::parse_one_review).collect();
                if !r.is_empty() {
                    return r;
                }
            }
        }
        // 2) 깊이 탐색: 스키마가 바뀌어도 리뷰처럼 보이는 배열을 재귀로 찾는다.
        Self::find_reviews_deep(json).unwrap_or_default()
    }

    /// 객체/배열을 재귀 순회하며 "리뷰 항목으로 파싱되는" 원소가 있는 배열을 찾는다.
    /// 가장 먼저 발견되는 비어있지 않은 결과를 반환한다.
    fn find_reviews_deep(v: &serde_json::Value) -> Option<Vec<Review>> {
        match v {
            serde_json::Value::Array(arr) => {
                let r: Vec<Review> = arr.iter().filter_map(Self::parse_one_review).collect();
                if !r.is_empty() {
                    return Some(r);
                }
                arr.iter().find_map(Self::find_reviews_deep)
            }
            serde_json::Value::Object(map) => map.values().find_map(Self::find_reviews_deep),
            _ => None,
        }
    }

    /// 응답에서 상품명을 깊이 탐색한다(여러 후보 키). 없으면 None.
    pub fn find_product_name(v: &serde_json::Value) -> Option<String> {
        const KEYS: [&str; 4] = ["productName", "sellerProductName", "title", "itemName"];
        match v {
            serde_json::Value::Object(map) => {
                for k in KEYS {
                    if let Some(s) = map.get(k).and_then(|x| x.as_str()) {
                        if !s.trim().is_empty() {
                            return Some(s.to_string());
                        }
                    }
                }
                map.values().find_map(Self::find_product_name)
            }
            serde_json::Value::Array(arr) => arr.iter().find_map(Self::find_product_name),
            _ => None,
        }
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

/// 의존성 추가 없이 URL 컴포넌트를 percent-encode 한다(RFC 3986 unreserved 외 인코딩).
/// 스크래핑 API 프록시 템플릿에 타겟 URL 을 안전하게 끼워넣기 위해 사용한다.
fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[async_trait]
impl CoupangCrawler for HttpCoupangCrawler {
    async fn fetch_reviews(&self, url: &str, limit: u32) -> AppResult<ProductReviews> {
        let product_id = Self::extract_product_id(url)
            .ok_or_else(|| AppError::BadRequest("유효하지 않은 쿠팡 상품 URL".into()))?;

        // 쿠팡 상품 상세가 실제로 호출하는 리뷰 JSON API.
        let target = Self::build_review_api_url(&product_id, limit);
        // 프록시(스크래핑 API)가 설정돼 있으면 경유, 아니면 직접 호출.
        let request_url = self.resolve_request_url(&target);

        // 실제 브라우저처럼 보이도록 헤더 구성(직접 호출 시 최선의 노력).
        // 주: 데이터센터 IP 는 Akamai 가 IP 단에서 403 처리하므로, 안정적 수집에는
        // 프록시(COUPANG_PROXY_URL) 또는 가정용 IP/헤드리스 환경이 필요하다.
        let resp = self
            .client
            .get(&request_url)
            .header("User-Agent", USER_AGENT)
            .header(
                "Referer",
                format!("https://www.coupang.com/vp/products/{product_id}"),
            )
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "ko-KR,ko;q=0.9,en;q=0.8")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .send()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        // 상태코드를 먼저 검사해 실패 원인이 보이도록 한다(403 anti-bot 등).
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        if !status.is_success() {
            let snippet: String = body.chars().take(200).collect();
            return Err(AppError::Internal(anyhow::anyhow!(
                "쿠팡 리뷰 API 응답 실패: HTTP {status} — {snippet}"
            )));
        }

        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
            let snippet: String = body.chars().take(200).collect();
            AppError::Internal(anyhow::anyhow!(
                "쿠팡 리뷰 응답이 JSON 이 아님({e}): {snippet}"
            ))
        })?;

        let mut reviews = Self::parse_reviews(&json);
        reviews.truncate(limit as usize);

        let product_name =
            Self::find_product_name(&json).unwrap_or_else(|| format!("쿠팡 상품 {product_id}"));

        Ok(ProductReviews {
            url: url.to_string(),
            product_name,
            reviews,
            is_mine: false,
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

    #[test]
    fn build_review_api_url_uses_next_api() {
        let url = HttpCoupangCrawler::build_review_api_url("7297090513", 50);
        assert!(url.contains("www.coupang.com/next-api/review"));
        assert!(url.contains("productId=7297090513"));
        assert!(url.contains("size=50"));
        assert!(url.contains("ratingSummary=true"));
    }

    #[test]
    fn resolve_request_url_passes_through_when_no_proxy() {
        let c = HttpCoupangCrawler::new(reqwest::Client::new(), None);
        let target = "https://www.coupang.com/next-api/review?productId=1";
        assert_eq!(c.resolve_request_url(target), target);
    }

    #[test]
    fn resolve_request_url_substitutes_url_placeholder() {
        let c = HttpCoupangCrawler::new(
            reqwest::Client::new(),
            Some("https://api.scraperapi.com/?api_key=KEY&url={url}".into()),
        );
        let out =
            c.resolve_request_url("https://www.coupang.com/next-api/review?productId=1&size=2");
        assert!(out.starts_with("https://api.scraperapi.com/?api_key=KEY&url="));
        // 타겟 URL 은 percent-encode 되어야 한다(콜론/슬래시/물음표/앰퍼샌드).
        assert!(out.contains("https%3A%2F%2Fwww.coupang.com"));
        assert!(out.contains("%3FproductId%3D1%26size%3D2"));
    }

    #[test]
    fn resolve_request_url_appends_url_param_without_placeholder() {
        let c = HttpCoupangCrawler::new(
            reqwest::Client::new(),
            Some("https://proxy.example/get?api_key=KEY".into()),
        );
        let out = c.resolve_request_url("https://x.test/a?b=1");
        assert!(out.starts_with("https://proxy.example/get?api_key=KEY&url="));
    }

    #[test]
    fn parse_reviews_deep_finds_nested_array() {
        // next-api/review 형태로 깊이 중첩된 리뷰 배열도 발견해야 한다.
        let body = json!({
            "rCode": "0",
            "rData": {
                "paging": { "page": 1 },
                "contents": [
                    { "reviewContent": "정말 좋아요", "reviewRating": 5 },
                    { "reviewContent": "별로", "reviewRating": "2" }
                ]
            }
        });
        let reviews = HttpCoupangCrawler::parse_reviews(&body);
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].text, "정말 좋아요");
        assert_eq!(reviews[0].rating, 5);
        assert_eq!(reviews[1].rating, 2);
    }

    #[test]
    fn find_product_name_searches_deeply() {
        let body = json!({
            "rData": { "product": { "sellerProductName": "꼬박꼬밥 도시락" } }
        });
        assert_eq!(
            HttpCoupangCrawler::find_product_name(&body),
            Some("꼬박꼬밥 도시락".to_string())
        );
    }
}
