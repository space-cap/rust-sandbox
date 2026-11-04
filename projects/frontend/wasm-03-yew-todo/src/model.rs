use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn label(&self) -> &'static str {
        match self {
            Filter::All => "전체 (All)",
            Filter::Active => "진행 중 (Active)",
            Filter::Completed => "완료됨 (Done)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoList {
    pub items: Vec<Todo>,
    pub next_id: u64,
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }

    /// 기본 데모용 샘플 목록 생성
    pub fn with_samples() -> Self {
        let mut list = Self::new();
        list.add("🦀 Rust WebAssembly 기초 다지기");
        list.add("⚛️ Yew 컴포넌트 라이프사이클과 Hooks 정복");
        list.add("💾 LocalStorage로 브라우저 새로고침해도 상태 유지");
        if let Some(first) = list.items.first_mut() {
            first.completed = true;
        }
        list
    }

    /// 새 할 일 추가 (공백 문자열은 무시)
    pub fn add(&mut self, title: &str) -> bool {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return false;
        }

        self.items.push(Todo {
            id: self.next_id,
            title: trimmed.to_string(),
            completed: false,
        });
        self.next_id += 1;
        true
    }

    /// 완료 여부 토글
    pub fn toggle(&mut self, id: u64) {
        if let Some(todo) = self.items.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    }

    /// 할 일 내용 수정
    pub fn edit(&mut self, id: u64, new_title: &str) -> bool {
        let trimmed = new_title.trim();
        if trimmed.is_empty() {
            // 빈 내용으로 수정 시 삭제 처리
            self.remove(id);
            return true;
        }

        if let Some(todo) = self.items.iter_mut().find(|t| t.id == id) {
            todo.title = trimmed.to_string();
            true
        } else {
            false
        }
    }

    /// 할 일 삭제
    pub fn remove(&mut self, id: u64) {
        self.items.retain(|t| t.id != id);
    }

    /// 전체 일괄 완료/미완료 토글
    pub fn toggle_all(&mut self, completed: bool) {
        for todo in &mut self.items {
            todo.completed = completed;
        }
    }

    /// 완료된 항목 일괄 삭제
    pub fn clear_completed(&mut self) {
        self.items.retain(|t| !t.completed);
    }

    pub fn total_count(&self) -> usize {
        self.items.len()
    }

    pub fn remaining_count(&self) -> usize {
        self.items.iter().filter(|t| !t.completed).count()
    }

    pub fn completed_count(&self) -> usize {
        self.items.iter().filter(|t| t.completed).count()
    }

    /// 필터 조건에 맞는 항목 반환
    pub fn filtered(&self, filter: Filter) -> Vec<&Todo> {
        match filter {
            Filter::All => self.items.iter().collect(),
            Filter::Active => self.items.iter().filter(|t| !t.completed).collect(),
            Filter::Completed => self.items.iter().filter(|t| t.completed).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_add_and_validation() {
        let mut list = TodoList::new();
        assert!(!list.add("   "));
        assert_eq!(list.total_count(), 0);

        assert!(list.add("러스트 프론트엔드 공부"));
        assert_eq!(list.total_count(), 1);
        assert_eq!(list.items[0].title, "러스트 프론트엔드 공부");
        assert!(!list.items[0].completed);
    }

    #[test]
    fn test_todo_toggle_and_remove() {
        let mut list = TodoList::new();
        list.add("테스크 1");
        let id = list.items[0].id;

        list.toggle(id);
        assert!(list.items[0].completed);
        assert_eq!(list.completed_count(), 1);
        assert_eq!(list.remaining_count(), 0);

        list.remove(id);
        assert_eq!(list.total_count(), 0);
    }

    #[test]
    fn test_todo_edit() {
        let mut list = TodoList::new();
        list.add("원래 제목");
        let id = list.items[0].id;

        list.edit(id, "수정된 제목");
        assert_eq!(list.items[0].title, "수정된 제목");

        // 빈 내용으로 수정 시 삭제
        list.edit(id, "   ");
        assert_eq!(list.total_count(), 0);
    }

    #[test]
    fn test_todo_filtering_and_batch_actions() {
        let mut list = TodoList::new();
        list.add("항목 A");
        list.add("항목 B");
        list.add("항목 C");

        let id_b = list.items[1].id;
        list.toggle(id_b);

        assert_eq!(list.filtered(Filter::All).len(), 3);
        assert_eq!(list.filtered(Filter::Active).len(), 2);
        assert_eq!(list.filtered(Filter::Completed).len(), 1);

        // 일괄 완료 처리
        list.toggle_all(true);
        assert_eq!(list.completed_count(), 3);

        // 완료 항목 일괄 삭제
        list.clear_completed();
        assert_eq!(list.total_count(), 0);
    }
}
