//! 사용자 핸들러 (`GET /users/me`).
//!
//! 수동 작성(AuthUser 추출기, 중첩 usage 객체). json! 금지, Serialize 구조체 사용.

use axum::{extract::State, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::models::Plan;
use crate::error::AppError;
use crate::http::extractors::AuthUser;
use crate::http::state::AppState;

#[derive(Debug, Serialize)]
pub struct UsageView {
    pub analyses_this_month: i64,
    /// 항상 null: 플랜별 제한 강제는 보류(수익 기능). null = 제한 없음.
    pub analyses_limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct UserMeResponse {
    pub id: Uuid,
    pub email: String,
    pub plan: Plan,
    pub usage: UsageView,
}

pub async fn get_me(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<UserMeResponse>, AppError> {
    let me = state.user_service.get_me(user_id).await?;
    let body = UserMeResponse {
        id: me.user.id,
        email: me.user.email,
        plan: me.user.plan,
        usage: UsageView {
            analyses_this_month: me.analyses_this_month,
            analyses_limit: None,
        },
    };
    Ok(Json(body))
}
