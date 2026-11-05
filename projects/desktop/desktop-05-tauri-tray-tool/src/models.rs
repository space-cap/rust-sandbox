//! 데이터 모델 정의: 트레이 유틸리티 상태, 타이머, 클립보드 메모 등
use serde::{Deserialize, Serialize};

/// 클립보드 퀵 메모 항목
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClipboardNote {
    pub id: usize,
    pub text: String,
    pub created_at: String,
}

/// 백그라운드 타이머 상태
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimerStatus {
    pub total_secs: u64,
    pub remaining_secs: u64,
    pub is_running: bool,
    pub label: String,
}

impl Default for TimerStatus {
    fn default() -> Self {
        Self {
            total_secs: 0,
            remaining_secs: 0,
            is_running: false,
            label: String::from("대기 중"),
        }
    }
}

/// 상주 애플리케이션 종합 상태 스냅샷
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidentAppState {
    pub notification_count: usize,
    pub shortcut_pressed_count: usize,
    pub shortcut_binding: String,
    pub is_window_visible: bool,
    pub timer: TimerStatus,
    pub notes: Vec<ClipboardNote>,
    pub single_instance_events: Vec<String>,
}

/// 토스트 알림 요청 파라미터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub title: String,
    pub body: String,
}

