use crate::application::analysis_service::AnalysisService;
use crate::application::auth_service::AuthService;
use crate::application::user_service::UserService;
use crate::config::Config;
use sqlx::PgPool;
use std::sync::Arc;

/// 앱 시작 시 한 번 초기화, Arc로 매 요청에 Clone.
/// 서비스는 Arc<dyn Trait>로 보관해 testability 확보.
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub config: Config,
    pub auth_service: Arc<dyn AuthService>,
    pub analysis_service: Arc<dyn AnalysisService>,
    pub user_service: Arc<dyn UserService>,
}

impl AppState {
    pub fn new(
        db_pool: PgPool,
        config: Config,
        auth_service: Arc<dyn AuthService>,
        analysis_service: Arc<dyn AnalysisService>,
        user_service: Arc<dyn UserService>,
    ) -> Self {
        Self {
            db_pool,
            config,
            auth_service,
            analysis_service,
            user_service,
        }
    }
}
