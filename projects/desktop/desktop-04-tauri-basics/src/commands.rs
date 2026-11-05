use sha2::{Digest, Sha256};
use std::time::Instant;
use sysinfo::System;
use tauri::Manager;

use crate::models::{HashResponse, SystemInfoDto};
use crate::state::AppState;

/// 1. 가장 기본적인 인사말 RPC 명령
#[tauri::command]
pub fn greet(name: &str) -> String {
    let target = if name.trim().is_empty() {
        "방문자"
    } else {
        name.trim()
    };
    format!("안녕하세요, {target}님! Rust Tauri 2.0 백엔드 IPC 코어에서 전송된 실시간 응답입니다.")
}

/// 2. 운영체제 및 하드웨어 정보 조회 명령 (sysinfo 크레이트 활용)
#[tauri::command]
pub fn get_system_info() -> SystemInfoDto {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "Windows (알 수 없음)".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "알 수 없음".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "알 수 없음".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "localhost".to_string());
    let cpu_arch = System::cpu_arch().unwrap_or_else(|| "x86_64".to_string());
    let cpu_count = sys.cpus().len();

    let total_memory_mb = sys.total_memory() / 1024 / 1024;
    let used_memory_mb = sys.used_memory() / 1024 / 1024;
    let uptime_secs = System::uptime();

    SystemInfoDto {
        os_name,
        os_version,
        kernel_version,
        host_name,
        cpu_arch,
        cpu_count,
        total_memory_mb,
        used_memory_mb,
        uptime_secs,
    }
}

/// 3. SHA-256 고속 암호화 해시 연산 명령
#[tauri::command]
pub fn calculate_hash(content: String) -> HashResponse {
    let start = Instant::now();
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    let hex_digest = format!("{:x}", result);
    let elapsed_micros = start.elapsed().as_micros();

    HashResponse {
        algorithm: "SHA-256".to_string(),
        input_length: content.len(),
        hex_digest,
        elapsed_micros,
    }
}

/// 4. 인메모리 상태 항목 추가
#[tauri::command]
pub fn add_item(item: String, state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    state.add_item(item)
}

/// 5. 인메모리 상태 항목 목록 조회
#[tauri::command]
pub fn get_items(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    state.get_items()
}

/// 6. 인메모리 상태 특정 항목 삭제
#[tauri::command]
pub fn delete_item(index: usize, state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    state.delete_item(index)
}

/// 7. 인메모리 상태 전체 항목 비우기
#[tauri::command]
pub fn clear_items(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    state.clear_items()
}

/// 8. 무프레임 창 제어: 최소화
#[tauri::command]
pub fn minimize_window(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

/// 9. 무프레임 창 제어: 최대화 / 복원 토글
#[tauri::command]
pub fn toggle_maximize_window(window: tauri::Window) -> Result<bool, String> {
    let is_max = window.is_maximized().map_err(|e| e.to_string())?;
    if is_max {
        window.unmaximize().map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        window.maximize().map_err(|e| e.to_string())?;
        Ok(true)
    }
}

/// 10. 무프레임 창 제어: 창 닫기 및 프로세스 정상 종료
#[tauri::command]
pub fn close_window(window: tauri::Window) -> Result<(), String> {
    window.app_handle().exit(0);
    Ok(())
}
