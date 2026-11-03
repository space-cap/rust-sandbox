use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sample_03_sqlx_database::{create_app, handlers::AppState, models::Todo, setup_database};
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
async fn test_database_crud_lifecycle() {
    // 인메모리 SQLite를 사용하여 다른 테스트나 로컬 파일에 영향을 주지 않는 완벽한 격리 테스트!
    let pool = setup_database("sqlite::memory:")
        .await
        .expect("인메모리 DB 설정 실패");
    let state = AppState::new(pool);
    let app = create_app(state);

    // 1. 신규 Todo 생성 (POST)
    let payload = json!({
        "title": "SQLx 마스터하기",
        "description": "컴파일 타임 안전성 검증"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let (status, body) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::CREATED);
    let created: Todo = serde_json::from_str(&body).unwrap();
    assert_eq!(created.id, 1);
    assert_eq!(created.title, "SQLx 마스터하기");
    assert!(!created.completed);

    // 2. 단건 조회 (GET)
    let req = Request::builder()
        .method("GET")
        .uri("/api/todos/1")
        .body(Body::empty())
        .unwrap();
    let (status, body) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::OK);
    let fetched: Todo = serde_json::from_str(&body).unwrap();
    assert_eq!(fetched.id, 1);

    // 3. 수정 (PUT)
    let update_payload = json!({
        "completed": true
    });
    let req = Request::builder()
        .method("PUT")
        .uri("/api/todos/1")
        .header("Content-Type", "application/json")
        .body(Body::from(update_payload.to_string()))
        .unwrap();
    let (status, body) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::OK);
    let updated: Todo = serde_json::from_str(&body).unwrap();
    assert!(updated.completed);

    // 4. 삭제 (DELETE)
    let req = Request::builder()
        .method("DELETE")
        .uri("/api/todos/1")
        .body(Body::empty())
        .unwrap();
    let (status, _) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    // 5. 삭제 후 재조회 시 404 Not Found 확인
    let req = Request::builder()
        .method("GET")
        .uri("/api/todos/1")
        .body(Body::empty())
        .unwrap();
    let (status, _) = send_request(app, req).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_empty_title_validation() {
    let pool = setup_database("sqlite::memory:").await.unwrap();
    let app = create_app(AppState::new(pool));

    let payload = json!({
        "title": "   "
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let (status, body) = send_request(app, req).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let err_json: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(err_json["code"], 400);
}

