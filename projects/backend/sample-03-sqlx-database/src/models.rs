use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 데이터베이스 `todos` 테이블과 1:1 매핑되는 엔티티 구조체
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String,
}

/// 신규 생성 요청 DTO
#[derive(Debug, Deserialize)]
pub struct CreateTodoDto {
    pub title: String,
    pub description: Option<String>,
}

/// 수정 요청 DTO
#[derive(Debug, Deserialize)]
pub struct UpdateTodoDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

