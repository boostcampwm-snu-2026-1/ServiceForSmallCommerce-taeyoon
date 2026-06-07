use std::collections::HashMap;

use async_trait::async_trait;

use crate::domain::models::{CompetitorWeakness, Complaint, ImprovementPoint, Insights, Positive};
use crate::domain::ports::ai_analyzer::AiAnalyzer;
use crate::domain::ports::crawler::ProductReviews;
use crate::error::{AppError, AppResult};

/// 기본 Claude 모델.
pub const DEFAULT_MODEL: &str = "claude-sonnet-4-6";

/// 테스트/개발용 결정론적 분석기.
///
/// 외부 호출 없이 규칙 기반으로 인사이트를 생성한다:
/// - rating <= 2 리뷰 텍스트를 빈도 집계 → top_complaints (severity 는 count 로 결정)
/// - rating >= 4 리뷰 텍스트를 빈도 집계 → top_positives
/// - 집계 결과에서 improvement_points / competitor_weaknesses / purchase_drivers 파생
#[derive(Debug, Default, Clone)]
pub struct MockAiAnalyzer;

impl MockAiAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// 텍스트별 빈도를 집계해 (text, count) 목록을 count 내림차순으로 반환한다.
    /// 동률은 텍스트 사전순으로 정렬해 결정론을 보장한다.
    fn count_by_text<'a>(texts: impl Iterator<Item = &'a str>) -> Vec<(String, i32)> {
        let mut counts: HashMap<String, i32> = HashMap::new();
        for t in texts {
            *counts.entry(t.to_string()).or_insert(0) += 1;
        }
        let mut items: Vec<(String, i32)> = counts.into_iter().collect();
        items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        items
    }

    fn severity_for(count: i32) -> String {
        if count >= 3 {
            "high".to_string()
        } else if count == 2 {
            "medium".to_string()
        } else {
            "low".to_string()
        }
    }
}

#[async_trait]
impl AiAnalyzer for MockAiAnalyzer {
    async fn analyze(&self, products: &[ProductReviews]) -> AppResult<Insights> {
        let all_reviews = products.iter().flat_map(|p| p.reviews.iter());

        let complaint_counts = Self::count_by_text(
            all_reviews
                .clone()
                .filter(|r| r.rating <= 2)
                .map(|r| r.text.as_str()),
        );
        let positive_counts = Self::count_by_text(
            all_reviews
                .filter(|r| r.rating >= 4)
                .map(|r| r.text.as_str()),
        );

        let top_complaints: Vec<Complaint> = complaint_counts
            .iter()
            .map(|(text, count)| Complaint {
                text: text.clone(),
                count: *count,
                severity: Self::severity_for(*count),
            })
            .collect();

        let top_positives: Vec<Positive> = positive_counts
            .iter()
            .map(|(text, count)| Positive {
                text: text.clone(),
                count: *count,
            })
            .collect();

        let improvement_points: Vec<ImprovementPoint> = top_complaints
            .iter()
            .enumerate()
            .map(|(i, c)| ImprovementPoint {
                rank: (i + 1) as i32,
                title: format!("'{}' 개선", c.text),
                detail: format!("리뷰 {}건에서 언급된 불만을 해소하세요.", c.count),
            })
            .collect();

        let competitor_weaknesses: Vec<CompetitorWeakness> = top_complaints
            .iter()
            .map(|c| CompetitorWeakness {
                title: c.text.clone(),
                opportunity: format!("경쟁사의 '{}' 약점을 공략하세요.", c.text),
            })
            .collect();

        let purchase_drivers: Vec<String> = top_positives.iter().map(|p| p.text.clone()).collect();

        Ok(Insights {
            top_complaints,
            top_positives,
            improvement_points,
            competitor_weaknesses,
            purchase_drivers,
        })
    }
}

