use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TechCategory {
    RustWasm,
    JavaScript,
    Fullstack,
    Tooling,
}

impl TechCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RustWasm => "Rust Wasm",
            Self::JavaScript => "JavaScript/TS",
            Self::Fullstack => "Fullstack",
            Self::Tooling => "Tooling",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            Self::RustWasm => "badge-rust",
            Self::JavaScript => "badge-js",
            Self::Fullstack => "badge-fullstack",
            Self::Tooling => "badge-tooling",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TechItem {
    pub id: u32,
    pub name: String,
    pub category: TechCategory,
    pub description: String,
    pub stars: u32,
    pub tags: Vec<String>,
}

impl TechItem {
    pub fn new(
        id: u32,
        name: impl Into<String>,
        category: TechCategory,
        description: impl Into<String>,
        stars: u32,
        tags: Vec<&str>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            category,
            description: description.into(),
            stars,
            tags: tags.into_iter().map(String::from).collect(),
        }
    }

    pub fn sample_data() -> Vec<Self> {
        vec![
            Self::new(
                1,
                "Leptos",
                TechCategory::RustWasm,
                "세분화된 반응성(Signal) 기반의 차세대 초고속 Rust 풀스택 프레임워크",
                18400,
                vec!["Signals", "Zero VDOM", "SSR", "Hydration"],
            ),
            Self::new(
                2,
                "Yew",
                TechCategory::RustWasm,
                "React 스타일의 친숙한 VDOM 컴포넌트 모델을 제공하는 Rust Wasm 원조 프레임워크",
                29800,
                vec!["Virtual DOM", "React-like", "Components", "Hooks"],
            ),
            Self::new(
                3,
                "Dioxus",
                TechCategory::Fullstack,
                "웹, 데스크톱, 모바일, 서버를 단일 Rust 코드베이스로 구축하는 멀티플랫폼 프레임워크",
                22500,
                vec!["Cross-platform", "Desktop", "Mobile", "SSR"],
            ),
            Self::new(
                4,
                "Axum",
                TechCategory::RustWasm,
                "Tokio 팀 공식 차세대 모듈식 비동기 웹 프레임워크 (Wasm 프론트엔드의 완벽한 파트너)",
                21000,
                vec!["Tokio", "Tower", "Hyper", "Type-safe DI"],
            ),
            Self::new(
                5,
                "SolidJS",
                TechCategory::JavaScript,
                "Leptos 반응성 모델의 영감이 된 JavaScript 생태계 최고의 초경량 Signal 프레임워크",
                32400,
                vec!["Fine-grained", "Signals", "No VDOM", "Fastest JS"],
            ),
            Self::new(
                6,
                "Trunk",
                TechCategory::Tooling,
                "Rust Wasm 애플리케이션을 위한 무설정(Zero-config) WASM 웹 애플리케이션 번들러",
                5200,
                vec!["Bundler", "Hot Reload", "Proxy", "Wasm-opt"],
            ),
        ]
    }
}

// 순수 비즈니스 로직 (단위 테스트 대상)
pub fn compute_doubled(n: i32) -> i32 {
    n * 2
}

pub fn compute_squared(n: i32) -> i64 {
    (n as i64) * (n as i64)
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn filter_tech_items(
    items: &[TechItem],
    query: &str,
    category_filter: Option<TechCategory>,
) -> Vec<TechItem> {
    let lower_q = query.trim().to_lowercase();
    items
        .iter()
        .filter(|item| {
            if let Some(cat) = category_filter {
                if item.category != cat {
                    return false;
                }
            }
            if lower_q.is_empty() {
                return true;
            }
            item.name.to_lowercase().contains(&lower_q)
                || item.description.to_lowercase().contains(&lower_q)
                || item.tags.iter().any(|t| t.to_lowercase().contains(&lower_q))
        })
        .cloned()
        .collect()
}

pub fn calculate_progress(checked_count: usize, total_count: usize) -> (usize, usize, u32) {
    if total_count == 0 {
        return (0, 0, 0);
    }
    let percent = ((checked_count as f64 / total_count as f64) * 100.0).round() as u32;
    (checked_count, total_count, percent)
}

