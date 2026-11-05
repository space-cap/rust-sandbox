use std::sync::Mutex;

/// Tauri 백엔드 전역 인메모리 관리 상태
#[derive(Debug, Default)]
pub struct AppState {
    pub items: Mutex<Vec<String>>,
    pub tick_counter: Mutex<u64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(vec![
                "Tauri 2.0 Tao + Wry 아키텍처 이해하기".to_string(),
                "#[tauri::command] 타입 안전 RPC 통신 검증".to_string(),
                "Capabilities 기반 보안 샌드박스 정책 학습".to_string(),
            ]),
            tick_counter: Mutex::new(0),
        }
    }

    /// 새 항목 추가 (빈 문자열 검증 포함)
    pub fn add_item(&self, item: String) -> Result<Vec<String>, String> {
        let trimmed = item.trim();
        if trimmed.is_empty() {
            return Err("추가할 항목의 내용이 비어 있습니다.".to_string());
        }

        let mut lock = self.items.lock().map_err(|e| e.to_string())?;
        lock.push(trimmed.to_string());
        Ok(lock.clone())
    }

    /// 전체 항목 목록 조회
    pub fn get_items(&self) -> Result<Vec<String>, String> {
        let lock = self.items.lock().map_err(|e| e.to_string())?;
        Ok(lock.clone())
    }

    /// 특정 인덱스의 항목 삭제
    pub fn delete_item(&self, index: usize) -> Result<Vec<String>, String> {
        let mut lock = self.items.lock().map_err(|e| e.to_string())?;
        if index >= lock.len() {
            return Err(format!("삭제할 인덱스({index})가 유효 범위를 벗어났습니다."));
        }
        lock.remove(index);
        Ok(lock.clone())
    }

    /// 전체 항목 초기화
    pub fn clear_items(&self) -> Result<Vec<String>, String> {
        let mut lock = self.items.lock().map_err(|e| e.to_string())?;
        lock.clear();
        Ok(lock.clone())
    }

    /// 틱 카운터 증가 및 현재 값 반환
    pub fn increment_tick(&self) -> u64 {
        if let Ok(mut lock) = self.tick_counter.lock() {
            *lock += 1;
            *lock
        } else {
            0
        }
    }
}

