use bevy::prelude::*;

/// 플레이어 에너지 캐논 마커
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerCannon;

/// 캐논 발사체 마커
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CannonBall;

/// 타겟 더미 (체력 포함)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetDummy {
    pub health: i32,
    pub max_health: i32,
}

/// 단일 파티클 컴포넌트
#[derive(Component, Debug, Clone)]
pub struct Particle {
    pub lifetime: Timer,
    pub drag: f32,
    pub base_color: Color,
    pub initial_size: f32,
}

/// 2D 속도 벡터
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

/// 화면 경계 제한
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

/// 발사 쿨다운 컴포넌트
#[derive(Component, Debug, Clone)]
pub struct CannonCooldown(pub Timer);

/// HUD 텍스트 마커
#[derive(Component, Debug, Clone, Copy)]
pub struct HudText;

/// 카메라 쉐이크(트라우마 모델) 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct CameraShake {
    pub trauma: f32, // 0.0 ~ 1.0
}

impl CameraShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }
}

/// 히트스톱(순간적 시간 동결) 매니저 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct HitstopManager {
    pub timer: Option<Timer>,
}

/// 디버그 기즈모 토글 설정 리소스
#[derive(Resource, Debug, Clone)]
pub struct DebugGizmoSettings {
    pub enabled: bool,
}

impl Default for DebugGizmoSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// 통계 집계 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct JuiceStats {
    pub targets_destroyed: u32,
    pub particles_spawned: u64,
    pub total_shots: u32,
}

/// 사운드 에셋 리소스
#[derive(Resource, Debug, Clone)]
pub struct SoundAssets {
    pub laser: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,
}

