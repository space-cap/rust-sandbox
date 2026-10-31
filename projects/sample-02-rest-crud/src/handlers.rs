use crate::error::AppError;
use crate::models::{CreateTodoDto, Todo, UpdateTodoDto};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::sync::RwLock;
use tracing::info;

/// 멀티스레드 환경에서 안전하게 공유되는 애플리케이션 상태
#[derive(Clone, Default)]
pub struct AppState {
    pub db: Arc<RwLock<HashMap<u64, Todo>>>,
    pub next_id: Arc<AtomicU64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

/// 1. 전체 Todo 목록 조회: GET /api/todos
pub async fn list_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    let store = state.db.read().await;
    let mut todos: Vec<Todo> = store.values().cloned().collect();
    // ID 순서대로 정렬
    todos.sort_by_key(|t| t.id);

    info!("전체 Todo 조회 (총 {}건)", todos.len());
    Ok(Json(todos))
}

/// 2. 단건 Todo 상세 조회: GET /api/todos/{id}
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, AppError> {
    let store = state.db.read().await;
    let todo = store
        .get(&id)
        .ok_or_else(|| AppError::NotFound(format!("ID {}번 할 일을 찾을 수 없습니다.", id)))?;

    info!("단건 Todo 조회 성공: ID={}", id);
    Ok(Json(todo.clone()))
}

/// 3. 신규 Todo 생성: POST /api/todos
pub async fn create_todo(
    State(state): State<AppState>,
    Json(dto): Json<CreateTodoDto>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    if dto.title.trim().is_empty() {
        return Err(AppError::BadRequest("할 일 제목(title)은 비어 있을 수 없습니다.".into()));
    }

    let id = state.next_id.fetch_add(1, Ordering::SeqCst);
    let new_todo = Todo {
        id,
        title: dto.title,
        description: dto.description,
        completed: false,
    };

    {
        let mut store = state.db.write().await;
        store.insert(id, new_todo.clone());
    }

    info!("신규 Todo 등록 완료: ID={}", id);
    Ok((StatusCode::CREATED, Json(new_todo)))
}

/// 4. Todo 수정: PUT /api/todos/{id}
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(dto): Json<UpdateTodoDto>,
) -> Result<Json<Todo>, AppError> {
    let mut store = state.db.write().await;
    let todo = store
        .get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("수정할 ID {}번 할 일이 존재하지 않습니다.", id)))?;

    if let Some(title) = dto.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("수정할 제목은 비어 있을 수 없습니다.".into()));
        }
        todo.title = title;
    }

    if let Some(description) = dto.description {
        todo.description = Some(description);
    }

    if let Some(completed) = dto.completed {
        todo.completed = completed;
    }

    info!("Todo 수정 완료: ID={}", id);
    Ok(Json(todo.clone()))
}

/// 5. Todo 삭제: DELETE /api/todos/{id}
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, AppError> {
    let mut store = state.db.write().await;
    if store.remove(&id).is_some() {
        info!("Todo 삭제 완료: ID={}", id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("삭제할 ID {}번 할 일을 찾을 수 없습니다.", id)))
    }
}

