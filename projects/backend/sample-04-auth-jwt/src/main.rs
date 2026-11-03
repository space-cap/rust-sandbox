use sample_04_auth_jwt::{create_app, handlers::AppState};
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
    // 1. 로깅 시스템 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sample_04_auth_jwt=debug,tower_http=info".into()),
        )
        .init();

    // 2. 상태 객체 생성
    let state = AppState::new();

    // 3. 라우터 및 미들웨어 생성
    let app = create_app(state);

    // 4. 포트 3003 바인딩
    let addr = SocketAddr::from(([127, 0, 0, 1], 3003));
    info!("🚀 [Sample 04] JWT 인증 서버 시작! http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("포트 3003 바인딩 실패");

    // 5. 서버 가동 및 Graceful Shutdown
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

