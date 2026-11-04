use desktop_03_iced_todo::models::{Filter, Message, TodoItem};
use desktop_03_iced_todo::storage::{load_todos, save_todos};
use desktop_03_iced_todo::TodoApp;

#[test]
fn test_todo_item_creation() {
    let item = TodoItem::new(1, "Rust Iced 공부하기".to_string());
    assert_eq!(item.id, 1);
    assert_eq!(item.title, "Rust Iced 공부하기");
    assert!(!item.is_completed);
    assert!(!item.is_editing);
    assert!(!item.created_at.is_empty());
}

#[test]
fn test_filter_matching() {
    let mut active_item = TodoItem::new(1, "진행 중 항목".to_string());
    active_item.is_completed = false;

    let mut completed_item = TodoItem::new(2, "완료 항목".to_string());
    completed_item.is_completed = true;

    // Filter::All
    assert!(Filter::All.matches(&active_item));
    assert!(Filter::All.matches(&completed_item));

    // Filter::Active
    assert!(Filter::Active.matches(&active_item));
    assert!(!Filter::Active.matches(&completed_item));

    // Filter::Completed
    assert!(!Filter::Completed.matches(&active_item));
    assert!(Filter::Completed.matches(&completed_item));

    // Filter labels
    assert_eq!(Filter::all().len(), 3);
}

#[test]
fn test_elm_update_add_todo() {
    let mut app = TodoApp::default();
    let initial_count = app.todos.len();

    // 1. 입력창 텍스트 변경
    let _ = app.update(Message::InputChanged("새로운 Iced 할 일 추가".to_string()));
    assert_eq!(app.input_value, "새로운 Iced 할 일 추가");

    // 2. 추가 메시지 발행
    let _ = app.update(Message::AddTodo);
    assert_eq!(app.todos.len(), initial_count + 1);
    assert_eq!(app.input_value, ""); // 입력창 자동 클리어
    assert_eq!(app.todos.last().unwrap().title, "새로운 Iced 할 일 추가");

    // 3. 공백 문자열 추가 시도 (무시되어야 함)
    let _ = app.update(Message::InputChanged("   ".to_string()));
    let _ = app.update(Message::AddTodo);
    assert_eq!(app.todos.len(), initial_count + 1);
}

#[test]
fn test_elm_update_toggle_todo() {
    let mut app = TodoApp::default();
    app.todos.clear();
    app.todos.push(TodoItem::new(100, "토글 대상".to_string()));

    assert!(!app.todos[0].is_completed);

    // 완료로 토글
    let _ = app.update(Message::ToggleTodo(100));
    assert!(app.todos[0].is_completed);

    // 다시 미완료로 토글
    let _ = app.update(Message::ToggleTodo(100));
    assert!(!app.todos[0].is_completed);
}

#[test]
fn test_elm_update_inline_editing() {
    let mut app = TodoApp::default();
    app.todos.clear();
    app.todos.push(TodoItem::new(200, "원래 제목".to_string()));

    // 1. 수정 모드 진입
    let _ = app.update(Message::StartEditing(200));
    assert!(app.todos[0].is_editing);
    assert_eq!(app.todos[0].edit_buffer, "원래 제목");

    // 2. 수정 버퍼 변경
    let _ = app.update(Message::EditInputChanged(200, "수정 완료된 새 제목".to_string()));
    assert_eq!(app.todos[0].edit_buffer, "수정 완료된 새 제목");

    // 3. 수정 저장
    let _ = app.update(Message::FinishEditing(200));
    assert!(!app.todos[0].is_editing);
    assert_eq!(app.todos[0].title, "수정 완료된 새 제목");

    // 4. 취소 테스트
    let _ = app.update(Message::StartEditing(200));
    let _ = app.update(Message::EditInputChanged(200, "취소할 텍스트".to_string()));
    let _ = app.update(Message::CancelEditing(200));
    assert!(!app.todos[0].is_editing);
    assert_eq!(app.todos[0].title, "수정 완료된 새 제목");
}

#[test]
fn test_elm_update_delete_and_clear_completed() {
    let mut app = TodoApp::default();
    app.todos.clear();
    let mut t1 = TodoItem::new(1, "1번 (완료)".to_string());
    t1.is_completed = true;
    let t2 = TodoItem::new(2, "2번 (진행 중)".to_string());
    let mut t3 = TodoItem::new(3, "3번 (완료)".to_string());
    t3.is_completed = true;

    app.todos.extend(vec![t1, t2, t3]);

    // 단일 삭제
    let _ = app.update(Message::DeleteTodo(2));
    assert_eq!(app.todos.len(), 2);
    assert!(app.todos.iter().all(|t| t.id != 2));

    // 완료 항목 일괄 정리
    let _ = app.update(Message::ClearCompleted);
    assert_eq!(app.todos.len(), 0);
}

#[test]
fn test_storage_json_roundtrip() {
    let temp_path = std::env::temp_dir().join("test_iced_todos.json");

    let original = vec![
        TodoItem::new(1, "저장 테스트 1".to_string()),
        {
            let mut done = TodoItem::new(2, "저장 테스트 2".to_string());
            done.is_completed = true;
            done
        },
    ];

    save_todos(&temp_path, &original).expect("JSON 저장 실패");
    let loaded = load_todos(&temp_path).expect("JSON 로드 실패");

    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[0].id, 1);
    assert_eq!(loaded[0].title, "저장 테스트 1");
    assert!(!loaded[0].is_completed);
    assert!(loaded[1].is_completed);

    // 임시 파일 정리
    let _ = std::fs::remove_file(&temp_path);
}
