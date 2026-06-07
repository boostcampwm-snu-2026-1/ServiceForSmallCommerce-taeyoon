//! 분석 핸들러 (생성 / 단건 조회 / 목록).
//!
//! 모두 비표준(AuthUser 추출기, Query, 커스텀 상태코드/뷰)이라 code-rules 의
//! "수동 작성" 규칙(명시적 Result 타입, Serialize 구조체, json! 금지)을 따른다.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::{Analysis, AnalysisResult, AnalysisStatus};
use crate::error::AppError;
use crate::http::extractors::AuthUser;
use crate::http::state::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateAnalysisRequest {
    pub urls: Vec<String>,
    pub review_limit: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateAnalysisResponse {
    pub analysis_id: Uuid,
    pub status: AnalysisStatus,
    pub created_at: DateTime<Utc>,
}

/// 단건 상세 뷰. user_id/review_limit 는 노출하지 않는다.
#[derive(Debug, Serialize)]
pub struct AnalysisDetailView {
    pub id: Uuid,
    pub status: AnalysisStatus,
    pub urls: Vec<String>,
    pub result: Option<AnalysisResult>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<Analysis> for AnalysisDetailView {
    fn from(a: Analysis) -> Self {
        AnalysisDetailView {
            id: a.id,
            status: a.status,
            urls: a.urls,
            result: a.result,
            error: a.error,
            created_at: a.created_at,
            completed_at: a.completed_at,
        }
    }
}

/// 목록용 요약 뷰.
#[derive(Debug, Serialize)]
pub struct AnalysisSummaryView {
    pub id: Uuid,
    pub status: AnalysisStatus,
    pub urls: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Analysis> for AnalysisSummaryView {
    fn from(a: Analysis) -> Self {
        AnalysisSummaryView {
            id: a.id,
            status: a.status,
            urls: a.urls,
            created_at: a.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListAnalysesResponse {
    pub analyses: Vec<AnalysisSummaryView>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

pub async fn create_analysis(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(req): Json<CreateAnalysisRequest>,
) -> Result<(StatusCode, Json<CreateAnalysisResponse>), AppError> {
    let analysis = state
        .analysis_service
        .create_analysis(user_id, req.urls, req.review_limit)
        .await?;
    let body = CreateAnalysisResponse {
        analysis_id: analysis.id,
        status: analysis.status,
        created_at: analysis.created_at,
    };
    Ok((StatusCode::ACCEPTED, Json(body)))
}

pub async fn get_analysis(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AnalysisDetailView>, AppError> {
    let analysis = state.analysis_service.get_analysis(id, user_id).await?;
    Ok(Json(analysis.into()))
}

pub async fn list_analyses(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<ListAnalysesResponse>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let (items, total) = state
        .analysis_service
        .list_analyses(user_id, page, per_page)
        .await?;

    let body = ListAnalysesResponse {
        analyses: items.into_iter().map(Into::into).collect(),
        total,
        page,
        per_page,
    };
    Ok(Json(body))
}
