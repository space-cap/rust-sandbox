use std::fs;
use std::path::Path;
use crate::models::TodoItem;

pub const DEFAULT_STORAGE_PATH: &str = "todos.json";

/// 할 일 목록을 로컬 JSON 파일로 영구 저장합니다.
pub fn save_todos<P: AsRef<Path>>(path: P, todos: &[TodoItem]) -> Result<(), std::io::Error> {
    let json_data = serde_json::to_string_pretty(todos)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    fs::write(path, json_data)
}

/// 로컬 JSON 파일로부터 할 일 목록을 불러옵니다. 파일이 없으면 기본 샘플을 반환합니다.
pub fn load_todos<P: AsRef<Path>>(path: P) -> Result<Vec<TodoItem>, std::io::Error> {
    let path_ref = path.as_ref();
    if !path_ref.exists() {
        return Ok(get_initial_sample_todos());
    }

    let json_data = fs::read_to_string(path_ref)?;
    let todos: Vec<TodoItem> = serde_json::from_str(&json_data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(todos)
}

/// 최초 실행 시 사용자에게 표시할 안내용 샘플 Todo 목록
pub fn get_initial_sample_todos() -> Vec<TodoItem> {
    vec![
        TodoItem::new(1, "Rust Iced 보유 모드(Retained Mode) 개념 익히기".to_string()),
        TodoItem::new(2, "Elm 아키텍처 (Model-View-Update) 단방향 흐름 체험하기".to_string()),
        {
            let mut done = TodoItem::new(3, "egui 즉시 모드(Immediate Mode) 마스터하기".to_string());
            done.is_completed = true;
            done
        },
    ]
}

