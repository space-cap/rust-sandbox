use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use futures_util::{SinkExt, StreamExt};
use http_body_util::BodyExt;
use sample_05_realtime_chat::{create_app, handlers::AppState, models::{ChatMessage, MessageType, ServerStats}};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as TungsteniteMessage};
use tower::ServiceExt;

/// 1. 정적 HTML UI 제공 검증
#[tokio::test]
async fn test_index_html_served() {
    let app = create_app(AppState::new());
    let req = Request::builder().uri("/").body(Body::empty()).unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(html.contains("Rust Axum 실시간 웹소켓"));
}

/// 2. 접속자 통계 API 검증
#[tokio::test]
async fn test_stats_api() {
    let app = create_app(AppState::new());
    let req = Request::builder().uri("/api/stats").body(Body::empty()).unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let stats: ServerStats = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(stats.active_users, 0);
    assert_eq!(stats.channel_capacity, 100);
}

/// 3. 실시간 웹소켓 E2E 양방향 브로드캐스트 테스트!
#[tokio::test]
async fn test_websocket_chat_broadcast() {
    let state = AppState::new();
    let app = create_app(state);

    // 임의의 사용 가능한 로컬 포트에 서버 바인딩
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    // 백그라운드 태스크로 Axum 서버 실행
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 1) 클라이언트 1 (Alice) 접속
    let alice_url = format!("ws://127.0.0.1:{port}/ws?username=Alice");
    let (mut alice_ws, _) = connect_async(&alice_url).await.expect("Alice 연결 실패");

    // Alice 입장 알림 수신 확인
    let first_msg = alice_ws.next().await.unwrap().unwrap();
    let chat: ChatMessage = serde_json::from_str(&first_msg.to_text().unwrap()).unwrap();
    assert_eq!(chat.msg_type, MessageType::System);
    assert!(chat.content.contains("Alice"));

    // 2) 클라이언트 2 (Bob) 접속
    let bob_url = format!("ws://127.0.0.1:{port}/ws?username=Bob");
    let (mut bob_ws, _) = connect_async(&bob_url).await.expect("Bob 연결 실패");

    // Bob 입장 알림 수신 (Alice와 Bob 둘 다 수신)
    let _alice_recv_bob_join = alice_ws.next().await.unwrap().unwrap();
    let _bob_recv_self_join = bob_ws.next().await.unwrap().unwrap();

    // 3) Alice가 Bob에게 메시지 전송!
    alice_ws
        .send(TungsteniteMessage::Text("안녕 Bob! 러스트 웹소켓 정말 빠르다 🦀".into()))
        .await
        .expect("Alice 메시지 전송 실패");

    // 4) Bob이 Alice의 메시지를 수신하는지 검증!
    let bob_received = bob_ws.next().await.unwrap().unwrap();
    let chat_received: ChatMessage = serde_json::from_str(&bob_received.to_text().unwrap()).unwrap();

    assert_eq!(chat_received.sender, "Alice");
    assert_eq!(chat_received.content, "안녕 Bob! 러스트 웹소켓 정말 빠르다 🦀");
    assert_eq!(chat_received.msg_type, MessageType::User);
}

