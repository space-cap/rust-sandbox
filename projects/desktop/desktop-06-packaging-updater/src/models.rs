//! 데이터 모델: 배포 패키징, Ed25519 암호화 서명, 자동 업데이트 매니페스트
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ed25519 비대칭 암호화 키 쌍
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyPairResult {
    pub public_key_base64: String,
    pub private_key_base64: String,
}

/// 서명 생성 결과
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignResult {
    pub payload_sha256: String,
    pub signature_base64: String,
    pub verified: bool,
}

/// 플랫폼별 릴리즈 아티팩트 정보 (Tauri latest.json 규격)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleasePlatform {
    pub signature: String,
    pub url: String,
}

/// Tauri 2.0 공식 latest.json 릴리즈 매니페스트
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub version: String,
    pub notes: String,
    pub pub_date: String,
    pub platforms: HashMap<String, ReleasePlatform>,
}

/// 자동 업데이트 단계별 시뮬레이션 상태
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateSimulationStep {
    pub step: usize,
    pub status: String,
    pub progress_percent: u32,
    pub message: String,
}

/// 번들 패키징 설정 분석 정보
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleConfigInfo {
    pub app_name: String,
    pub app_version: String,
    pub app_identifier: String,
    pub targets: Vec<String>,
    pub nsis_enabled: bool,
    pub msi_enabled: bool,
    pub languages: Vec<String>,
    pub icon_path: String,
}

