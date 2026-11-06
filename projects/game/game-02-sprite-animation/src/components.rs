use bevy::prelude::*;

/// 플레이어 개체 식별 마커 컴포넌트
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

/// 플레이어 이동 속도 (초당 픽셀)
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Speed(pub f32);

/// 캐릭터 애니메이션 상태 (대기 vs 달리기)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Run,
}

/// 아틀라스 내 프레임 재생 범위
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    pub const IDLE: AnimationIndices = AnimationIndices { first: 0, last: 3 };
    pub const RUN: AnimationIndices = AnimationIndices { first: 4, last: 7 };
}

/// 프레임 틱 전환 타이머
#[derive(Component, Deref, DerefMut, Debug, Clone)]
pub struct AnimationTimer(pub Timer);

/// 화면 밖 이탈 방지 경계
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

    pub fn clamp(&self, pos: &mut Vec3) {
        pos.x = pos.x.clamp(self.min_x, self.max_x);
        pos.y = pos.y.clamp(self.min_y, self.max_y);
    }
}

/// HUD 정보 텍스트 컴포넌트 마커
#[derive(Component, Debug, Clone, Copy)]
pub struct HudInfoText;

/// 전역 게임 통계 리소스
#[derive(Resource, Debug, Clone, Default)]
pub struct GameStats {
    pub frame_count: u64,
    pub total_distance: f32,
}

