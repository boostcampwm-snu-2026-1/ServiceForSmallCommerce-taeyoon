use std::collections::HashMap;

use async_trait::async_trait;

use crate::domain::models::{CompetitorWeakness, Complaint, ImprovementPoint, Insights, Positive};
use crate::domain::ports::ai_analyzer::AiAnalyzer;
use crate::domain::ports::crawler::ProductReviews;
use crate::error::{AppError, AppResult};

/// 기본 Gemini 모델. 제공된 키의 free tier 에서 호출 가능한 모델(gemini-2.5-flash).
/// 참고: gemini-2.0-flash-lite 는 해당 키에서 free tier 쿼터가 0이라 사용 불가.
pub const DEFAULT_MODEL: &str = "gemini-2.5-flash";

/// 테스트/개발용 결정론적 분석기.
///
/// 외부 호출 없이 규칙 기반으로 "내 제품 vs 경쟁사" 비교 인사이트를 생성한다:
/// - 내 제품(is_mine) 리뷰의 rating <= 2 텍스트를 빈도 집계 → top_complaints
/// - 내 제품 리뷰의 rating >= 4 텍스트를 빈도 집계 → top_positives
/// - top_complaints 에서 improvement_points 파생
/// - 경쟁사 리뷰의 rating <= 2 텍스트 → competitor_weaknesses
/// - 경쟁사 리뷰의 rating >= 4 텍스트(빈도순) → purchase_drivers (없으면 내 긍정으로 폴백)
/// - 내/경쟁사 평균 평점 비교 → comparison_summary
///
/// 하위호환: is_mine 인 product 가 하나도 없으면 모든 product 를 "내 제품"처럼 취급한다.
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

    /// 리뷰 묶음의 평균 평점(빈 경우 0.0).
    fn avg_rating<'a>(reviews: impl Iterator<Item = &'a crate::domain::models::Review>) -> f64 {
        let mut sum = 0i64;
        let mut n = 0i64;
        for r in reviews {
            sum += r.rating as i64;
            n += 1;
        }
        if n == 0 {
            0.0
        } else {
            sum as f64 / n as f64
        }
    }
}

#[async_trait]
impl AiAnalyzer for MockAiAnalyzer {
    async fn analyze(&self, products: &[ProductReviews]) -> AppResult<Insights> {
        // 내 제품 / 경쟁사 분리. is_mine 인 product 가 하나도 없으면
        // 하위호환을 위해 모든 product 를 "내 제품"처럼 취급한다.
        let has_mine = products.iter().any(|p| p.is_mine);
        let mine: Vec<&ProductReviews> = if has_mine {
            products.iter().filter(|p| p.is_mine).collect()
        } else {
            products.iter().collect()
        };
        let competitors: Vec<&ProductReviews> = if has_mine {
            products.iter().filter(|p| !p.is_mine).collect()
        } else {
            Vec::new()
        };

        let mine_reviews = || mine.iter().flat_map(|p| p.reviews.iter());
        let comp_reviews = || competitors.iter().flat_map(|p| p.reviews.iter());

        // top_complaints: 내 제품 리뷰 중 rating <= 2.
        let complaint_counts = Self::count_by_text(
            mine_reviews()
                .filter(|r| r.rating <= 2)
                .map(|r| r.text.as_str()),
        );
        // top_positives: 내 제품 리뷰 중 rating >= 4.
        let positive_counts = Self::count_by_text(
            mine_reviews()
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
                detail: format!(
                    "내 제품 리뷰 {}건에서 지적된 불만입니다. 개선하면 경쟁사 대비 우위를 확보합니다.",
                    c.count
                ),
            })
            .collect();

        // competitor_weaknesses: 경쟁사 리뷰 중 rating <= 2.
        let competitor_complaint_counts = Self::count_by_text(
            comp_reviews()
                .filter(|r| r.rating <= 2)
                .map(|r| r.text.as_str()),
        );
        let competitor_weaknesses: Vec<CompetitorWeakness> = competitor_complaint_counts
            .iter()
            .map(|(text, _)| CompetitorWeakness {
                title: text.clone(),
                opportunity: format!("경쟁사의 '{text}' 약점을 내 상품 강점으로 공략하세요."),
            })
            .collect();

        // purchase_drivers: 경쟁사 리뷰 중 rating >= 4 (빈도순). 경쟁사 없으면 내 긍정으로 폴백.
        let competitor_positive_counts = Self::count_by_text(
            comp_reviews()
                .filter(|r| r.rating >= 4)
                .map(|r| r.text.as_str()),
        );
        let purchase_drivers: Vec<String> = if competitor_positive_counts.is_empty() {
            top_positives.iter().map(|p| p.text.clone()).collect()
        } else {
            competitor_positive_counts
                .iter()
                .map(|(text, _)| text.clone())
                .collect()
        };

