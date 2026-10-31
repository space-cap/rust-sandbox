use crate::error::AppError;
use crate::models::{CreateTodoDto, Todo, UpdateTodoDto};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use tracing::info;

/// 비동기 데이터베이스 커넥션 풀을 보관하는 전역 상태
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

/// 1. 전체 Todo 목록 조회: GET /api/todos
pub async fn list_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at FROM todos ORDER BY id ASC",
    )
    .fetch_all(&state.pool)
    .await?;

    info!("DB에서 Todo 전체 조회: 총 {}건", todos.len());
    Ok(Json(todos))
}

/// 2. 단건 Todo 상세 조회: GET /api/todos/{id}
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, AppError> {
    let todo = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at FROM todos WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("ID {}번 할 일을 DB에서 찾을 수 없습니다.", id)))?;

    info!("DB 단건 Todo 조회 성공: ID={}", id);
    Ok(Json(todo))
}

/// 3. 신규 Todo 등록: POST /api/todos
pub async fn create_todo(
    State(state): State<AppState>,
    Json(dto): Json<CreateTodoDto>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    if dto.title.trim().is_empty() {
        return Err(AppError::BadRequest("할 일 제목(title)은 비어 있을 수 없습니다.".into()));
    }

    let result = sqlx::query(
        "INSERT INTO todos (title, description, completed) VALUES (?, ?, 0)",
    )
    .bind(&dto.title)
    .bind(&dto.description)
    .execute(&state.pool)
    .await?;

    let inserted_id = result.last_insert_rowid();

    // 생성된 최신 행을 조회하여 반환
    let new_todo = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at FROM todos WHERE id = ?",
    )
    .bind(inserted_id)
    .fetch_one(&state.pool)
    .await?;

    info!("DB 신규 Todo 생성 완료: ID={}", inserted_id);
    Ok((StatusCode::CREATED, Json(new_todo)))
}

/// 4. Todo 내용 및 상태 수정: PUT /api/todos/{id}
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(dto): Json<UpdateTodoDto>,
) -> Result<Json<Todo>, AppError> {
    // 1. 기존 데이터 존재 확인
    let current = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at FROM todos WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("수정할 ID {}번 할 일이 존재하지 않습니다.", id)))?;

    // 2. 수정 값 결정
    let new_title = match dto.title {
        Some(t) => {
            if t.trim().is_empty() {
                return Err(AppError::BadRequest("수정할 제목은 비어 있을 수 없습니다.".into()));
            }
            t
        }
        None => current.title,
    };
    let new_desc = dto.description.or(current.description);
    let new_completed = dto.completed.unwrap_or(current.completed);

    // 3. UPDATE 쿼리 실행
    sqlx::query(
        "UPDATE todos SET title = ?, description = ?, completed = ? WHERE id = ?",
    )
    .bind(&new_title)
    .bind(&new_desc)
    .bind(new_completed)
    .bind(id)
    .execute(&state.pool)
    .await?;

    // 4. 업데이트된 레코드 반환
    let updated = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at FROM todos WHERE id = ?",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    info!("DB Todo 수정 완료: ID={}", id);
    Ok(Json(updated))
}

/// 5. Todo 삭제: DELETE /api/todos/{id}
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        Err(AppError::NotFound(format!("삭제할 ID {}번 할 일을 찾을 수 없습니다.", id)))
    } else {
        info!("DB Todo 삭제 완료: ID={}", id);
        Ok(StatusCode::NO_CONTENT)
    }
}