/// Claude Messages API 기반 분석기.
///
/// 주의: 모델 출력은 자유 텍스트일 수 있어 `parse_insights` 는 방어적으로 작성되어 있다
/// (코드펜스 제거, 첫 `{` ~ 마지막 `}` 구간 추출 후 파싱). 실제 운영 시 프롬프트와
/// 파싱을 함께 보정해야 한다. 네트워크 호출은 테스트하지 않는다.
pub struct ClaudeAiAnalyzer {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

impl ClaudeAiAnalyzer {
    pub fn new(client: reqwest::Client, api_key: String, model: String) -> Self {
        let model = if model.trim().is_empty() {
            DEFAULT_MODEL.to_string()
        } else {
            model
        };
        Self {
            client,
            api_key,
            model,
        }
    }

    /// 리뷰들을 포함한 한국어 구조화 JSON 출력 요청 프롬프트를 만든다.
    pub fn build_prompt(products: &[ProductReviews]) -> String {
        let mut s = String::new();
        s.push_str(
            "당신은 쿠팡 셀러를 돕는 리뷰 분석 전문가입니다. \
             아래 경쟁 상품들의 리뷰를 분석해, 셀러가 경쟁사를 이길 수 있는 인사이트를 도출하세요.\n\n",
        );
        for p in products {
            s.push_str(&format!("## 상품: {} ({})\n", p.product_name, p.url));
            for r in &p.reviews {
                s.push_str(&format!("- [{}점] {}\n", r.rating, r.text));
            }
            s.push('\n');
        }
        s.push_str(
            "다음 JSON 스키마로만 응답하세요(설명/코드펜스 금지):\n\
             {\n\
             \"top_complaints\": [{\"text\": string, \"count\": number, \"severity\": \"high|medium|low\"}],\n\
             \"top_positives\": [{\"text\": string, \"count\": number}],\n\
             \"improvement_points\": [{\"rank\": number, \"title\": string, \"detail\": string}],\n\
             \"competitor_weaknesses\": [{\"title\": string, \"opportunity\": string}],\n\
             \"purchase_drivers\": [string]\n\
             }\n",
        );
        s
    }

    /// 모델 응답 텍스트에서 JSON 을 추출해 `Insights` 로 파싱한다.
    ///
    /// 코드펜스(```json ... ```)나 앞뒤 설명 텍스트가 섞여 있어도
    /// 첫 `{` 부터 마지막 `}` 까지를 잘라 파싱을 시도한다.
    pub fn parse_insights(text: &str) -> AppResult<Insights> {
        let start = text
            .find('{')
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("응답에서 JSON 을 찾을 수 없음")))?;
        let end = text
            .rfind('}')
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("응답에서 JSON 을 찾을 수 없음")))?;
        if end < start {
            return Err(AppError::Internal(anyhow::anyhow!("잘못된 JSON 범위")));
        }
        let slice = &text[start..=end];
        serde_json::from_str::<Insights>(slice)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Insights 파싱 실패: {e}")))
    }
}

