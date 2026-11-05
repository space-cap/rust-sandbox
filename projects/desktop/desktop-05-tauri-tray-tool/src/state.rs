//! 스레드 안전한 인메모리 상주 상태 관리
use crate::models::{ClipboardNote, ResidentAppState, TimerStatus};
use chrono::Local;
use std::sync::Mutex;

pub struct AppState {
    inner: Mutex<InnerState>,
}

struct InnerState {
    notification_count: usize,
    shortcut_pressed_count: usize,
    shortcut_binding: String,
    is_window_visible: bool,
    timer: TimerStatus,
    notes: Vec<ClipboardNote>,
    next_note_id: usize,
    single_instance_events: Vec<String>,
}

impl AppState {
    pub fn new() -> Self {
        let default_notes = vec![
            ClipboardNote {
                id: 1,
                text: "Tauri 2.0 단축키: Ctrl+Shift+U (창 토글)".to_string(),
                created_at: Local::now().format("%H:%M:%S").to_string(),
            },
            ClipboardNote {
                id: 2,
                text: "트레이 우클릭 컨텍스트 메뉴로 백그라운드 제어 가능".to_string(),
                created_at: Local::now().format("%H:%M:%S").to_string(),
            },
        ];

        Self {
            inner: Mutex::new(InnerState {
                notification_count: 0,
                shortcut_pressed_count: 0,
                shortcut_binding: "Ctrl+Shift+U".to_string(),
                is_window_visible: true,
                timer: TimerStatus::default(),
                notes: default_notes,
                next_note_id: 3,
                single_instance_events: Vec::new(),
            }),
        }
    }

    pub fn increment_notification(&self) -> usize {
        let mut lock = self.inner.lock().unwrap();
        lock.notification_count += 1;
        lock.notification_count
    }

    pub fn increment_shortcut(&self) -> usize {
        let mut lock = self.inner.lock().unwrap();
        lock.shortcut_pressed_count += 1;
        lock.shortcut_pressed_count
    }

    pub fn set_window_visibility(&self, visible: bool) {
        let mut lock = self.inner.lock().unwrap();
        lock.is_window_visible = visible;
    }

    pub fn start_timer(&self, secs: u64, label: String) -> TimerStatus {
        let mut lock = self.inner.lock().unwrap();
        lock.timer = TimerStatus {
            total_secs: secs,
            remaining_secs: secs,
            is_running: true,
            label,
        };
        lock.timer.clone()
    }

    pub fn stop_timer(&self) -> TimerStatus {
        let mut lock = self.inner.lock().unwrap();
        lock.timer.is_running = false;
        lock.timer.remaining_secs = 0;
        lock.timer.label = "중지됨".to_string();
        lock.timer.clone()
    }

    /// 1초마다 호출되어 타이머를 감소시키고, 타이머 완료 시 완료 레이블을 반환합니다.
    pub fn tick_timer(&self) -> Option<String> {
        let mut lock = self.inner.lock().unwrap();
        if lock.timer.is_running && lock.timer.remaining_secs > 0 {
            lock.timer.remaining_secs -= 1;
            if lock.timer.remaining_secs == 0 {
                lock.timer.is_running = false;
                return Some(lock.timer.label.clone());
            }
        }
        None
    }

    pub fn add_note(&self, text: String) -> Vec<ClipboardNote> {
        let mut lock = self.inner.lock().unwrap();
        let id = lock.next_note_id;
        lock.next_note_id += 1;

        lock.notes.insert(
            0,
            ClipboardNote {
                id,
                text,
                created_at: Local::now().format("%H:%M:%S").to_string(),
            },
        );

        if lock.notes.len() > 30 {
            lock.notes.truncate(30);
        }

        lock.notes.clone()
    }

    pub fn get_notes(&self) -> Vec<ClipboardNote> {
        let lock = self.inner.lock().unwrap();
        lock.notes.clone()
    }

    pub fn delete_note(&self, id: usize) -> Vec<ClipboardNote> {
        let mut lock = self.inner.lock().unwrap();
        lock.notes.retain(|n| n.id != id);
        lock.notes.clone()
    }

    pub fn clear_notes(&self) -> Vec<ClipboardNote> {
        let mut lock = self.inner.lock().unwrap();
        lock.notes.clear();
        lock.notes.clone()
    }

    pub fn record_single_instance_event(&self, args: Vec<String>) {
        let mut lock = self.inner.lock().unwrap();
        let timestamp = Local::now().format("%H:%M:%S").to_string();
        let entry = format!(
            "[{}] 중복 실행 차단 (인수: {})",
            timestamp,
            if args.is_empty() {
                "없음".to_string()
            } else {
                args.join(" ")
            }
        );
        lock.single_instance_events.insert(0, entry);
        if lock.single_instance_events.len() > 20 {
            lock.single_instance_events.truncate(20);
        }
    }

    pub fn get_summary(&self) -> ResidentAppState {
        let lock = self.inner.lock().unwrap();
        ResidentAppState {
            notification_count: lock.notification_count,
            shortcut_pressed_count: lock.shortcut_pressed_count,
            shortcut_binding: lock.shortcut_binding.clone(),
            is_window_visible: lock.is_window_visible,
            timer: lock.timer.clone(),
            notes: lock.notes.clone(),
            single_instance_events: lock.single_instance_events.clone(),
        }
    }
}

