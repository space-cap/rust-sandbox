//! Tauri IPC Commands: 암호화 서명, 자동 업데이트 시뮬레이션, 번들 환경 분석
use crate::crypto::{generate_ed25519_keypair, sign_payload, verify_signature};
use crate::models::{BundleConfigInfo, KeyPairResult, ReleaseManifest, SignResult, UpdateSimulationStep};
use crate::updater::{compare_semver, generate_release_manifest};
use std::cmp::Ordering;
use tauri::{AppHandle, Emitter, WebviewWindow};

#[tauri::command]
pub fn generate_keypair() -> KeyPairResult {
    generate_ed25519_keypair()
}

#[tauri::command]
pub fn sign_data(private_key_base64: String, payload: String) -> Result<SignResult, String> {
    sign_payload(&private_key_base64, &payload)
}

#[tauri::command]
pub fn verify_data(
    public_key_base64: String,
    payload: String,
    signature_base64: String,
) -> Result<bool, String> {
    verify_signature(&public_key_base64, &payload, &signature_base64)
}

#[tauri::command]
pub fn check_update_version(current_ver: String, remote_ver: String) -> Result<bool, String> {
    let ord = compare_semver(&current_ver, &remote_ver)?;
    Ok(ord == Ordering::Greater) // remote_ver > current_ver
}

#[tauri::command]
pub fn generate_manifest(
    version: String,
    notes: String,
    private_key: String,
) -> Result<ReleaseManifest, String> {
    generate_release_manifest(
        &version,
        &notes,
        &private_key,
        "space-cap",
        "desktop-06-packaging-updater",
    )
}

#[tauri::command]
pub async fn simulate_update_stream(app: AppHandle, target_ver: String) -> Result<(), String> {
    let steps = vec![
        UpdateSimulationStep {
            step: 1,
            status: "CHECKING".to_string(),
            progress_percent: 15,
            message: format!("원격 릴리즈 서버에서 latest.json 매니페스트 수신 및 SemVer({}) 비교", target_ver),
        },
        UpdateSimulationStep {
            step: 2,
            status: "VERIFYING_KEY".to_string(),
            progress_percent: 35,
            message: "Ed25519 비대칭 공개키를 사용하여 릴리즈 패키지 전자 서명 무결성 검증 통과".to_string(),
        },
        UpdateSimulationStep {
            step: 3,
            status: "DOWNLOADING".to_string(),
            progress_percent: 70,
            message: format!("백그라운드 청크 스트리밍 다운로드 진행 중 (임시 디렉토리 .tar.gz / .zip)"),
        },
        UpdateSimulationStep {
            step: 4,
            status: "EXTRACTING".to_string(),
            progress_percent: 90,
            message: "SHA-256 체크섬 교차 검증 및 원자적(Atomic) 바이너리 교체 준비 완료".to_string(),
        },
        UpdateSimulationStep {
            step: 5,
            status: "READY".to_string(),
            progress_percent: 100,
            message: format!("새 버전({}) 패치가 완료되었습니다. 재시작 시 새 버전이 적용됩니다!", target_ver),
        },
    ];

    for s in steps {
        tokio::time::sleep(std::time::Duration::from_millis(600)).await;
        let _ = app.emit("update-step-progress", &s);
    }

    Ok(())
}

#[tauri::command]
pub fn get_bundle_config() -> BundleConfigInfo {
    BundleConfigInfo {
        app_name: "desktop-06-packaging-updater".to_string(),
        app_version: "1.0.0".to_string(),
        app_identifier: "com.rustsandbox.desktop06".to_string(),
        targets: vec!["nsis".to_string(), "msi".to_string()],
        nsis_enabled: true,
        msi_enabled: true,
        languages: vec!["Korean".to_string(), "English".to_string()],
        icon_path: "icons/icon.ico".to_string(),
    }
}

// 창 조작 커맨드
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
pub fn close_window(window: WebviewWindow) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

