//! 비즈니스 로직 및 상태 관리 모델 (순수 Rust 단위 테스트 가능)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn class_name(&self) -> &'static str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Theme::Light => "☀️ 라이트 모드",
            Theme::Dark => "🌙 다크 모드",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryItem {
    pub id: usize,
    pub description: String,
    pub before: i32,
    pub after: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterState {
    pub count: i32,
    pub step: i32,
    pub history: Vec<HistoryItem>,
    pub next_id: usize,
}

impl Default for CounterState {
    fn default() -> Self {
        Self::new()
    }
}

impl CounterState {
    pub fn new() -> Self {
        Self {
            count: 0,
            step: 1,
            history: Vec::new(),
            next_id: 1,
        }
    }

    pub fn increment(&mut self) {
        let before = self.count;
        self.count += self.step;
        self.record_history(format!("+{step}", step = self.step), before, self.count);
    }

    pub fn decrement(&mut self) {
        let before = self.count;
        self.count -= self.step;
        self.record_history(format!("-{step}", step = self.step), before, self.count);
    }

    pub fn reset(&mut self) {
        if self.count == 0 {
            return;
        }
        let before = self.count;
        self.count = 0;
        self.record_history("초기화 (Reset)".to_string(), before, 0);
    }

    pub fn set_step(&mut self, step: i32) {
        if step > 0 {
            self.step = step;
        }
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    fn record_history(&mut self, description: String, before: i32, after: i32) {
        let item = HistoryItem {
            id: self.next_id,
            description,
            before,
            after,
        };
        self.next_id += 1;
        // 최신 내역이 상단에 오도록 prepend
        self.history.insert(0, item);
        // 최대 30개 기록 유지
        if self.history.len() > 30 {
            self.history.pop();
        }
    }

    pub fn status_badge(&self) -> (&'static str, &'static str) {
        if self.count > 0 {
            ("badge-positive", "양수 (Positive)")
        } else if self.count < 0 {
            ("badge-negative", "음수 (Negative)")
        } else {
            ("badge-zero", "영점 (Zero)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_initial_state() {
        let state = CounterState::new();
        assert_eq!(state.count, 0);
        assert_eq!(state.step, 1);
        assert!(state.history.is_empty());
    }

    #[test]
    fn test_increment_and_decrement() {
        let mut state = CounterState::new();
        state.increment();
        assert_eq!(state.count, 1);
        assert_eq!(state.history.len(), 1);
        assert_eq!(state.history[0].before, 0);
        assert_eq!(state.history[0].after, 1);

        state.set_step(5);
        state.increment();
        assert_eq!(state.count, 6);

        state.decrement();
        assert_eq!(state.count, 1);
    }

    #[test]
    fn test_reset_behavior() {
        let mut state = CounterState::new();
        state.increment();
        state.increment();
        assert_eq!(state.count, 2);

        state.reset();
        assert_eq!(state.count, 0);
        assert_eq!(state.history[0].description, "초기화 (Reset)");

        // 이미 0일 때 리셋하면 히스토리 중복 기록 안 됨
        let hist_len = state.history.len();
        state.reset();
        assert_eq!(state.history.len(), hist_len);
    }

    #[test]
    fn test_theme_toggle() {
        let theme = Theme::Dark;
        let next = theme.toggle();
        assert_eq!(next, Theme::Light);
        assert_eq!(next.class_name(), "theme-light");
        assert_eq!(next.toggle(), Theme::Dark);
    }
}

