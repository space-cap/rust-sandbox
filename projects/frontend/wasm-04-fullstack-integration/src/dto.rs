//! 백엔드(Axum)와 100% 동일하게 공유되는 DTO 및 데이터 모델
use serde::{Deserialize, Serialize};

/// 서버로부터 수신하는 Todo 엔티티 모델 (sample-02-rest-crud의 Todo 모델과 1:1 일치)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

/// 새 Todo 등록 요청 DTO (POST /api/todos)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateTodoDto {
    pub title: String,
    pub description: Option<String>,
}

/// Todo 수정 요청 DTO (PUT /api/todos/{id})
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateTodoDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}

/// 백엔드 서버와의 실시간 연결 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
}

impl ConnectionStatus {
    pub fn badge(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            ConnectionStatus::Connecting => ("badge-connecting", "🟡", "서버 확인 중..."),
            ConnectionStatus::Connected => ("badge-connected", "🟢", "Axum 서버 연결됨 (3001)"),
            ConnectionStatus::Disconnected => ("badge-disconnected", "🔴", "오프라인 (백엔드 미구동)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_json_roundtrip() {
        let original = Todo {
            id: 42,
            title: "Axum 풀스택 연동".to_string(),
            description: Some("gloo-net 비동기 통신".to_string()),
            completed: false,
        };

        let json = serde_json::to_string(&original).expect("직렬화 실패");
        let parsed: Todo = serde_json::from_str(&json).expect("역직렬화 실패");

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_create_dto_json() {
        let dto = CreateTodoDto {
            title: "새로운 작업".to_string(),
            description: None,
        };

        let json = serde_json::to_string(&dto).unwrap();
        assert!(json.contains("\"title\":\"새로운 작업\""));
        assert!(json.contains("\"description\":null"));
    }

    #[test]
    fn test_update_dto_selective_serialization() {
        let mut dto = UpdateTodoDto::default();
        dto.completed = Some(true);

        let json = serde_json::to_string(&dto).unwrap();
        assert_eq!(json, r#"{"completed":true}"#);
    }
}