#[async_trait]
impl AiAnalyzer for ClaudeAiAnalyzer {
    async fn analyze(&self, products: &[ProductReviews]) -> AppResult<Insights> {
        let prompt = Self::build_prompt(products);

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": 4096,
            "messages": [{ "role": "user", "content": prompt }],
        });

        let resp = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        // Messages API: content[0].text
        let text = json
            .get("content")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|block| block.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Claude 응답 형식이 예상과 다름")))?;

        Self::parse_insights(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::Review;

    fn product(reviews: Vec<(&str, i32)>) -> ProductReviews {
        ProductReviews {
            url: "https://www.coupang.com/vp/products/1".to_string(),
            product_name: "테스트 상품".to_string(),
            reviews: reviews
                .into_iter()
                .map(|(t, r)| Review {
                    text: t.to_string(),
                    rating: r,
                })
                .collect(),
        }
    }

    #[tokio::test]
    async fn mock_analyzer_aggregates_complaints_and_positives() {
        let products = vec![product(vec![
            ("포장이 허술하다", 1),
            ("포장이 허술하다", 2),
            ("포장이 허술하다", 1),
            ("배송이 느리다", 2),
            ("가성비가 좋다", 5),
            ("가성비가 좋다", 5),
            ("품질이 좋다", 4),
            ("그저 그렇다", 3), // 무시됨 (3점)
        ])];

        let a = MockAiAnalyzer::new();
        let insights = a.analyze(&products).await.unwrap();

        // 불만: 포장(3) > 배송(1)
        assert_eq!(insights.top_complaints.len(), 2);
        assert_eq!(insights.top_complaints[0].text, "포장이 허술하다");
        assert_eq!(insights.top_complaints[0].count, 3);
        assert_eq!(insights.top_complaints[0].severity, "high");
        assert_eq!(insights.top_complaints[1].text, "배송이 느리다");
        assert_eq!(insights.top_complaints[1].count, 1);
        assert_eq!(insights.top_complaints[1].severity, "low");

        // 긍정: 가성비(2) > 품질(1)
        assert_eq!(insights.top_positives.len(), 2);
        assert_eq!(insights.top_positives[0].text, "가성비가 좋다");
        assert_eq!(insights.top_positives[0].count, 2);

        // 파생 항목 개수 일치
        assert_eq!(insights.improvement_points.len(), 2);
        assert_eq!(insights.improvement_points[0].rank, 1);
        assert_eq!(insights.competitor_weaknesses.len(), 2);
        assert_eq!(
            insights.purchase_drivers,
            vec!["가성비가 좋다", "품질이 좋다"]
        );
    }

    #[tokio::test]
    async fn mock_analyzer_is_deterministic() {
        let products = vec![product(vec![("a", 1), ("b", 1), ("c", 5), ("d", 5)])];
        let a = MockAiAnalyzer::new();
        let r1 = a.analyze(&products).await.unwrap();
        let r2 = a.analyze(&products).await.unwrap();
        assert_eq!(
            serde_json::to_value(&r1).unwrap(),
            serde_json::to_value(&r2).unwrap()
        );
    }

    #[test]
    fn parse_insights_from_valid_json() {
        let text = r#"{
            "top_complaints": [{"text": "포장", "count": 3, "severity": "high"}],
            "top_positives": [{"text": "가성비", "count": 5}],
            "improvement_points": [{"rank": 1, "title": "포장 강화", "detail": "완충재"}],
            "competitor_weaknesses": [{"title": "AS", "opportunity": "빠른 CS"}],
            "purchase_drivers": ["가격"]
        }"#;
        let insights = ClaudeAiAnalyzer::parse_insights(text).unwrap();
        assert_eq!(insights.top_complaints[0].text, "포장");
        assert_eq!(insights.top_positives[0].count, 5);
        assert_eq!(insights.purchase_drivers, vec!["가격"]);
    }

    #[test]
    fn parse_insights_strips_code_fence_and_prose() {
        let text = "다음은 분석 결과입니다:\n```json\n{\
            \"top_complaints\": [], \"top_positives\": [], \
            \"improvement_points\": [], \"competitor_weaknesses\": [], \
            \"purchase_drivers\": []}\n```\n감사합니다.";
        let insights = ClaudeAiAnalyzer::parse_insights(text).unwrap();
        assert!(insights.top_complaints.is_empty());
        assert!(insights.purchase_drivers.is_empty());
    }

    #[test]
    fn parse_insights_fails_without_json() {
        let err = ClaudeAiAnalyzer::parse_insights("JSON 없음").unwrap_err();
        assert!(matches!(err, AppError::Internal(_)));
    }

    #[test]
    fn build_prompt_includes_review_text() {
        let products = vec![product(vec![("배송이 빠르다", 5), ("포장이 허술하다", 1)])];
        let prompt = ClaudeAiAnalyzer::build_prompt(&products);
        assert!(prompt.contains("배송이 빠르다"));
        assert!(prompt.contains("포장이 허술하다"));
        assert!(prompt.contains("테스트 상품"));
        assert!(prompt.contains("top_complaints"));
    }

    #[test]
    fn new_uses_default_model_when_empty() {
        let analyzer = ClaudeAiAnalyzer::new(reqwest::Client::new(), "key".into(), "".into());
        assert_eq!(analyzer.model, DEFAULT_MODEL);
        assert_eq!(analyzer.api_key, "key");
    }
}
