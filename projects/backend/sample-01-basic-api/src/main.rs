use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
    // 1. 로깅 시스템(tracing) 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sample_01_basic_api=debug,tower_http=debug".into()),
        )
        .init();

    // 2. 라우터 구성
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check_handler))
        .route("/api/users/{id}", get(get_user_handler))
        .route("/api/search", get(search_handler))
        .route("/api/users", post(create_user_handler));

    // 3. 서버 바인딩 주소 설정 (127.0.0.1:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 고성능 Axum 서버 시작! http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("포트 3000 바인딩 실패");

    // 4. Graceful Shutdown을 적용하여 안전하게 서버 실행
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("서버 비정상 종료");
}

// ---------------------------------------------------------
// 핸들러 함수들
// ---------------------------------------------------------

/// 루트 페이지: 환영 HTML 반환
async fn root_handler() -> Html<&'static str> {
    Html("<h1>🦀 페리스와 함께하는 고성능 Axum 백엔드 서버!</h1><p>API 엔드포인트가 준비되었습니다.</p>")
}

/// 헬스체크 핸들러
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    framework: &'static str,
    performance: &'static str,
}

async fn health_check_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        framework: "Axum (Tokio-based)",
        performance: "Blazing Fast ⚡",
    })
}

/// Path 파라미터 핸들러: /api/users/{id}
async fn get_user_handler(Path(id): Path<u32>) -> impl IntoResponse {
    info!("유저 ID 조회 요청: {id}");
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "id": id,
            "username": format!("user_{id}"),
            "role": "Rustacean Developer",
            "active": true
        })),
    )
}

/// Query 파라미터 핸들러: /api/search?keyword=rust&limit=5
#[derive(Deserialize, Debug)]
struct SearchParams {
    keyword: String,
    limit: Option<usize>,
}

async fn search_handler(Query(params): Query<SearchParams>) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(10);
    info!("검색 쿼리: keyword='{}', limit={}", params.keyword, limit);

    Json(serde_json::json!({
        "query": params.keyword,
        "limit": limit,
        "results": [
            format!("'{}' 관련 고성능 백엔드 포스팅 1", params.keyword),
            format!("'{}' 비동기 처리 최적화 꿀팁 2", params.keyword),
        ]
    }))
}

/// JSON Body 요청 및 응답 핸들러: POST /api/users
#[derive(Deserialize, Debug)]
struct CreateUserRequest {
    username: String,
    email: String,
}

#[derive(Serialize, Debug)]
struct UserResponse {
    id: u32,
    username: String,
    email: String,
    created_at: &'static str,
}

async fn create_user_handler(
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    info!("새 유저 생성 요청: {:?}", payload);

    let new_user = UserResponse {
        id: 42,
        username: payload.username,
        email: payload.email,
        created_at: "2026-11-01T00:00:00Z",
    };

    (StatusCode::CREATED, Json(new_user))
}

// ---------------------------------------------------------
// Graceful Shutdown (서버 안전 종료) 시그널 수신기
// ---------------------------------------------------------
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Ctrl+C 시그널 리스너 등록 실패");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("SIGTERM 시그널 리스너 등록 실패")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("🛑 종료 시그널 수신! 진행 중인 요청을 처리한 후 서버를 안전하게 종료합니다...");
}

