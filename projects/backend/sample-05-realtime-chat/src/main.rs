use sample_05_realtime_chat::{create_app, handlers::AppState};
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
    // 1. 비동기 구조화 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sample_05_realtime_chat=debug,tower_http=info".into()),
        )
        .init();

    // 2. 브로드캐스트 채널을 포함한 전역 상태 생성
    let state = AppState::new();

    // 3. 라우터 생성
    let app = create_app(state);

    // 4. 포트 3004 바인딩
    let addr = SocketAddr::from(([127, 0, 0, 1], 3004));
    info!("🚀 [Sample 05] 실시간 웹소켓 채팅 서버 시작! http://{}", addr);
    info!("👉 브라우저로 http://127.0.0.1:3004 에 접속하시면 즉시 실시간 채팅을 해보실 수 있습니다!");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("포트 3004 바인딩 실패");

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
    info!("🛑 종료 시그널 수신! 모든 웹소켓 연결을 안전하게 정리하고 종료합니다...");
}

