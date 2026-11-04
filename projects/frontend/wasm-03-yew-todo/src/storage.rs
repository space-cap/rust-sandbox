use gloo_storage::{LocalStorage, Storage};
use crate::model::TodoList;

pub const STORAGE_KEY: &str = "rust_wasm_todos_v1";

/// 브라우저 LocalStorage에서 TodoList 불러오기 (실패 시 기본 샘플 목록 반환)
pub fn load_todos() -> TodoList {
    LocalStorage::get(STORAGE_KEY).unwrap_or_else(|_| TodoList::with_samples())
}

/// 브라우저 LocalStorage에 TodoList 저장하기
pub fn save_todos(todos: &TodoList) {
    let _ = LocalStorage::set(STORAGE_KEY, todos);
}

/// 로컬스토리지 초기화
pub fn clear_storage() {
    LocalStorage::delete(STORAGE_KEY);
}
