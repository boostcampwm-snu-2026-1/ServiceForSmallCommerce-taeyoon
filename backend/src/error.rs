use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    /// 내부 진단용 상세 메시지.
    ///
    /// 사용자 응답(`Display`)은 `Internal`/`Database` 의 원인을 숨기지만,
    /// 로그·분석 실패 기록(DB `error` 컬럼)에는 실제 원인이 필요하다.
    /// 따라서 여기서는 내부 anyhow(`{:#}` 전체 체인)/sqlx 메시지를 노출한다.
    pub fn detail(&self) -> String {
        match self {
            AppError::Internal(e) => format!("{e:#}"),
            AppError::Database(e) => format!("Database error: {e}"),
            other => other.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Database(_) | AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
