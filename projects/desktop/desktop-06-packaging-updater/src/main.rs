// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod crypto;
mod models;
mod updater;

use commands::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            generate_keypair,
            sign_data,
            verify_data,
            check_update_version,
            generate_manifest,
            simulate_update_stream,
            get_bundle_config,
            minimize_window,
            toggle_maximize,
            close_window,
        ])
        .run(tauri::generate_context!())
        .expect("Tauri Packaging & Updater 실행 실패");
}

#[cfg(test)]
mod tests {
    use crate::crypto::{calculate_sha256, generate_ed25519_keypair, sign_payload, verify_signature};
    use crate::updater::{compare_semver, generate_release_manifest};
    use std::cmp::Ordering;

    #[test]
    fn test_ed25519_keypair_and_sign_verify() {
        let keys = generate_ed25519_keypair();
        assert!(!keys.public_key_base64.is_empty());
        assert!(!keys.private_key_base64.is_empty());

        let payload = "Tauri 2.0 Packaging v1.0.0 Release Binary";
        let sign_res = sign_payload(&keys.private_key_base64, payload).expect("서명 생성 성공");
        assert!(sign_res.verified);
        assert_eq!(sign_res.payload_sha256.len(), 64);

        let verified = verify_signature(&keys.public_key_base64, payload, &sign_res.signature_base64)
            .expect("검증 실행 성공");
        assert!(verified, "정상 서명 검증 성공");
    }

    #[test]
    fn test_tampered_payload_fails_verification() {
        let keys = generate_ed25519_keypair();
        let payload = "Original Release Binary";
        let sign_res = sign_payload(&keys.private_key_base64, payload).unwrap();

        // 변조된 페이로드로 검증 시도
        let tampered_payload = "Hacked Malicious Binary Payload";
        let verified = verify_signature(
            &keys.public_key_base64,
            tampered_payload,
            &sign_res.signature_base64,
        )
        .unwrap();

        assert!(!verified, "변조된 페이로드는 검증 실패해야 함");
    }

    #[test]
    fn test_tampered_signature_fails() {
        let keys = generate_ed25519_keypair();
        let payload = "Safe Payload";
        let sign_res = sign_payload(&keys.private_key_base64, payload).unwrap();

        // 서명 1글자 변조
        let mut tampered_sig = sign_res.signature_base64;
        if tampered_sig.starts_with('A') {
            tampered_sig.replace_range(0..1, "B");
        } else {
            tampered_sig.replace_range(0..1, "A");
        }

        let result = verify_signature(&keys.public_key_base64, payload, &tampered_sig);
        // 디코딩 에러 또는 검증 실패(false)
        assert!(result.is_err() || result == Ok(false));
    }

    #[test]
    fn test_semver_comparison() {
        assert_eq!(
            compare_semver("1.0.0", "1.1.0").unwrap(),
            Ordering::Greater,
            "1.1.0은 1.0.0보다 상위 버전"
        );
        assert_eq!(
            compare_semver("v1.0.0", "v1.0.0").unwrap(),
            Ordering::Equal,
            "동일 버전"
        );
        assert_eq!(
            compare_semver("2.0.0", "1.9.9").unwrap(),
            Ordering::Less,
            "1.9.9는 2.0.0보다 하위 버전"
        );
        assert_eq!(
            compare_semver("1.0.0-rc.1", "1.0.0").unwrap(),
            Ordering::Greater,
            "정식 릴리즈는 rc보다 상위"
        );
    }

    #[test]
    fn test_manifest_generation() {
        let keys = generate_ed25519_keypair();
        let manifest = generate_release_manifest(
            "1.1.0",
            "보안 패치 및 신규 기능 추가",
            &keys.private_key_base64,
            "space-cap",
            "rust-sandbox",
        )
        .expect("매니페스트 생성 성공");

        assert_eq!(manifest.version, "v1.1.0");
        assert!(manifest.platforms.contains_key("windows-x86_64"));
        let win = &manifest.platforms["windows-x86_64"];
        assert!(!win.signature.is_empty());
        assert!(win.url.contains("releases/download/v1.1.0"));
    }

    #[test]
    fn test_sha256_hash() {
        let hash = calculate_sha256(b"hello world");
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_bundle_config() {
        let config = crate::commands::get_bundle_config();
        assert_eq!(config.app_version, "1.0.0");
        assert!(config.targets.contains(&"nsis".to_string()));
        assert!(config.targets.contains(&"msi".to_string()));
        assert!(config.nsis_enabled);
        assert!(config.languages.contains(&"Korean".to_string()));
    }
}

