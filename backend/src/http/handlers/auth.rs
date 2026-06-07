//! 인증 핸들러 (register / login).
//!
//! 매크로(`post_endpoint!`)는 `Json<Resp>` 만 반환하므로 상태코드를 제어할 수 없다.
//! api.md 가 register=201, login=200 을 요구하고 응답이 중첩 `user` 객체를 포함하므로
//! code-rules 의 "수동 작성" 규칙(명시적 Result 타입, Serialize 구조체, json! 금지)을 따라
//! 수동으로 작성한다.

use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::{Plan, User};
use crate::error::AppError;
use crate::http::state::AppState;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// API 노출용 사용자 뷰. `password_hash` 는 절대 포함하지 않는다.
#[derive(Debug, Serialize)]
pub struct UserView {
    pub id: Uuid,
    pub email: String,
    pub plan: Plan,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserView {
    fn from(u: User) -> Self {
        UserView {
            id: u.id,
            email: u.email,
            plan: u.plan,
            created_at: u.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserView,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let (token, user) = state
        .auth_service
        .register(&req.email, &req.password)
        .await?;
    let body = AuthResponse {
        token,
        user: user.into(),
    };
    Ok((StatusCode::CREATED, Json(body)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let (token, user) = state.auth_service.login(&req.email, &req.password).await?;
    let body = AuthResponse {
        token,
        user: user.into(),
    };
    Ok(Json(body))
}
