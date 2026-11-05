use chrono::Local;
use serde::{Deserialize, Serialize};

/// 할 일 항목의 완료 상태 필터
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn label(&self) -> &'static str {
        match self {
            Self::All => "전체",
            Self::Active => "진행 중",
            Self::Completed => "완료",
        }
    }

    pub fn matches(&self, item: &TodoItem) -> bool {
        match self {
            Self::All => true,
            Self::Active => !item.is_completed,
            Self::Completed => item.is_completed,
        }
    }

    pub fn all() -> [Self; 3] {
        [Self::All, Self::Active, Self::Completed]
    }
}

/// 개별 할 일 데이터 모델
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub is_completed: bool,
    pub created_at: String,
    #[serde(skip)]
    pub is_editing: bool,
    #[serde(skip)]
    pub edit_buffer: String,
}

impl TodoItem {
    pub fn new(id: usize, title: String) -> Self {
        let created_at = Local::now().format("%m/%d %H:%M").to_string();
        Self {
            id,
            title,
            is_completed: false,
            created_at,
            is_editing: false,
            edit_buffer: String::new(),
        }
    }
}

/// Elm 아키텍처의 핵심: 사용자의 모든 인터랙션 이벤트 열거형
#[derive(Debug, Clone)]
pub enum Message {
    /// 새 할 일 텍스트 입력창 값 변경
    InputChanged(String),
    /// 새 할 일 추가 버튼 클릭 또는 엔터
    AddTodo,
    /// 특정 항목 완료 여부 체크박스 토글
    ToggleTodo(usize),
    /// 특정 항목 삭제
    DeleteTodo(usize),
    /// 특정 항목 인라인 수정 모드 진입
    StartEditing(usize),
    /// 인라인 수정 입력창 값 변경
    EditInputChanged(usize, String),
    /// 인라인 수정 완료 저장
    FinishEditing(usize),
    /// 인라인 수정 취소
    CancelEditing(usize),
    /// 필터 선택 (전체 / 진행 중 / 완료)
    FilterSelected(Filter),
    /// 완료된 모든 항목 일괄 삭제
    ClearCompleted,
    /// 다크/라이트 테마 전환
    ToggleTheme,
}

