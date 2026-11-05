//! 자동 업데이트(Auto-Updater) 로직 및 latest.json 매니페스트 관리
use crate::crypto::sign_payload;
use crate::models::{ReleaseManifest, ReleasePlatform};
use chrono::Utc;
use semver::Version;
use std::cmp::Ordering;
use std::collections::HashMap;

/// 유의적 버전(SemVer) 비교: target_ver가 current_ver보다 높은지 판정
pub fn compare_semver(current: &str, target: &str) -> Result<Ordering, String> {
    let clean_current = current.trim().trim_start_matches('v');
    let clean_target = target.trim().trim_start_matches('v');

    let cur = Version::parse(clean_current)
        .map_err(|e| format!("현재 버전 파싱 오류 ('{}'): {}", current, e))?;
    let tar = Version::parse(clean_target)
        .map_err(|e| format!("대상 버전 파싱 오류 ('{}'): {}", target, e))?;

    Ok(tar.cmp(&cur))
}

/// Tauri 2.0 공식 latest.json 릴리즈 매니페스트 생성
pub fn generate_release_manifest(
    version: &str,
    notes: &str,
    private_key_base64: &str,
    repo_owner: &str,
    repo_name: &str,
) -> Result<ReleaseManifest, String> {
    let clean_ver = version.trim().trim_start_matches('v');
    let _ = Version::parse(clean_ver).map_err(|e| format!("잘못된 버전 포맷: {}", e))?;

    // Windows x86_64 NSIS 인스톨러 모의 바이너리 서명 생성
    let win_filename = format!("{}_{}_x64-setup.nsis.zip", repo_name, clean_ver);
    let win_url = format!(
        "https://github.com/{}/{}/releases/download/v{}/{}",
        repo_owner, repo_name, clean_ver, win_filename
    );

    // 모의 바이너리 페이로드에 대한 전자 서명 생성
    let mock_binary_payload = format!("{}:{}", win_filename, clean_ver);
    let sign_res = sign_payload(private_key_base64, &mock_binary_payload)?;

    let mut platforms = HashMap::new();
    platforms.insert(
        "windows-x86_64".to_string(),
        ReleasePlatform {
            signature: sign_res.signature_base64,
            url: win_url,
        },
    );

    Ok(ReleaseManifest {
        version: format!("v{}", clean_ver),
        notes: notes.to_string(),
        pub_date: Utc::now().to_rfc3339(),
        platforms,
    })
}

