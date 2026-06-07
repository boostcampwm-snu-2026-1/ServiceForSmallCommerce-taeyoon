use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::User;
use crate::domain::ports::user_repository::UserRepository;
use crate::error::{AppError, AppResult};

/// JWT 클레임. `sub` 는 user id(UUID 문자열), `exp` 는 만료(epoch secs).
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// 인증 서비스 포트.
#[async_trait]
pub trait AuthService: Send + Sync {
    /// 회원가입. 성공 시 `(JWT, User)` 반환.
    async fn register(&self, email: &str, password: &str) -> AppResult<(String, User)>;
    /// 로그인. 성공 시 `(JWT, User)` 반환.
    async fn login(&self, email: &str, password: &str) -> AppResult<(String, User)>;
    /// 토큰 검증. 성공 시 user id 반환. 만료/위조 시 `Unauthorized`.
    async fn verify_token(&self, token: &str) -> AppResult<Uuid>;
}

/// argon2 + jsonwebtoken 기반 기본 구현.
pub struct StandardAuthService {
    repo: Arc<dyn UserRepository>,
    jwt_secret: String,
    jwt_expires_in: u64,
}

impl StandardAuthService {
    pub fn new(repo: Arc<dyn UserRepository>, jwt_secret: String, jwt_expires_in: u64) -> Self {
        Self {
            repo,
            jwt_secret,
            jwt_expires_in,
        }
    }

    fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("password hashing failed: {e}")))?
            .to_string();
        Ok(hash)
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> bool {
        let parsed = match PasswordHash::new(password_hash) {
            Ok(p) => p,
            Err(_) => return false,
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }

    fn issue_token(&self, user_id: Uuid) -> AppResult<String> {
        let exp = (chrono::Utc::now().timestamp() as u64 + self.jwt_expires_in) as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            exp,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(anyhow::anyhow!("token encoding failed: {e}")))
    }
}

/// 매우 단순한 이메일 형식 검증(`@` 와 `.` 존재 여부).
fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    parts.len() == 2 && !parts[0].is_empty() && parts[1].contains('.') && !parts[1].starts_with('.')
}

#[async_trait]
impl AuthService for StandardAuthService {
    async fn register(&self, email: &str, password: &str) -> AppResult<(String, User)> {
        if !is_valid_email(email) {
            return Err(AppError::BadRequest("올바른 이메일 형식이 아닙니다".into()));
        }
        if password.len() < 8 {
            return Err(AppError::BadRequest(
                "비밀번호는 8자 이상이어야 합니다".into(),
            ));
        }

        let password_hash = self.hash_password(password)?;
        let user = self.repo.create(email, &password_hash).await?;
        let token = self.issue_token(user.id)?;
        Ok((token, user))
    }

    async fn login(&self, email: &str, password: &str) -> AppResult<(String, User)> {
        let user = self
            .repo
            .find_by_email(email)
            .await?
            .ok_or(AppError::Unauthorized)?;

        if !self.verify_password(password, &user.password_hash) {
            return Err(AppError::Unauthorized);
        }

        let token = self.issue_token(user.id)?;
        Ok((token, user))
    }

    async fn verify_token(&self, token: &str) -> AppResult<Uuid> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        Uuid::parse_str(&data.claims.sub).map_err(|_| AppError::Unauthorized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::Plan;
    use chrono::Utc;
    use tokio::sync::Mutex;

    /// in-memory UserRepository (Docker 불필요).
    struct MockUserRepository {
        users: Mutex<Vec<User>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, email: &str, password_hash: &str) -> AppResult<User> {
            let mut users = self.users.lock().await;
            if users.iter().any(|u| u.email == email) {
                return Err(AppError::BadRequest("이미 가입된 이메일입니다".into()));
            }
            let user = User {
                id: Uuid::new_v4(),
                email: email.to_string(),
                password_hash: password_hash.to_string(),
                plan: Plan::Free,
                created_at: Utc::now(),
            };
            users.push(user.clone());
            Ok(user)
        }

        async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
            let users = self.users.lock().await;
            Ok(users.iter().find(|u| u.email == email).cloned())
        }

        async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
            let users = self.users.lock().await;
            Ok(users.iter().find(|u| u.id == id).cloned())
        }
    }

    fn service() -> StandardAuthService {
        StandardAuthService::new(
            Arc::new(MockUserRepository::new()),
            "unit-test-secret".to_string(),
            86400,
        )
    }

    #[tokio::test]
    async fn register_then_verify_token_roundtrip() {
        let svc = service();
        let (token, user) = svc
            .register("bob@example.com", "password123")
            .await
            .expect("register should succeed");

        assert_eq!(user.email, "bob@example.com");
        assert_ne!(user.password_hash, "password123"); // 해시 저장
        let uid = svc
            .verify_token(&token)
            .await
            .expect("verify should succeed");
        assert_eq!(uid, user.id);
    }

    #[tokio::test]
    async fn register_rejects_short_password() {
        let svc = service();
        let err = svc
            .register("short@example.com", "short")
            .await
            .expect_err("should reject short password");
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[tokio::test]
    async fn register_rejects_invalid_email() {
        let svc = service();
        let err = svc
            .register("not-an-email", "password123")
            .await
            .expect_err("should reject invalid email");
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[tokio::test]
    async fn duplicate_email_register_fails() {
        let svc = service();
        svc.register("dup@example.com", "password123")
            .await
            .expect("first register should succeed");
        let err = svc
            .register("dup@example.com", "password456")
            .await
            .expect_err("duplicate should fail");
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[tokio::test]
    async fn login_with_wrong_password_fails() {
        let svc = service();
        svc.register("login@example.com", "password123")
            .await
            .expect("register should succeed");

        let err = svc
            .login("login@example.com", "wrongpassword")
            .await
            .expect_err("wrong password should fail");
        assert!(matches!(err, AppError::Unauthorized));

        let (token, _) = svc
            .login("login@example.com", "password123")
            .await
            .expect("correct password should succeed");
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn login_unknown_email_fails() {
        let svc = service();
        let err = svc
            .login("ghost@example.com", "password123")
            .await
            .expect_err("unknown email should fail");
        assert!(matches!(err, AppError::Unauthorized));
    }

    #[tokio::test]
    async fn verify_invalid_token_fails() {
        let svc = service();
        let err = svc
            .verify_token("not.a.real.token")
            .await
            .expect_err("invalid token should fail");
        assert!(matches!(err, AppError::Unauthorized));
    }
}
