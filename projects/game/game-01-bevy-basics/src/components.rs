use bevy::prelude::*;

/// 플레이어 개체(Entity)를 식별하기 위한 마커 컴포넌트
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

/// 플레이어의 이동 속도 (초당 픽셀 이동량)
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Speed(pub f32);

/// 2D 윈도우 화면 경계 제한 컴포넌트
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Boundary {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl Boundary {
    pub fn new(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self {
        Self { min_x, max_x, min_y, max_y }
    }

    /// 좌표가 화면 밖으로 벗어나지 않도록 클램핑
    pub fn clamp(&self, position: &mut Vec3) {
        position.x = position.x.clamp(self.min_x, self.max_x);
        position.y = position.y.clamp(self.min_y, self.max_y);
    }
}

/// HUD 정보 텍스트 컴포넌트 마커
#[derive(Component, Debug, Clone, Copy)]
pub struct HudInfoText;

/// 전역 게임 통계 리소스 (Resource)
#[derive(Resource, Debug, Clone, Default)]
pub struct GameStats {
    pub total_distance_moved: f32,
    pub frame_count: u64,
}

