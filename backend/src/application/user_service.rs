use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::User;
use crate::domain::ports::analysis_repository::AnalysisRepository;
use crate::domain::ports::user_repository::UserRepository;
use crate::error::{AppError, AppResult};

/// `/users/me` 응답용 사용자 + 사용량 묶음.
pub struct UserMe {
    pub user: User,
    pub analyses_this_month: i64,
}

/// 사용자 정보 유스케이스 포트.
#[async_trait]
pub trait UserService: Send + Sync {
    /// 현재 사용자 + 이번 달 분석 사용량. 사용자 없으면 `NotFound`.
    async fn get_me(&self, user_id: Uuid) -> AppResult<UserMe>;
}

/// user_repo + analysis_repo 조합 기본 구현.
pub struct StandardUserService {
    user_repo: Arc<dyn UserRepository>,
    analysis_repo: Arc<dyn AnalysisRepository>,
}

impl StandardUserService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        analysis_repo: Arc<dyn AnalysisRepository>,
    ) -> Self {
        Self {
            user_repo,
            analysis_repo,
        }
    }
}

#[async_trait]
impl UserService for StandardUserService {
    async fn get_me(&self, user_id: Uuid) -> AppResult<UserMe> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound)?;
        let analyses_this_month = self.analysis_repo.count_this_month(user_id).await?;
        Ok(UserMe {
            user,
            analyses_this_month,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{Analysis, AnalysisResult, AnalysisStatus, Plan};
    use chrono::Utc;
    use std::sync::Mutex;

    struct MockUserRepository {
        users: Mutex<Vec<User>>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, email: &str, password_hash: &str) -> AppResult<User> {
            let user = User {
                id: Uuid::new_v4(),
                email: email.to_string(),
                password_hash: password_hash.to_string(),
                plan: Plan::Free,
                created_at: Utc::now(),
            };
            self.users.lock().unwrap().push(user.clone());
            Ok(user)
        }

        async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.email == email)
                .cloned())
        }

        async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.id == id)
                .cloned())
        }
    }

    struct MockAnalysisRepository {
        count: i64,
    }

    #[async_trait]
    impl AnalysisRepository for MockAnalysisRepository {
        async fn create(
            &self,
            _user_id: Uuid,
            _my_url: &str,
            _urls: &[String],
            _review_limit: i32,
        ) -> AppResult<Analysis> {
            unimplemented!()
        }
        async fn find_by_id(&self, _id: Uuid) -> AppResult<Option<Analysis>> {
            Ok(None)
        }
        async fn list_by_user(
            &self,
            _user_id: Uuid,
            _page: i64,
            _per_page: i64,
        ) -> AppResult<(Vec<Analysis>, i64)> {
            Ok((vec![], 0))
        }
        async fn update_status(&self, _id: Uuid, _status: AnalysisStatus) -> AppResult<()> {
            Ok(())
        }
        async fn set_result(&self, _id: Uuid, _result: &AnalysisResult) -> AppResult<()> {
            Ok(())
        }
        async fn set_error(&self, _id: Uuid, _error: &str) -> AppResult<()> {
            Ok(())
        }
        async fn count_this_month(&self, _user_id: Uuid) -> AppResult<i64> {
            Ok(self.count)
        }
    }

    #[tokio::test]
    async fn get_me_returns_user_and_count() {
        let user_repo = Arc::new(MockUserRepository {
            users: Mutex::new(Vec::new()),
        });
        let created = user_repo.create("me@example.com", "hash").await.unwrap();
        let analysis_repo = Arc::new(MockAnalysisRepository { count: 7 });

        let svc = StandardUserService::new(user_repo, analysis_repo);
        let me = svc.get_me(created.id).await.unwrap();
        assert_eq!(me.user.email, "me@example.com");
        assert_eq!(me.analyses_this_month, 7);
    }

    #[tokio::test]
    async fn get_me_unknown_user_not_found() {
        let user_repo = Arc::new(MockUserRepository {
            users: Mutex::new(Vec::new()),
        });
        let analysis_repo = Arc::new(MockAnalysisRepository { count: 0 });
        let svc = StandardUserService::new(user_repo, analysis_repo);
        assert!(matches!(
            svc.get_me(Uuid::new_v4()).await,
            Err(AppError::NotFound)
        ));
    }
}
