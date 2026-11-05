//! Tauri IPC Commands: 트레이 제어, 알림 전송, 타이머, 메모 관리 등
use crate::models::{ClipboardNote, NotificationPayload, ResidentAppState, TimerStatus};
use crate::state::AppState;
use tauri::{AppHandle, State, WebviewWindow};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
pub fn get_app_status(state: State<'_, AppState>) -> ResidentAppState {
    state.get_summary()
}

#[tauri::command]
pub fn send_toast_notification(
    app: AppHandle,
    state: State<'_, AppState>,
    payload: NotificationPayload,
) -> Result<usize, String> {
    if payload.title.trim().is_empty() {
        return Err("알림 제목을 입력해주세요.".to_string());
    }

    let count = state.increment_notification();

    app.notification()
        .builder()
        .title(&payload.title)
        .body(&payload.body)
        .show()
        .map_err(|e| format!("네이티브 토스트 알림 발송 실패: {}", e))?;

    Ok(count)
}

#[tauri::command]
pub fn start_timer(
    state: State<'_, AppState>,
    secs: u64,
    label: String,
) -> Result<TimerStatus, String> {
    if secs == 0 {
        return Err("타이머 시간은 1초 이상이어야 합니다.".to_string());
    }
    let status = state.start_timer(secs, label);
    Ok(status)
}

#[tauri::command]
pub fn stop_timer(state: State<'_, AppState>) -> TimerStatus {
    state.stop_timer()
}

#[tauri::command]
pub fn add_clipboard_note(state: State<'_, AppState>, text: String) -> Result<Vec<ClipboardNote>, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("메모 내용을 입력해주세요.".to_string());
    }
    Ok(state.add_note(trimmed.to_string()))
}

#[tauri::command]
pub fn get_clipboard_notes(state: State<'_, AppState>) -> Vec<ClipboardNote> {
    state.get_notes()
}

#[tauri::command]
pub fn delete_clipboard_note(state: State<'_, AppState>, id: usize) -> Vec<ClipboardNote> {
    state.delete_note(id)
}

#[tauri::command]
pub fn clear_clipboard_notes(state: State<'_, AppState>) -> Vec<ClipboardNote> {
    state.clear_notes()
}

#[tauri::command]
pub fn hide_to_tray(window: WebviewWindow, state: State<'_, AppState>) -> Result<(), String> {
    state.set_window_visibility(false);
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn show_window(window: WebviewWindow, state: State<'_, AppState>) -> Result<(), String> {
    state.set_window_visibility(true);
    window.show().map_err(|e| e.to_string())?;
    window.unminimize().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn minimize_window(window: WebviewWindow) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_maximize(window: WebviewWindow) -> Result<(), String> {
    let is_max = window.is_maximized().map_err(|e| e.to_string())?;
    if is_max {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn exit_app(app: AppHandle) {
    app.exit(0);
}

