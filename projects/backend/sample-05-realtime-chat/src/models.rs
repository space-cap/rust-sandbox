use serde::{Deserialize, Serialize};

/// 메시지 종류 (일반 대화 vs 시스템 알림)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    User,
    System,
}

/// 웹소켓을 통해 실시간으로 오가는 채팅 메시지 규격
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
    pub msg_type: MessageType,
    pub timestamp: String,
}

/// 웹소켓 연결 시 쿼리 파라미터 (/ws?username=...)
#[derive(Debug, Deserialize)]
pub struct WsQuery {
    pub username: Option<String>,
}

/// 서버 상태 통계 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStats {
    pub active_users: usize,
    pub channel_capacity: usize,
    pub online_list: Vec<String>,
}