        // comparison_summary: 내/경쟁사 평균 평점 비교.
        let my_avg = Self::avg_rating(mine_reviews());
        let comp_avg = Self::avg_rating(comp_reviews());
        let comparison_summary = Some(format!(
            "내 제품 평균 평점 {my_avg:.1}점, 경쟁사 평균 {comp_avg:.1}점입니다. \
             내 제품 리뷰의 불만을 우선 개선하면 경쟁사 대비 우위를 확보할 수 있습니다."
        ));

        Ok(Insights {
            top_complaints,
            top_positives,
            improvement_points,
            competitor_weaknesses,
            purchase_drivers,
            comparison_summary,
        })
    }
}

/// Google Gemini `generateContent` API 기반 분석기.
///
/// 주의: 모델 출력은 자유 텍스트일 수 있어 `parse_insights` 는 방어적으로 작성되어 있다
/// (코드펜스 제거, 첫 `{` ~ 마지막 `}` 구간 추출 후 파싱). `responseMimeType` 으로 JSON 을
/// 강제하지만, 안전을 위해 파싱은 그대로 방어적으로 유지한다. 네트워크 호출은 테스트하지 않는다.
pub struct GeminiAiAnalyzer {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

impl GeminiAiAnalyzer {
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

    /// `generateContent` 엔드포인트 URL 을 만든다.
    fn endpoint(&self) -> String {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            self.model
        )
    }

