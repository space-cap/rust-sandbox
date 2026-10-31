use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sample_04_auth_jwt::{create_app, handlers::AppState, models::{AuthResponse, UserResponse}};
use serde_json::{json, Value};
use tower::ServiceExt;

/// 헬퍼 함수: 요청 전송 및 응답 반환
async fn send_request(
    app: axum::Router,
    req: Request<Body>,
) -> (StatusCode, String) {
    let response = app.oneshot(req).await.expect("요청 처리 실패");
    let status = response.status();
    let body_bytes = response
        .into_body()
        .collect()
        .await
        .expect("응답 바디 읽기 실패")
        .to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).expect("UTF-8 파싱 실패");
    (status, body_str)
}

#[tokio::test]
async fn test_auth_full_lifecycle() {
    let state = AppState::new();
    let app = create_app(state);

    // 1. 회원가입 (POST /api/auth/register)
    let reg_payload = json!({
        "username": "ferris_master",
        "password": "strong_password_123"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/register")
        .header("Content-Type", "application/json")
        .body(Body::from(reg_payload.to_string()))
        .unwrap();

    let (status, body) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::CREATED);
    let user: UserResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(user.username, "ferris_master");

    // 2. 로그인 (POST /api/auth/login)
    let login_payload = json!({
        "username": "ferris_master",
        "password": "strong_password_123"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/login")
        .header("Content-Type", "application/json")
        .body(Body::from(login_payload.to_string()))
        .unwrap();

    let (status, body) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::OK);
    let auth: AuthResponse = serde_json::from_str(&body).unwrap();
    assert!(!auth.token.is_empty());
    assert_eq!(auth.token_type, "Bearer");

    // 3. 보호된 라우트 접근 (GET /api/auth/me) - 토큰 지참!
    let req = Request::builder()
        .method("GET")
        .uri("/api/auth/me")
        .header("Authorization", format!("Bearer {}", auth.token))
        .body(Body::empty())
        .unwrap();

    let (status, body) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::OK);
    let me: UserResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(me.username, "ferris_master");
}

#[tokio::test]
async fn test_unauthorized_without_token() {
    let state = AppState::new();
    let app = create_app(state);

    // 토큰 없이 보호된 라우트 접근 시도 (GET /api/auth/me)
    let req = Request::builder()
        .method("GET")
        .uri("/api/auth/me")
        .body(Body::empty())
        .unwrap();

    let (status, body) = send_request(app, req).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let err: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(err["code"], 401);
    assert_eq!(err["success"], false);
}

#[tokio::test]
async fn test_invalid_password_fails() {
    let state = AppState::new();
    let app = create_app(state);

    // 회원가입
    let reg_payload = json!({
        "username": "test_user",
        "password": "correct_password"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/register")
        .header("Content-Type", "application/json")
        .body(Body::from(reg_payload.to_string()))
        .unwrap();
    let _ = send_request(app.clone(), req).await;

    // 잘못된 패스워드로 로그인 시도
    let bad_login = json!({
        "username": "test_user",
        "password": "wrong_password!"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/login")
        .header("Content-Type", "application/json")
        .body(Body::from(bad_login.to_string()))
        .unwrap();

    let (status, _) = send_request(app, req).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_duplicate_username_conflict() {
    let state = AppState::new();
    let app = create_app(state);

    let reg_payload = json!({
        "username": "duplicate_user",
        "password": "password123"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/register")
        .header("Content-Type", "application/json")
        .body(Body::from(reg_payload.to_string()))
        .unwrap();
    let (status, _) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::CREATED);

    // 같은 아이디로 다시 가입 시도 -> 409 Conflict
    let req2 = Request::builder()
        .method("POST")
        .uri("/api/auth/register")
        .header("Content-Type", "application/json")
        .body(Body::from(reg_payload.to_string()))
        .unwrap();
    let (status2, _) = send_request(app, req2).await;
    assert_eq!(status2, StatusCode::CONFLICT);
}

