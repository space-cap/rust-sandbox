pub mod handlers;
pub mod models;

use axum::{
    routing::get,
    Router,
};
use handlers::{index_handler, stats_handler, ws_handler, AppState};

/// 라우터를 생성하고 웹소켓 및 정적 웹 UI 엔드포인트를 바인딩합니다.
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .route("/api/stats", get(stats_handler))
        .with_state(state)
}

