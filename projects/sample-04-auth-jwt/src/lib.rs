pub mod auth;
pub mod error;
pub mod handlers;
pub mod models;

use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use handlers::{login_handler, me_handler, public_hello_handler, register_handler, AppState};
use tower_http::cors::{Any, CorsLayer};

/// 라우터를 생성하고 CORS 미들웨어와 상태(State)를 설정합니다.
pub fn create_app(state: AppState) -> Router {
    // 프론트엔드 연동을 위한 관대한 CORS 미들웨어 설정
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(root_handler))
        .route("/api/public/hello", get(public_hello_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/me", get(me_handler))
        .layer(cors)
        .with_state(state)
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>🦀 Sample 04: Argon2 & JWT 보안 인증 서버</h1><p>보안 엔드포인트가 준비되었습니다!</p>")
}

