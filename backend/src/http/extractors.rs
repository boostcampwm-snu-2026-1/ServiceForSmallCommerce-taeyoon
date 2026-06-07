//! Axum 추출기.

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use uuid::Uuid;

use crate::error::AppError;
use crate::http::state::AppState;

/// 인증된 사용자 id.
///
/// `Authorization: Bearer <token>` 헤더를 파싱하고
/// `auth_service.verify_token` 으로 검증한다.
/// 헤더가 없거나 형식이 잘못됐거나 토큰이 유효하지 않으면 `Unauthorized`.
pub struct AuthUser(pub Uuid);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = header
            .strip_prefix("Bearer ")
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .ok_or(AppError::Unauthorized)?;

        let user_id = state.auth_service.verify_token(token).await?;
        Ok(AuthUser(user_id))
    }
}
