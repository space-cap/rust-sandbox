pub mod error;
pub mod handlers;
pub mod models;

use axum::{
    response::Html,
    routing::get,
    Router,
};
use handlers::{create_todo, delete_todo, get_todo, list_todos, update_todo, AppState};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

/// 라우터를 생성하고 데이터베이스 풀 상태를 주입합니다.
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/api/todos", get(list_todos).post(create_todo))
        .route(
            "/api/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .with_state(state)
}

/// 데이터베이스 커넥션 풀을 초기화하고 마이그레이션을 자동으로 수행합니다.
pub async fn setup_database(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    // migrations 폴더의 SQL 스크립트를 자동 실행하여 스키마 동기화!
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>🦀 Sample 03: SQLx 영구 데이터베이스 API 서버</h1><p>SQLite 기반의 비동기 CRUD가 가동 중입니다!</p>")
}

