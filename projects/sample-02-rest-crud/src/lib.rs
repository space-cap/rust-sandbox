pub mod error;
pub mod handlers;
pub mod models;

use axum::{
    response::Html,
    routing::get,
    Router,
};
use handlers::{create_todo, delete_todo, get_todo, list_todos, update_todo, AppState};

/// 라우터를 생성하고 애플리케이션 상태를 주입하는 팩토리 함수
/// (메인 서버 실행 및 자동화 테스트 양쪽에서 재사용됩니다)
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

async fn root_handler() -> Html<&'static str> {
    Html("<h1>🦀 Sample 02: 견고한 RESTful CRUD API 서버</h1><p><code>/api/todos</code> 엔드포인트를 사용해 보세요!</p>")
}
