// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod state;

use std::str::FromStr;
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tauri_plugin_notification::NotificationExt;

use commands::*;
use state::AppState;

fn main() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .manage(app_state)
        // 1. Single Instance 플러그인: 중복 실행 감지 시 기존 창 포커스
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }

            let state = app.state::<AppState>();
            state.set_window_visibility(true);
            state.record_single_instance_event(args.clone());

            let _ = app.emit("single-instance-event", &args);
            let _ = app
                .notification()
                .builder()
                .title("⚠️ 중복 실행 방지 (Single Instance)")
                .body("앱이 이미 백그라운드에 실행 중입니다. 기존 창을 맨 앞으로 활성화했습니다.")
                .show();
        }))
        // 2. OS 네이티브 토스트 알림 플러그인
        .plugin(tauri_plugin_notification::init())
        // 3. 시스템 전역 단축키 플러그인
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        let state = app.state::<AppState>();
                        let count = state.increment_shortcut();

                        if let Some(window) = app.get_webview_window("main") {
                            let is_visible = window.is_visible().unwrap_or(false);
                            if is_visible {
                                let _ = window.hide();
                                state.set_window_visibility(false);
                            } else {
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                                state.set_window_visibility(true);
                            }
                        }

                        let _ = app.emit("shortcut-triggered", count);
                    }
                })
                .build(),
        )
        // 4. 앱 초기화 및 시스템 트레이 / 백그라운드 워커 등록
        .setup(|app| {
            // 전역 단축키 등록: Ctrl+Shift+U
            let shortcut = Shortcut::from_str("Ctrl+Shift+U")
                .map_err(|e| format!("단축키 파싱 실패: {:?}", e))?;
            let _ = app.global_shortcut().register(shortcut);

            // 시스템 트레이 컨텍스트 메뉴 구성
            let toggle_i = MenuItem::with_id(app, "toggle", "📌 창 보이기 / 숨기기", true, None::<&str>)?;
            let notify_i = MenuItem::with_id(app, "notify", "🔔 즉시 알림 발송", true, None::<&str>)?;
            let timer_i = MenuItem::with_id(app, "timer_10s", "⏱️ 10초 퀵 타이머 시작", true, None::<&str>)?;
            let sep_i = PredefinedMenuItem::separator(app)?;
            let quit_i = MenuItem::with_id(app, "quit", "❌ 완전히 종료", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&toggle_i, &notify_i, &timer_i, &sep_i, &quit_i])?;

            // 트레이 아이콘 인스턴스 빌드
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("Tauri 2.0 Resident Utility")
                .on_menu_event(|app, event| {
                    let state = app.state::<AppState>();
                    match event.id.as_ref() {
                        "toggle" => {
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                    state.set_window_visibility(false);
                                } else {
                                    let _ = window.show();
                                    let _ = window.unminimize();
                                    let _ = window.set_focus();
                                    state.set_window_visibility(true);
                                }
                            }
                        }
                        "notify" => {
                            let count = state.increment_notification();
                            let _ = app
                                .notification()
                                .builder()
                                .title("🔔 트레이 컨텍스트 알림")
                                .body(format!("작업표시줄 트레이 메뉴에서 발송되었습니다 (누적: {}회)", count))
                                .show();
                        }
                        "timer_10s" => {
                            let status = state.start_timer(10, "트레이 10초 퀵 타이머".to_string());
                            let _ = app.emit("timer-updated", &status);
                            let _ = app
                                .notification()
                                .builder()
                                .title("⏱️ 타이머 시작됨")
                                .body("10초 백그라운드 퀵 타이머가 시작되었습니다.")
                                .show();
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    // 트레이 아이콘 좌클릭 시 토글
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        let state = app.state::<AppState>();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                                state.set_window_visibility(false);
                            } else {
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                                state.set_window_visibility(true);
                            }
                        }
                    }
                })
                .build(app)?;

            // 백그라운드 상주 틱 워커 (타이머 카운트다운 및 상태 브로드캐스트)
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                loop {
                    interval.tick().await;

                    if app_handle.webview_windows().is_empty() {
                        break;
                    }

                    let state = app_handle.state::<AppState>();
                    if let Some(label) = state.tick_timer() {
                        // 타이머 완료 시 네이티브 토스트 알림 발송 및 웹뷰 이벤트 발행
                        let _ = app_handle
                            .notification()
                            .builder()
                            .title("⏱️ 백그라운드 타이머 완료!")
                            .body(format!("요청하신 '{}' 타이머가 완료되었습니다.", label))
                            .show();
                        let _ = app_handle.emit("timer-finished", &label);
                    }

                    // 1초마다 현재 상주 앱 상태 브로드캐스트
                    let summary = state.get_summary();
                    let _ = app_handle.emit("status-tick", &summary);
                }
            });

            Ok(())
        })
        // 5. 창 닫기(X) 클릭 시 종료 방지 및 트레이 최소화
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();

                let app = window.app_handle();
                let state = app.state::<AppState>();
                state.set_window_visibility(false);

                let _ = app
                    .notification()
                    .builder()
                    .title("📌 시스템 트레이로 최소화됨")
                    .body("앱이 종료되지 않고 백그라운드 트레이에 계속 상주합니다. [Ctrl+Shift+U]로 복원할 수 있습니다.")
                    .show();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_app_status,
            send_toast_notification,
            start_timer,
            stop_timer,
            add_clipboard_note,
            get_clipboard_notes,
            delete_clipboard_note,
            clear_clipboard_notes,
            hide_to_tray,
            show_window,
            minimize_window,
            toggle_maximize,
            exit_app,
        ])
        .run(tauri::generate_context!())
        .expect("Tauri Tray Tool 실행 실패");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;

    #[test]
    fn test_initial_app_state() {
        let state = AppState::new();
        let summary = state.get_summary();

        assert_eq!(summary.notification_count, 0);
        assert_eq!(summary.shortcut_pressed_count, 0);
        assert_eq!(summary.shortcut_binding, "Ctrl+Shift+U");
        assert!(summary.is_window_visible);
        assert_eq!(summary.notes.len(), 2);
        assert!(!summary.timer.is_running);
    }

    #[test]
    fn test_notification_and_shortcut_counters() {
        let state = AppState::new();
        assert_eq!(state.increment_notification(), 1);
        assert_eq!(state.increment_notification(), 2);

        assert_eq!(state.increment_shortcut(), 1);
        assert_eq!(state.increment_shortcut(), 2);
        assert_eq!(state.increment_shortcut(), 3);

        let summary = state.get_summary();
        assert_eq!(summary.notification_count, 2);
        assert_eq!(summary.shortcut_pressed_count, 3);
    }

    #[test]
    fn test_timer_lifecycle_and_tick() {
        let state = AppState::new();
        let status = state.start_timer(2, "테스트 타이머".to_string());
        assert!(status.is_running);
        assert_eq!(status.remaining_secs, 2);

        // 첫 번째 틱 (2초 -> 1초)
        let finish1 = state.tick_timer();
        assert_eq!(finish1, None);

        // 두 번째 틱 (1초 -> 0초, 완료)
        let finish2 = state.tick_timer();
        assert_eq!(finish2, Some("테스트 타이머".to_string()));

        // 완료 후 타이머는 정지 상태여야 함
        let summary = state.get_summary();
        assert!(!summary.timer.is_running);
        assert_eq!(summary.timer.remaining_secs, 0);
    }

    #[test]
    fn test_timer_manual_stop() {
        let state = AppState::new();
        state.start_timer(100, "작업 타이머".to_string());
        let stopped = state.stop_timer();

        assert!(!stopped.is_running);
        assert_eq!(stopped.remaining_secs, 0);
        assert_eq!(stopped.label, "중지됨");
    }

    #[test]
    fn test_clipboard_notes_crud() {
        let state = AppState::new();
        let initial_len = state.get_notes().len();

        let notes_after_add = state.add_note("새로운 퀵 클립보드 메모".to_string());
        assert_eq!(notes_after_add.len(), initial_len + 1);
        assert_eq!(notes_after_add[0].text, "새로운 퀵 클립보드 메모");

        let added_id = notes_after_add[0].id;
        let notes_after_delete = state.delete_note(added_id);
        assert_eq!(notes_after_delete.len(), initial_len);

        let cleared = state.clear_notes();
        assert_eq!(cleared.len(), 0);
    }

    #[test]
    fn test_single_instance_event_recording() {
        let state = AppState::new();
        state.record_single_instance_event(vec!["--open-url".to_string(), "myapp://item/1".to_string()]);

        let summary = state.get_summary();
        assert_eq!(summary.single_instance_events.len(), 1);
        assert!(summary.single_instance_events[0].contains("--open-url"));
    }

    #[test]
    fn test_shortcut_parsing() {
        let shortcut = Shortcut::from_str("Ctrl+Shift+U");
        assert!(shortcut.is_ok(), "Ctrl+Shift+U 파싱 성공");

        let shortcut_alt = Shortcut::from_str("Alt+Shift+X");
        assert!(shortcut_alt.is_ok(), "Alt+Shift+X 파싱 성공");
    }
}

