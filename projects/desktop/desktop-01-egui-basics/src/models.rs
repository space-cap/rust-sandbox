use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Developer,
    Designer,
    Manager,
    DevOps,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Developer => "💻 개발자 (Developer)",
            Self::Designer => "🎨 디자이너 (Designer)",
            Self::Manager => "📋 프로덕트 매니저 (PM)",
            Self::DevOps => "🚀 데브옵스 (DevOps)",
        }
    }

    pub fn all() -> [Self; 4] {
        [Self::Developer, Self::Designer, Self::Manager, Self::DevOps]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppTheme {
    Dark,
    Light,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub name: String,
    pub role: UserRole,
    pub experience_years: u32,
    pub is_subscribed: bool,
    pub volume: f32,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            name: "페리스 (Ferris)".to_string(),
            role: UserRole::Developer,
            experience_years: 3,
            is_subscribed: true,
            volume: 75.0,
        }
    }
}

// 순수 비즈니스 로직 (단위 테스트 대상)
pub fn format_greeting(name: &str, role: &UserRole) -> String {
    let clean_name = name.trim();
    if clean_name.is_empty() {
        "익명의 러스트 탐험가님, 환영합니다!".to_string()
    } else {
        format!("{clean_name}님 ({}), 환영합니다! 🦀", role.as_str())
    }
}

pub fn calculate_experience_level(years: u32) -> &'static str {
    match years {
        0 => "새싹 주니어 (Rookie) 🌱",
        1..=3 => "주니어 엔지니어 (Junior) 🚀",
        4..=7 => "미드레벨 엔지니어 (Mid-level) ⚡",
        8..=14 => "시니어 아키텍트 (Senior) 🏛️",
        _ => "마스터 장인 (Master / Fellow) 👑",
    }
}

pub fn validate_username(name: &str) -> Result<(), &'static str> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("이름을 입력해 주세요.");
    }
    if trimmed.len() > 30 {
        return Err("이름은 30자 이내여야 합니다.");
    }
    Ok(())
}

