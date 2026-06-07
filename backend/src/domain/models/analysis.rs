use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 분석 작업 상태. DB 에는 TEXT(lowercase)로 저장된다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AnalysisStatus {
    #[default]
    Pending,
    Crawling,
    Analyzing,
    Completed,
    Failed,
}

impl AnalysisStatus {
    /// DB 저장용 문자열.
    pub fn as_str(&self) -> &'static str {
        match self {
            AnalysisStatus::Pending => "pending",
            AnalysisStatus::Crawling => "crawling",
            AnalysisStatus::Analyzing => "analyzing",
            AnalysisStatus::Completed => "completed",
            AnalysisStatus::Failed => "failed",
        }
    }

    /// DB 문자열 → AnalysisStatus. 알 수 없는 값은 Pending 으로 폴백한다.
    pub fn from_db_str(s: &str) -> Self {
        match s {
            "crawling" => AnalysisStatus::Crawling,
            "analyzing" => AnalysisStatus::Analyzing,
            "completed" => AnalysisStatus::Completed,
            "failed" => AnalysisStatus::Failed,
            _ => AnalysisStatus::Pending,
        }
    }
}

/// 크롤링된 단일 리뷰. AI 분석 입력으로 사용된다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub text: String,
    pub rating: i32,
}

/// 불만 항목 (집계).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Complaint {
    pub text: String,
    pub count: i32,
    pub severity: String,
}

/// 긍정 항목 (집계).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Positive {
    pub text: String,
    pub count: i32,
}

/// 개선 포인트.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementPoint {
    pub rank: i32,
    pub title: String,
    pub detail: String,
}

/// 경쟁사 약점 → 기회.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorWeakness {
    pub title: String,
    pub opportunity: String,
}

/// AI 가 생성하는 인사이트 묶음.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insights {
    pub top_complaints: Vec<Complaint>,
    pub top_positives: Vec<Positive>,
    pub improvement_points: Vec<ImprovementPoint>,
    pub competitor_weaknesses: Vec<CompetitorWeakness>,
    pub purchase_drivers: Vec<String>,
}

/// 상품별 통계 요약 (리뷰에서 결정론적으로 집계).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSummary {
    pub url: String,
    pub product_name: String,
    pub total_reviews: i32,
    pub avg_rating: f64,
    pub rating_distribution: HashMap<String, i32>,
}

/// 분석 결과 전체. DB 에 JSONB 로 저장된다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub products: Vec<ProductSummary>,
    pub insights: Insights,
}

/// 분석 작업 도메인 엔티티.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub id: Uuid,
    pub user_id: Uuid,
    pub urls: Vec<String>,
    pub review_limit: i32,
    pub status: AnalysisStatus,
    pub result: Option<AnalysisResult>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
