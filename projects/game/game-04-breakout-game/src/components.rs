use bevy::prelude::*;

/// 게임 전체의 생명주기 상태 머신
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
    Victory,
}

/// 패들(플레이어) 마커 컴포넌트
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paddle;

/// 공(Ball) 마커 컴포넌트
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ball;

/// 벽돌(Brick) 컴포넌트 (획득 점수 포함)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brick {
    pub points: u32,
}

/// 메뉴 화면 전용 UI 요소 마커 (메뉴 탈출 시 자동 정리)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuElement;

/// 일시정지 오버레이 UI 요소 마커
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PausedElement;

/// 게임 오버 오버레이 UI 요소 마커
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOverElement;

/// 승리 클리어 오버레이 UI 요소 마커
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct VictoryElement;

/// 물리 이동 속도 벡터
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vec2);

/// 2D AABB 충돌체
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

    pub fn intersects(&self, pos_a: Vec2, other: &Collider2d, pos_b: Vec2) -> bool {
        let half_a = self.size * 0.5;
        let half_b = other.size * 0.5;

        (pos_a.x - pos_b.x).abs() < (half_a.x + half_b.x)
            && (pos_a.y - pos_b.y).abs() < (half_a.y + half_b.y)
    }
}

/// 이동 경계 제한 구조체
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Boundary {
    pub min_x: f32,
    pub max_x: f32,
}

impl Boundary {
    pub fn new(min_x: f32, max_x: f32) -> Self {
        Self { min_x, max_x }
    }

    pub fn clamp(&self, pos: &mut Vec3) {
        pos.x = pos.x.clamp(self.min_x, self.max_x);
    }
}

/// HUD 텍스트 마커
#[derive(Component, Debug, Clone, Copy)]
pub struct HudText;

/// 전역 게임 점수 및 목숨 리소스
#[derive(Resource, Debug, Clone)]
pub struct GameScore {
    pub score: u32,
    pub lives: u32,
    pub total_bricks: usize,
    pub remaining_bricks: usize,
}

impl Default for GameScore {
    fn default() -> Self {
        Self {
            score: 0,
            lives: 3,
            total_bricks: 0,
            remaining_bricks: 0,
        }
    }
}

/// 사전 로드된 사운드 효과음 핸들 리소스
#[derive(Resource, Debug, Clone)]
pub struct SoundEffects {
    pub bounce: Handle<AudioSource>,
    pub brick_break: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
}

