use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 사용자 플랜. DB 에는 TEXT(lowercase)로 저장된다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Plan {
    #[default]
    Free,
    Starter,
    Pro,
}

impl Plan {
    /// DB 저장용 문자열.
    pub fn as_str(&self) -> &'static str {
        match self {
            Plan::Free => "free",
            Plan::Starter => "starter",
            Plan::Pro => "pro",
        }
    }

    /// DB 문자열 → Plan. 알 수 없는 값은 Free 로 폴백한다.
    pub fn from_db_str(s: &str) -> Self {
        match s {
            "starter" => Plan::Starter,
            "pro" => Plan::Pro,
            _ => Plan::Free,
        }
    }
}

/// 사용자 도메인 엔티티.
///
/// `password_hash` 는 내부 전용이며 절대 직렬화되어 외부로 노출되어선 안 된다.
/// API 응답은 핸들러에서 별도 view 구조체로 구성한다.
#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub plan: Plan,
    pub created_at: DateTime<Utc>,
}
