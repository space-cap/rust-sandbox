use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sample_02_rest_crud::{create_app, handlers::AppState, models::Todo};
use serde_json::{json, Value};
use tower::ServiceExt; // oneshot 메서드 사용

/// 헬퍼: 요청을 보내고 응답의 상태 코드와 본문 문자열을 반환
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
    let body_str = String::from_utf8(body_bytes.to_vec()).expect("UTF-8 변환 실패");
    (status, body_str)
}

#[tokio::test]
async fn test_create_and_get_todo() {
    let state = AppState::new();
    let app = create_app(state.clone());

    // 1. 신규 Todo 등록 (POST /api/todos)
    let create_payload = json!({
        "title": "Rust 자동화 테스트 마스터하기",
        "description": "Axum oneshot 테스트 작성법 익히기"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/api/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(create_payload.to_string()))
        .unwrap();

    let (status, body) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::CREATED);

    let created: Todo = serde_json::from_str(&body).expect("JSON 역직렬화 실패");
    assert_eq!(created.id, 1);
    assert_eq!(created.title, "Rust 자동화 테스트 마스터하기");
    assert!(!created.completed);

    // 2. 등록된 Todo 상세 조회 (GET /api/todos/1)
    let req = Request::builder()
        .method("GET")
        .uri("/api/todos/1")
        .body(Body::empty())
        .unwrap();

    let (status, body) = send_request(app, req).await;
    assert_eq!(status, StatusCode::OK);

    let fetched: Todo = serde_json::from_str(&body).unwrap();
    assert_eq!(fetched.id, 1);
    assert_eq!(fetched.title, "Rust 자동화 테스트 마스터하기");
}

#[tokio::test]
async fn test_empty_title_validation_fails() {
    let state = AppState::new();
    let app = create_app(state);

    // 공백 문자열 제목으로 등록 시도 (POST /api/todos)
    let invalid_payload = json!({
        "title": "   "
    });

    let req = Request::builder()
        .method("POST")
        .uri("/api/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(invalid_payload.to_string()))
        .unwrap();

    let (status, body) = send_request(app, req).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let err_json: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(err_json["code"], 400);
    assert_eq!(err_json["success"], false);
    assert!(err_json["error"]
        .as_str()
        .unwrap()
        .contains("비어 있을 수 없습니다"));
}

#[tokio::test]
async fn test_get_not_found_todo() {
    let state = AppState::new();
    let app = create_app(state);

    // 존재하지 않는 999번 조회 (GET /api/todos/999)
    let req = Request::builder()
        .method("GET")
        .uri("/api/todos/999")
        .body(Body::empty())
        .unwrap();

    let (status, body) = send_request(app, req).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    let err_json: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(err_json["code"], 404);
    assert_eq!(err_json["success"], false);
}

#[tokio::test]
async fn test_update_and_delete_todo() {
    let state = AppState::new();
    let app = create_app(state);

    // 1. 초기 Todo 등록
    let create_payload = json!({
        "title": "삭제 예정 할 일"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(create_payload.to_string()))
        .unwrap();
    let (status, _) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::CREATED);

    // 2. 완료 상태로 수정 (PUT /api/todos/1)
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

    // 3. Todo 삭제 (DELETE /api/todos/1)
    let req = Request::builder()
        .method("DELETE")
        .uri("/api/todos/1")
        .body(Body::empty())
        .unwrap();
    let (status, _) = send_request(app.clone(), req).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    // 4. 삭제 확인 (재조회 시 404 Not Found)
    let req = Request::builder()
        .method("GET")
        .uri("/api/todos/1")
        .body(Body::empty())
        .unwrap();
    let (status, _) = send_request(app, req).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

