use iced::widget::{
    button, checkbox, column, container, horizontal_space, row, scrollable, text, text_input,
};
use iced::{Alignment, Element, Length, Theme};

use crate::models::{Filter, Message, TodoItem};
use crate::storage::{load_todos, save_todos, DEFAULT_STORAGE_PATH};

pub struct TodoApp {
    pub todos: Vec<TodoItem>,
    pub input_value: String,
    pub filter: Filter,
    pub next_id: usize,
    pub is_dark: bool,
    pub status_message: Option<String>,
}

impl Default for TodoApp {
    fn default() -> Self {
        let todos = load_todos(DEFAULT_STORAGE_PATH).unwrap_or_default();
        let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        Self {
            todos,
            input_value: String::new(),
            filter: Filter::All,
            next_id,
            is_dark: true,
            status_message: None,
        }
    }
}

impl TodoApp {
    pub fn new() -> (Self, iced::Task<Message>) {
        (Self::default(), iced::Task::none())
    }

    pub fn title(&self) -> String {
        let active_count = self.todos.iter().filter(|t| !t.is_completed).count();
        format!("Rust Desktop 03: Iced Elm Todo & 프로젝트 관리 🌲 (미완료: {active_count}개)")
    }

    pub fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    /// Elm Architecture의 Update: 메시지를 받아 상태를 변경하는 순수 진입점
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::InputChanged(val) => {
                self.input_value = val;
            }
            Message::AddTodo => {
                let trimmed = self.input_value.trim();
                if !trimmed.is_empty() {
                    let new_item = TodoItem::new(self.next_id, trimmed.to_string());
                    self.next_id += 1;
                    self.todos.push(new_item);
                    self.input_value.clear();
                    self.status_message = Some("새 할 일이 추가되었습니다.".to_string());
                    let _ = save_todos(DEFAULT_STORAGE_PATH, &self.todos);
                }
            }
            Message::ToggleTodo(id) => {
                if let Some(item) = self.todos.iter_mut().find(|t| t.id == id) {
                    item.is_completed = !item.is_completed;
                    let _ = save_todos(DEFAULT_STORAGE_PATH, &self.todos);
                }
            }
            Message::DeleteTodo(id) => {
                self.todos.retain(|t| t.id != id);
                self.status_message = Some("항목이 삭제되었습니다.".to_string());
                let _ = save_todos(DEFAULT_STORAGE_PATH, &self.todos);
            }
            Message::StartEditing(id) => {
                for item in &mut self.todos {
                    if item.id == id {
                        item.is_editing = true;
                        item.edit_buffer = item.title.clone();
                    } else {
                        item.is_editing = false;
                    }
                }
            }
            Message::EditInputChanged(id, new_val) => {
                if let Some(item) = self.todos.iter_mut().find(|t| t.id == id) {
                    item.edit_buffer = new_val;
                }
            }
            Message::FinishEditing(id) => {
                if let Some(item) = self.todos.iter_mut().find(|t| t.id == id) {
                    let trimmed = item.edit_buffer.trim();
                    if !trimmed.is_empty() {
                        item.title = trimmed.to_string();
                    }
                    item.is_editing = false;
                    let _ = save_todos(DEFAULT_STORAGE_PATH, &self.todos);
                }
            }
            Message::CancelEditing(id) => {
                if let Some(item) = self.todos.iter_mut().find(|t| t.id == id) {
                    item.is_editing = false;
                }
            }
            Message::FilterSelected(filter) => {
                self.filter = filter;
            }
            Message::ClearCompleted => {
                self.todos.retain(|t| !t.is_completed);
                self.status_message = Some("완료된 항목이 일괄 정리되었습니다.".to_string());
                let _ = save_todos(DEFAULT_STORAGE_PATH, &self.todos);
            }
            Message::ToggleTheme => {
                self.is_dark = !self.is_dark;
            }
        }

        iced::Task::none()
    }

    /// Elm Architecture의 View: 상태를 읽어 선언적 위젯 트리를 구성
    pub fn view(&self) -> Element<'_, Message> {
        // 1. 상단 헤더 (제목 및 테마 전환)
        let header = row![
            text("🌲 Rust Iced 데스크톱 Todo")
                .size(24),
            horizontal_space(),
            button(if self.is_dark { "☀️ 라이트 모드" } else { "🌙 다크 모드" })
                .on_press(Message::ToggleTheme),
        ]
        .align_y(Alignment::Center);

        // 2. 새 할 일 입력 행
        let input_row = row![
            text_input("새로운 할 일을 입력하세요...", &self.input_value)
                .on_input(Message::InputChanged)
                .on_submit(Message::AddTodo)
                .padding(10)
                .size(16),
            button("➕ 추가")
                .padding([10, 16])
                .on_press(Message::AddTodo),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // 3. 필터 선택 탭 행
        let mut filter_row = row![text("필터:").size(15)].spacing(8).align_y(Alignment::Center);
        for filter_opt in Filter::all() {
            let is_selected = self.filter == filter_opt;
            let filter_btn = if is_selected {
                button(text(filter_opt.label()).size(14))
                    .padding([6, 12])
                    .on_press(Message::FilterSelected(filter_opt))
                    .style(button::primary)
            } else {
                button(text(filter_opt.label()).size(14))
                    .padding([6, 12])
                    .on_press(Message::FilterSelected(filter_opt))
                    .style(button::secondary)
            };
            filter_row = filter_row.push(filter_btn);
        }

        filter_row = filter_row.push(horizontal_space());
        if self.todos.iter().any(|t| t.is_completed) {
            filter_row = filter_row.push(
                button("🗑️ 완료 항목 정리")
                    .padding([6, 12])
                    .on_press(Message::ClearCompleted),
            );
        }

        // 4. 할 일 리스트 (Scrollable Column)
        let filtered_items: Vec<_> = self.todos.iter().filter(|t| self.filter.matches(t)).collect();

        let todo_list_col: Element<'_, Message> = if filtered_items.is_empty() {
            container(
                text("표시할 할 일 항목이 없습니다. 새로운 할 일을 추가해보세요! 🦀")
                    .size(15),
            )
            .padding(30)
            .center_x(Length::Fill)
            .into()
        } else {
            let mut list_col = column![].spacing(8);
            for item in filtered_items {
                list_col = list_col.push(self.render_todo_item(item));
            }
            scrollable(list_col).into()
        };

        // 5. 하단 상태 바
        let total_count = self.todos.len();
        let completed_count = self.todos.iter().filter(|t| t.is_completed).count();
        let active_count = total_count - completed_count;

        let status_bar = row![
            text(format!("전체: {total_count}개  |  진행 중: {active_count}개  |  완료: {completed_count}개"))
                .size(13),
            horizontal_space(),
            text("Elm 단방향 보유 모드 (Iced 0.13)").size(13),
        ]
        .align_y(Alignment::Center);

        // 전체 레이아웃 조합
        let content = column![
            header,
            input_row,
            filter_row,
            todo_list_col,
            horizontal_space(),
            status_bar,
        ]
        .spacing(16)
        .padding(20)
        .max_width(750);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }

    /// 개별 Todo 아이템 위젯 렌더링
    fn render_todo_item<'a>(&self, item: &'a TodoItem) -> Element<'a, Message> {
        let item_id = item.id;

        if item.is_editing {
            row![
                text_input("수정할 내용을 입력하세요...", &item.edit_buffer)
                    .on_input(move |val| Message::EditInputChanged(item_id, val))
                    .on_submit(Message::FinishEditing(item_id))
                    .padding(8)
                    .size(15),
                button("💾 저장").padding([6, 10]).on_press(Message::FinishEditing(item_id)),
                button("❌ 취소").padding([6, 10]).on_press(Message::CancelEditing(item_id)),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .into()
        } else {
            let check = checkbox("", item.is_completed)
                .on_toggle(move |_| Message::ToggleTodo(item_id));

            let title_text = if item.is_completed {
                text(format!("✓ {}", item.title)).size(16)
            } else {
                text(&item.title).size(16)
            };

            let date_label = text(&item.created_at).size(12);

            let edit_btn = button("✏️").padding([4, 8]).on_press(Message::StartEditing(item_id));
            let del_btn = button("🗑️").padding([4, 8]).on_press(Message::DeleteTodo(item_id));

            container(
                row![
                    check,
                    title_text,
                    horizontal_space(),
                    date_label,
                    edit_btn,
                    del_btn,
                ]
                .spacing(10)
                .align_y(Alignment::Center)
                .padding(10),
            )
            .into()
        }
    }
}