    /// 리뷰들을 포함한 한국어 구조화 JSON 출력 요청 프롬프트를 만든다.
    ///
    /// 내 제품과 경쟁사를 명확히 구분해 제시하고, 셀러의 **내 제품**에서
    /// 무엇을 개선해야 경쟁사를 이길 수 있는지 도출하도록 지시한다.
    pub fn build_prompt(products: &[ProductReviews]) -> String {
        let mut s = String::new();
        s.push_str(
            "당신은 쿠팡 셀러를 돕는 리뷰 분석 전문가입니다. \
             아래에는 셀러의 [내 제품]과 [경쟁사] 상품들의 리뷰가 있습니다. \
             두 제품군의 리뷰를 비교해, 셀러의 [내 제품]에서 무엇을 개선해야 \
             경쟁사를 이길 수 있는지 인사이트를 도출하세요.\n\n",
        );
        for p in products {
            let label = if p.is_mine {
                "[내 제품]"
            } else {
                "[경쟁사]"
            };
            s.push_str(&format!("## {} {} ({})\n", label, p.product_name, p.url));
            for r in &p.reviews {
                s.push_str(&format!("- [{}점] {}\n", r.rating, r.text));
            }
            s.push('\n');
        }
        s.push_str(
            "다음 JSON 스키마로만 응답하세요(설명/코드펜스 금지). \
             top_complaints/top_positives/improvement_points 는 [내 제품] 리뷰 기준, \
             competitor_weaknesses 는 [경쟁사] 약점 기준으로 작성하세요:\n\
             {\n\
             \"top_complaints\": [{\"text\": string, \"count\": number, \"severity\": \"high|medium|low\"}],\n\
             \"top_positives\": [{\"text\": string, \"count\": number}],\n\
             \"improvement_points\": [{\"rank\": number, \"title\": string, \"detail\": string}],\n\
             \"competitor_weaknesses\": [{\"title\": string, \"opportunity\": string}],\n\
             \"purchase_drivers\": [string],\n\
             \"comparison_summary\": string\n\
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
impl AiAnalyzer for GeminiAiAnalyzer {
    async fn analyze(&self, products: &[ProductReviews]) -> AppResult<Insights> {
        let prompt = Self::build_prompt(products);

        let body = serde_json::json!({
            "contents": [{
                "parts": [{ "text": prompt }],
            }],
            "generationConfig": {
                "maxOutputTokens": 4096,
                "responseMimeType": "application/json",
            },
        });

        let resp = self
            .client
            .post(self.endpoint())
            .header("x-goog-api-key", &self.api_key)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        // generateContent: candidates[0].content.parts[0].text
        let text = json
            .get("candidates")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|cand| cand.get("content"))
            .and_then(|content| content.get("parts"))
            .and_then(|parts| parts.as_array())
            .and_then(|arr| arr.first())
            .and_then(|part| part.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Gemini 응답 형식이 예상과 다름")))?;

        Self::parse_insights(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::Review;

    fn product_with(reviews: Vec<(&str, i32)>, is_mine: bool) -> ProductReviews {
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
            is_mine,
        }
    }

    /// 내 제품 ProductReviews 헬퍼.
    fn product_mine(reviews: Vec<(&str, i32)>) -> ProductReviews {
        product_with(reviews, true)
    }

    /// 경쟁사 ProductReviews 헬퍼.
    fn product_comp(reviews: Vec<(&str, i32)>) -> ProductReviews {
        product_with(reviews, false)
    }

    #[tokio::test]
    async fn mock_analyzer_aggregates_complaints_and_positives() {
        // 내 제품: 불만/긍정의 출처. 경쟁사: purchase_drivers/competitor_weaknesses 출처.
        let products = vec![
            product_mine(vec![
                ("포장이 허술하다", 1),
                ("포장이 허술하다", 2),
                ("포장이 허술하다", 1),
                ("배송이 느리다", 2),
                ("가성비가 좋다", 5),
                ("가성비가 좋다", 5),
                ("품질이 좋다", 4),
                ("그저 그렇다", 3), // 무시됨 (3점)
            ]),
            product_comp(vec![
                ("AS 응답이 느리다", 1), // 경쟁사 약점
                ("디자인이 예쁘다", 5),  // 경쟁사 구매 요인
            ]),
        ];

        let a = MockAiAnalyzer::new();
        let insights = a.analyze(&products).await.unwrap();

        // 불만(내 제품 기준): 포장(3) > 배송(1)
        assert_eq!(insights.top_complaints.len(), 2);
        assert_eq!(insights.top_complaints[0].text, "포장이 허술하다");
        assert_eq!(insights.top_complaints[0].count, 3);
        assert_eq!(insights.top_complaints[0].severity, "high");
        assert_eq!(insights.top_complaints[1].text, "배송이 느리다");
        assert_eq!(insights.top_complaints[1].count, 1);
        assert_eq!(insights.top_complaints[1].severity, "low");

        // 긍정(내 제품 기준): 가성비(2) > 품질(1)
        assert_eq!(insights.top_positives.len(), 2);
        assert_eq!(insights.top_positives[0].text, "가성비가 좋다");
        assert_eq!(insights.top_positives[0].count, 2);

        // improvement_points 는 내 제품 불만 기반
        assert_eq!(insights.improvement_points.len(), 2);
        assert_eq!(insights.improvement_points[0].rank, 1);

        // competitor_weaknesses 는 경쟁사 약점 기반
        assert_eq!(insights.competitor_weaknesses.len(), 1);
        assert_eq!(insights.competitor_weaknesses[0].title, "AS 응답이 느리다");

        // purchase_drivers 는 경쟁사 긍정 기반
        assert_eq!(insights.purchase_drivers, vec!["디자인이 예쁘다"]);

        // comparison_summary 존재
        assert!(insights.comparison_summary.is_some());
    }

    #[tokio::test]
    async fn mock_analyzer_is_deterministic() {
        let products = vec![product_mine(vec![("a", 1), ("b", 1), ("c", 5), ("d", 5)])];
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
            "purchase_drivers": ["가격"],
            "comparison_summary": "내 제품이 경쟁사보다 평점이 낮습니다."
        }"#;
        let insights = GeminiAiAnalyzer::parse_insights(text).unwrap();
        assert_eq!(insights.top_complaints[0].text, "포장");
        assert_eq!(insights.top_positives[0].count, 5);
        assert_eq!(insights.purchase_drivers, vec!["가격"]);
        assert_eq!(
            insights.comparison_summary.as_deref(),
            Some("내 제품이 경쟁사보다 평점이 낮습니다.")
        );
    }

    #[test]
    fn parse_insights_strips_code_fence_and_prose() {
        let text = "다음은 분석 결과입니다:\n```json\n{\
            \"top_complaints\": [], \"top_positives\": [], \
            \"improvement_points\": [], \"competitor_weaknesses\": [], \
            \"purchase_drivers\": []}\n```\n감사합니다.";
        let insights = GeminiAiAnalyzer::parse_insights(text).unwrap();
        assert!(insights.top_complaints.is_empty());
        assert!(insights.purchase_drivers.is_empty());
    }

    #[test]
    fn parse_insights_fails_without_json() {
        let err = GeminiAiAnalyzer::parse_insights("JSON 없음").unwrap_err();
        assert!(matches!(err, AppError::Internal(_)));
    }

    #[test]
    fn build_prompt_includes_review_text() {
        let products = vec![
            product_mine(vec![("배송이 빠르다", 5), ("포장이 허술하다", 1)]),
            product_comp(vec![("디자인이 예쁘다", 5)]),
        ];
        let prompt = GeminiAiAnalyzer::build_prompt(&products);
        assert!(prompt.contains("배송이 빠르다"));
        assert!(prompt.contains("포장이 허술하다"));
        assert!(prompt.contains("디자인이 예쁘다"));
        assert!(prompt.contains("테스트 상품"));
        assert!(prompt.contains("top_complaints"));
        // 내 제품/경쟁사 라벨과 comparison_summary 키 포함
        assert!(prompt.contains("[내 제품]"));
        assert!(prompt.contains("[경쟁사]"));
        assert!(prompt.contains("comparison_summary"));
    }

    #[test]
    fn new_uses_default_model_when_empty() {
        let analyzer = GeminiAiAnalyzer::new(reqwest::Client::new(), "key".into(), "".into());
        assert_eq!(analyzer.model, DEFAULT_MODEL);
        assert_eq!(analyzer.api_key, "key");
    }

    #[test]
    fn endpoint_includes_model_name() {
        let analyzer = GeminiAiAnalyzer::new(
            reqwest::Client::new(),
            "key".into(),
            "gemini-2.5-flash".into(),
        );
        let url = analyzer.endpoint();
        assert!(url.contains("generativelanguage.googleapis.com"));
        assert!(url.contains("models/gemini-2.5-flash:generateContent"));
    }
}
