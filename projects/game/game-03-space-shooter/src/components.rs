use bevy::prelude::*;

/// 플레이어 우주선 마커 컴포넌트
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

/// 플레이어가 발사한 레이저 총알 마커 컴포넌트
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaserBullet;

/// 외계 적기 마커 컴포넌트
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Enemy;

/// 우주 배경의 별 마커 컴포넌트
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Star;

/// 이동 속도 (픽셀/초)
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Speed(pub f32);

/// 2D 방향 속도 벡터 (방향 * 속도)
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vec2);

/// 2D 축 정렬 충돌체 (AABB: Axis-Aligned Bounding Box)
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Collider2d {
    pub size: Vec2,
}

impl Collider2d {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
        }
    }

    /// 두 중심 좌표와 바운딩 박스를 이용한 AABB 교차 판정
    pub fn intersects(&self, pos_a: Vec2, other: &Collider2d, pos_b: Vec2) -> bool {
        let half_a = self.size * 0.5;
        let half_b = other.size * 0.5;

        (pos_a.x - pos_b.x).abs() < (half_a.x + half_b.x)
            && (pos_a.y - pos_b.y).abs() < (half_a.y + half_b.y)
    }
}

/// 무기 발사 쿨다운 타이머
#[derive(Component, Debug, Clone)]
pub struct WeaponCooldown(pub Timer);

/// 화면 이동 경계
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

/// HUD 텍스트 마커 컴포넌트
#[derive(Component, Debug, Clone, Copy)]
pub struct HudText;

/// 전역 게임 상태 리소스
#[derive(Resource, Debug, Clone)]
pub struct GameState {
    pub score: u32,
    pub enemies_killed: u32,
    pub shields: i32,
    pub is_game_over: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            enemies_killed: 0,
            shields: 3,
            is_game_over: false,
        }
    }
}

/// 주기적 적기 스폰 타이머 리소스
#[derive(Resource, Debug, Clone)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.75, TimerMode::Repeating))
    }
}

