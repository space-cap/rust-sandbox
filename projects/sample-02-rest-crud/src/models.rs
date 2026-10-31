use serde::{Deserialize, Serialize};

/// Todo 엔티티 모델
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

/// Todo 생성 요청 DTO
#[derive(Debug, Deserialize)]
pub struct CreateTodoDto {
    pub title: String,
    pub description: Option<String>,
}

/// Todo 수정 요청 DTO
#[derive(Debug, Deserialize)]
pub struct UpdateTodoDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
