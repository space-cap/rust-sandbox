use sample_03_sqlx_database::{create_app, handlers::AppState, setup_database};
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
    // 1. 로깅 시스템 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sample_03_sqlx_database=debug,sqlx=info".into()),
        )
        .init();

    // 2. SQLite 데이터베이스 연결 및 자동 마이그레이션 실행
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:todos.db".into());
    info!("🗄️ 데이터베이스 연결 시도: {}", db_url);

    let pool = setup_database(&db_url)
        .await
        .expect("데이터베이스 초기화 실패");
    info!("✅ 데이터베이스 마이그레이션 및 커넥션 풀 준비 완료!");

    // 3. 상태 주입 및 라우터 생성
    let state = AppState::new(pool);
    let app = create_app(state);

    // 4. 포트 3002 바인딩
    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    info!("🚀 [Sample 03] SQLx DB 서버 시작! http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("포트 3002 바인딩 실패");

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
    info!("🛑 종료 시그널 수신! 진행 중인 쿼리를 마친 뒤 안전하게 서버를 종료합니다...");
}

