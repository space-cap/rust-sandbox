use serde::{Deserialize, Serialize};

/// 운영체제 및 하드웨어 정보 전송 객체 (DTO)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemInfoDto {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub host_name: String,
    pub cpu_arch: String,
    pub cpu_count: usize,
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
    pub uptime_secs: u64,
}

/// 고속 암호화 해시 계산 결과 객체
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HashResponse {
    pub algorithm: String,
    pub input_length: usize,
    pub hex_digest: String,
    pub elapsed_micros: u128,
}

/// 백엔드에서 1초마다 프론트엔드로 브로드캐스팅하는 시스템 실시간 펄스 이벤트 데이터
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricPulse {
    pub timestamp: String,
    pub cpu_usage: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub active_items_count: usize,
    pub tick_count: u64,
}

