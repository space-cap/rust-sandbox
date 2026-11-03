use crate::models::{ChatMessage, MessageType, ServerStats, WsQuery};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::{Html, IntoResponse},
    Json,
};
use futures_util::{SinkExt, StreamExt};
use std::{
    collections::HashSet,
    sync::Arc,
};
use tokio::sync::{broadcast, RwLock};
use tracing::info;

/// 실시간 브로드캐스트 채널 및 동시 접속자 상태
#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<String>,
    pub users: Arc<RwLock<HashSet<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        // 최대 100개의 버퍼를 갖는 1:N 브로드캐스트 채널 생성
        let (tx, _rx) = broadcast::channel(100);
        Self {
            tx,
            users: Arc::new(RwLock::new(HashSet::new())),
        }
    }
}

/// 1. 웹소켓 업그레이드 핸들러: GET /ws?username=...
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let username = query
        .username
        .map(|u| u.trim().to_string())
        .filter(|u| !u.is_empty())
        .unwrap_or_else(|| {
            let id = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
                % 1000;
            format!("익명_{id}")
        });

    info!("새 웹소켓 접속 시도: username='{}'", username);

    ws.on_upgrade(move |socket| handle_socket(socket, state, username))
}

/// 2. 개별 웹소켓 연결 처리 로직
async fn handle_socket(socket: WebSocket, state: AppState, username: String) {
    // 1) 접속자 목록에 추가
    {
        let mut users = state.users.write().await;
        users.insert(username.clone());
    }

    // 2) 송신(Send)과 수신(Receive) 스트림 분리 및 채널 사전 구독
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // 3) 전체 방에 입장 안내 메시지 브로드캐스트 (사전 구독했으므로 본인에게도 전송됨)
    let join_msg = ChatMessage {
        sender: "시스템".into(),
        content: format!("🎉 '{}'님이 채팅방에 입장하셨습니다!", username),
        msg_type: MessageType::System,
        timestamp: current_timestamp(),
    };
    let _ = state.tx.send(serde_json::to_string(&join_msg).unwrap());

    // 4) 브로드캐스트 수신 태스크 (채널 -> 클라이언트 소켓)
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if ws_sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // 5) 클라이언트 메시지 수신 태스크 (클라이언트 소켓 -> 채널)
    let tx = state.tx.clone();
    let user_clone = username.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(text) => {
                    let chat = ChatMessage {
                        sender: user_clone.clone(),
                        content: text.to_string(),
                        msg_type: MessageType::User,
                        timestamp: current_timestamp(),
                    };
                    let _ = tx.send(serde_json::to_string(&chat).unwrap());
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // 둘 중 하나의 태스크가 종료(연결 끊김)되면 나머지 하나도 정리
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // 6) 퇴장 처리: 접속자 목록에서 제거 & 퇴장 안내 브로드캐스트
    {
        let mut users = state.users.write().await;
        users.remove(&username);
    }

    let leave_msg = ChatMessage {
        sender: "시스템".into(),
        content: format!("👋 '{}'님이 퇴장하셨습니다.", username),
        msg_type: MessageType::System,
        timestamp: current_timestamp(),
    };
    let _ = state.tx.send(serde_json::to_string(&leave_msg).unwrap());
    info!("웹소켓 연결 종료: username='{}'", username);
}

/// 3. 현재 접속자 통계 API: GET /api/stats
pub async fn stats_handler(State(state): State<AppState>) -> Json<ServerStats> {
    let users = state.users.read().await;
    let list: Vec<String> = users.iter().cloned().collect();
    Json(ServerStats {
        active_users: list.len(),
        channel_capacity: 100,
        online_list: list,
    })
}

/// 4. 반응형 웹 채팅 UI: GET /
pub async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

fn current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let hours = (now / 3600) % 24;
    let minutes = (now / 60) % 60;
    let seconds = now % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

