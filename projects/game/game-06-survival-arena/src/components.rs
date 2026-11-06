use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// 게임의 4대 라이프사이클 상태 머신
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    Playing,
    LevelUp,
    GameOver,
}

/// 플레이어 마커
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

/// 플레이어 종합 능력치 컴포넌트
#[derive(Component, Debug, Clone)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub level: u32,
    pub exp: f32,
    pub next_level_exp: f32,
    pub magnet_radius: f32,
    pub orbit_count: u32,
    pub orbit_speed: f32,
    pub orbit_damage: f32,
    pub arrow_cooldown: Timer,
    pub arrow_damage: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            speed: 240.0,
            level: 1,
            exp: 0.0,
            next_level_exp: 15.0,
            magnet_radius: 120.0,
            orbit_count: 1,
            orbit_speed: 2.5,
            orbit_damage: 18.0,
            arrow_cooldown: Timer::from_seconds(0.85, TimerMode::Repeating),
            arrow_damage: 35.0,
        }
    }
}

/// 적 몬스터 컴포넌트
#[derive(Component, Debug, Clone)]
pub struct Enemy {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub damage: f32,
    pub exp_value: f32,
}

/// 플레이어 주위를 회전하는 마법 칼날 (오비탈 블레이드)
#[derive(Component, Debug, Clone)]
pub struct OrbitBlade {
    pub index: u32,
    pub radius: f32,
    pub angle: f32,
    pub angular_speed: f32,
    pub damage: f32,
}

/// 최단 거리 적 자동 유도 화살
#[derive(Component, Debug, Clone)]
pub struct HomingArrow {
    pub target_pos: Vec2,
    pub speed: f32,
    pub damage: f32,
    pub lifetime: Timer,
}

/// 경험치 보석 (자석 인력 반응)
#[derive(Component, Debug, Clone)]
pub struct ExpGem {
    pub value: f32,
    pub is_attracted: bool,
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

/// 파티클 이펙트 컴포넌트
#[derive(Component, Debug, Clone)]
pub struct Particle {
    pub lifetime: Timer,
    pub drag: f32,
    pub base_color: Color,
}

/// 페리스 스프라이트 애니메이션 타이머
#[derive(Component, Debug, Clone)]
pub struct AnimationTimer(pub Timer);

/// 페리스 스프라이트 애니메이션 인덱스
#[derive(Component, Debug, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

/// 플레이어 방향 (flip_x 연동)
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PlayerFacing {
    pub flip_x: bool,
}

/// HUD 마커
#[derive(Component, Debug, Clone, Copy)]
pub struct HudHealthText;

#[derive(Component, Debug, Clone, Copy)]
pub struct HudExpText;

#[derive(Component, Debug, Clone, Copy)]
pub struct HudTimeKillsText;

/// UI 정리용 마커
#[derive(Component, Debug, Clone, Copy)]
pub struct MenuUi;

#[derive(Component, Debug, Clone, Copy)]
pub struct LevelUpUi;

#[derive(Component, Debug, Clone, Copy)]
pub struct GameOverUi;

/// 레벨업 스킬 선택 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeType {
    ExtraBlade,
    ArrowCooldown,
    HealAndMaxHp,
    MagnetRadius,
    MoveSpeed,
}

impl UpgradeType {
    pub fn title(&self) -> &'static str {
        match self {
            UpgradeType::ExtraBlade => "마법 칼날 +1개 (공전 회전 속도 +20%)",
            UpgradeType::ArrowCooldown => "유도 번개 화살 (발사 속도 +30%)",
            UpgradeType::HealAndMaxHp => "체력 즉시 +35 회복 (최대 체력 +20)",
            UpgradeType::MagnetRadius => "경험치 자석 반경 +50px 확장",
            UpgradeType::MoveSpeed => "플레이어 이동 속도 +15% 증가",
        }
    }

    pub fn desc(&self) -> &'static str {
        match self {
            UpgradeType::ExtraBlade => "플레이어 주변을 돌며 적을 갈아버리는 칼날이 추가됩니다.",
            UpgradeType::ArrowCooldown => "가장 가까운 적을 탐색해 자동 발사하는 번개 화살의 쿨다운이 대폭 감소합니다.",
            UpgradeType::HealAndMaxHp => "위기의 순간 체력을 채우고 최대 체력 통을 넓힙니다.",
            UpgradeType::MagnetRadius => "멀리 떨어진 경험치 보석을 자석처럼 더 넓은 범위에서 끌어당깁니다.",
            UpgradeType::MoveSpeed => "군집하는 적들을 따돌릴 수 있는 기동력을 확보합니다.",
        }
    }
}

/// 실시간 생존 및 전투 통계 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct SurvivalStats {
    pub survival_time: f32,
    pub kills: u32,
    pub gems_collected: u32,
}

/// 적 스폰 관리 타이머 리소스
#[derive(Resource, Debug, Clone)]
pub struct EnemySpawnTimer(pub Timer);

/// 로컬 영구 저장용 게임 최고 기록 (JSON)
#[derive(Resource, Serialize, Deserialize, Debug, Default, Clone)]
pub struct GameRecord {
    pub best_survival_time_sec: f32,
    pub max_kills: u32,
}

impl GameRecord {
    pub fn load_or_default(path: &str) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, json)
    }
}

/// 사운드 에셋 리소스
#[derive(Resource, Debug, Clone)]
pub struct SoundAssets {
    pub hit: Handle<AudioSource>,
    pub levelup: Handle<AudioSource>,
    pub gem: Handle<AudioSource>,
    pub arrow: Handle<AudioSource>,
    pub gameover: Handle<AudioSource>,
}

/// 카메라 쉐이크 트라우마 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct CameraShake {
    pub trauma: f32,
}

impl CameraShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }
}

/// 히트스톱 매니저 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct HitstopTimer {
    pub timer: Option<Timer>,
}

/// 레벨업 카드 선택 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct UpgradeChoices {
    pub choices: Vec<UpgradeType>,
}

/// 페리스 스프라이트 아틀라스 리소스
#[derive(Resource, Debug, Clone)]
pub struct FerrisSpriteAssets {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

