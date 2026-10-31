use sample_02_rest_crud::{create_app, handlers::AppState};
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

    // 3. 라우터 생성
    let app = create_app(state);

    // 4. 포트 3001 바인딩
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

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Ctrl+C 시그널 등록 실패");
    info!("🛑 종료 시그널 수신! 안전하게 서버를 종료합니다...");
}
