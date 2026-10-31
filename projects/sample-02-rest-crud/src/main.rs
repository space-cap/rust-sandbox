mod error;
mod handlers;
mod models;

use axum::{response::Html, routing::get, Router};
use handlers::{create_todo, delete_todo, get_todo, list_todos, update_todo, AppState};
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
    // 1. 구조화된 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sample_02_rest_crud=debug,tower_http=debug".into()),
        )
        .init();

    // 2. 스레드 안전한 인메모리 상태 생성
    let state = AppState::new();

    // 3. 라우터 설정 및 상태(State) 주입
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/todos", get(list_todos).post(create_todo))
        .route(
            "/api/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .with_state(state);

    // 4. 포트 3001 바인딩 (sample-01과의 포트 충돌 방지)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("🚀 [Sample 02] REST CRUD 서버 시작! http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("포트 3001 바인딩 실패");

    // 5. Graceful Shutdown 적용하여 안전하게 구동
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("서버 비정상 종료");
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>🦀 Sample 02: 견고한 RESTful CRUD API 서버</h1><p><code>/api/todos</code> 엔드포인트를 사용해 보세요!</p>")
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Ctrl+C 시그널 등록 실패");
    info!("🛑 종료 시그널 수신! 안전하게 서버를 종료합니다...");
}
