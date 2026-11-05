// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use tauri::{Emitter, Manager};

use desktop_04_tauri_basics::commands::*;
use desktop_04_tauri_basics::models::MetricPulse;
use desktop_04_tauri_basics::state::AppState;

fn main() {
    println!("🚀 [desktop-04-tauri-basics] Tauri 2.0 하이브리드 데스크톱 앱을 실행합니다...");

    tauri::Builder::default()
        .manage(AppState::new())
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                window.app_handle().exit(0);
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_system_info,
            calculate_hash,
            add_item,
            get_items,
            delete_item,
            clear_items,
            minimize_window,
            toggle_maximize_window,
            close_window,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // 백그라운드 실시간 시스템 메트릭 이벤트 스트리머 (1초 주기)
            tauri::async_runtime::spawn(async move {
                let mut sys = sysinfo::System::new_all();
                let mut tick = 0u64;

                loop {
                    tokio::time::sleep(Duration::from_millis(1000)).await;

                    // 창이 모두 닫혔으면 백그라운드 스트림 안전 종료
                    if app_handle.webview_windows().is_empty() {
                        break;
                    }

                    sys.refresh_cpu_usage();
                    sys.refresh_memory();
                    tick += 1;

                    let cpu_usage = sys.global_cpu_info().cpu_usage();
                    let memory_used_mb = sys.used_memory() / 1024 / 1024;
                    let memory_total_mb = sys.total_memory() / 1024 / 1024;
                    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();

                    let items_count = if let Some(state) = app_handle.try_state::<AppState>() {
                        state.get_items().map(|v| v.len()).unwrap_or(0)
                    } else {
                        0
                    };

                    let pulse = MetricPulse {
                        timestamp,
                        cpu_usage,
                        memory_used_mb,
                        memory_total_mb,
                        active_items_count: items_count,
                        tick_count: tick,
                    };

                    let _ = app_handle.emit("sys-pulse", pulse);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri 앱 실행 실패");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_initial_items() {
        let state = AppState::new();
        let items = state.get_items().expect("초기 항목 조회 실패");
        assert_eq!(items.len(), 3);
        assert!(items[0].contains("Tao"));
    }

    #[test]
    fn test_state_add_and_get() {
        let state = AppState::new();

        // 1. 정상 항목 추가
        let updated = state.add_item("새로운 테스트 항목".to_string()).expect("항목 추가 실패");
        assert_eq!(updated.len(), 4);
        assert_eq!(updated.last().unwrap(), "새로운 테스트 항목");

        // 2. 공백 문자열 추가 시 에러 반환 검증
        let err_result = state.add_item("   ".to_string());
        assert!(err_result.is_err());
    }

    #[test]
    fn test_state_delete_and_clear() {
        let state = AppState::new();

        // 1. 유효 범위 내 항목 삭제
        let after_delete = state.delete_item(0).expect("0번 인덱스 삭제 실패");
        assert_eq!(after_delete.len(), 2);

        // 2. 범위 초과 인덱스 삭제 시 에러 검증
        let out_of_bounds = state.delete_item(999);
        assert!(out_of_bounds.is_err());

        // 3. 전체 초기화
        let cleared = state.clear_items().expect("전체 초기화 실패");
        assert_eq!(cleared.len(), 0);
    }

    #[test]
    fn test_greet_command() {
        // 1. 이름이 주어진 경우
        let response = greet("Rustacean");
        assert!(response.contains("Rustacean"));
        assert!(response.contains("Tauri 2.0"));

        // 2. 빈 문자열인 경우 기본값 "방문자" 처리
        let default_response = greet("   ");
        assert!(default_response.contains("방문자"));
    }

    #[test]
    fn test_calculate_hash() {
        // SHA-256 빈 문자열의 표준 해시값
        let empty_hash = calculate_hash("".to_string());
        assert_eq!(empty_hash.algorithm, "SHA-256");
        assert_eq!(empty_hash.input_length, 0);
        assert_eq!(
            empty_hash.hex_digest,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );

        // 임의의 문자열 해시 검증
        let hello_hash = calculate_hash("Tauri 2.0 Rocks".to_string());
        assert_eq!(hello_hash.hex_digest.len(), 64);
    }

    #[test]
    fn test_system_info_fields() {
        let info = get_system_info();
        assert!(!info.os_name.is_empty());
        assert!(info.cpu_count > 0);
        assert!(info.total_memory_mb > 0);
    }

    #[test]
    fn test_tick_counter() {
        let state = AppState::new();
        assert_eq!(state.increment_tick(), 1);
        assert_eq!(state.increment_tick(), 2);
        assert_eq!(state.increment_tick(), 3);
    }
}
