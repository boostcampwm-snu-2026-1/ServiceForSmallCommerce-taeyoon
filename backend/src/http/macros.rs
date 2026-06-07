//! 엔드포인트 매크로 (컴파일타임 강제).
//!
//! Request/Response 구조체 + 핸들러 함수를 한 번에 생성한다.
//! 반환 타입이 `Json<XxxResponse>`로 고정되어 `impl IntoResponse` 남용을 막는다.
//!
//! hygiene 주의: `$state:ident`, `$req:ident`, `$id:ident`를 metavariable로
//! 캡처해야 `$body` 블록에서 사용 가능하다.

/// POST + JSON body → typed Response
///
/// ```rust,ignore
/// post_endpoint! {
///     fn create_item(state, req: CreateItemRequest {
///         name: String,
///     }) -> CreateItemResponse {
///         id: uuid::Uuid,
///         name: String,
///     }
///     {
///         let item = state.item_service.create(&req.name).await?;
///         Ok(Json(CreateItemResponse { id: item.id, name: item.name }))
///     }
/// }
/// ```
#[macro_export]
macro_rules! post_endpoint {
    (
        fn $handler:ident($state:ident, $req:ident: $req_name:ident {
            $( $req_field:ident : $req_ty:ty ),* $(,)?
        }) -> $resp_name:ident {
            $( $resp_field:ident : $resp_ty:ty ),* $(,)?
        }
        $body:block
    ) => {
        #[derive(Debug, serde::Deserialize)]
        pub struct $req_name { $( pub $req_field: $req_ty, )* }

        #[derive(Debug, serde::Serialize)]
        pub struct $resp_name { $( pub $resp_field: $resp_ty, )* }

        pub async fn $handler(
            axum::extract::State($state): axum::extract::State<$crate::http::state::AppState>,
            axum::Json($req): axum::Json<$req_name>,
        ) -> ::std::result::Result<axum::Json<$resp_name>, $crate::error::AppError>
        $body
    };
}

/// GET /:id → typed Response
#[macro_export]
macro_rules! get_endpoint_with_id {
    (
        fn $handler:ident($state:ident, $id:ident) -> $resp_name:ident {
            $( $resp_field:ident : $resp_ty:ty ),* $(,)?
        }
        $body:block
    ) => {
        #[derive(Debug, serde::Serialize)]
        pub struct $resp_name { $( pub $resp_field: $resp_ty, )* }

        pub async fn $handler(
            axum::extract::State($state): axum::extract::State<$crate::http::state::AppState>,
            axum::extract::Path($id): axum::extract::Path<uuid::Uuid>,
        ) -> ::std::result::Result<axum::Json<$resp_name>, $crate::error::AppError>
        $body
    };
}

/// POST /:id (body 없음) → typed Response
#[macro_export]
macro_rules! post_endpoint_with_id {
    (
        fn $handler:ident($state:ident, $id:ident) -> $resp_name:ident {
            $( $resp_field:ident : $resp_ty:ty ),* $(,)?
        }
        $body:block
    ) => {
        #[derive(Debug, serde::Serialize)]
        pub struct $resp_name { $( pub $resp_field: $resp_ty, )* }

        pub async fn $handler(
            axum::extract::State($state): axum::extract::State<$crate::http::state::AppState>,
            axum::extract::Path($id): axum::extract::Path<uuid::Uuid>,
        ) -> ::std::result::Result<axum::Json<$resp_name>, $crate::error::AppError>
        $body
    };
}
