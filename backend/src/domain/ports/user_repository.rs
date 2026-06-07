use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::User;
use crate::error::AppResult;

/// 사용자 영속성 포트.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// 새 사용자를 생성한다. 이메일 중복 시 `AppError::BadRequest`.
    async fn create(&self, email: &str, password_hash: &str) -> AppResult<User>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
}
