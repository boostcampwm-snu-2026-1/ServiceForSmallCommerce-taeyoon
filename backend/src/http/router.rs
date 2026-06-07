use axum::{
    routing::{get, MethodRouter},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::http::handlers::health;
use crate::http::state::AppState;

/// 전체 API 라우트 테이블.
///
/// 새 엔드포인트 추가 = 이 목록에 한 줄 추가.
/// 핸들러 함수가 존재하지 않거나 Axum Handler trait 시그니처가 맞지 않으면
/// 이 함수의 컴파일 시점에 에러가 발생한다.
fn route_table() -> Vec<(&'static str, MethodRouter<AppState>)> {
    vec![
        ("/health", get(health::health_check)),
        // 새 엔드포인트는 여기에 추가:
        // ("/api/v1/analyses", get(analysis::list_analyses).post(analysis::create_analysis)),
        // ("/api/v1/analyses/:id", get(analysis::get_analysis)),
    ]
}

pub fn build_router(state: AppState) -> Router {
    route_table()
        .into_iter()
        .fold(Router::new(), |router, (path, handler)| {
            router.route(path, handler)
        })
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
