use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// 게임의 4대 상태 머신
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    Playing,
    LevelUp,
    GameOver,
}

/// 10대 기본 몬스터 종족 실루엣
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonsterArchetype {
    Slime,
    Bat,
    Goblin,
    Skeleton,
    Spider,
    Golem,
    Ghost,
    Mimic,
    Warlock,
    Dragon,
}

impl MonsterArchetype {
    pub fn all() -> [MonsterArchetype; 10] {
        [
            MonsterArchetype::Slime,
            MonsterArchetype::Bat,
            MonsterArchetype::Goblin,
            MonsterArchetype::Skeleton,
            MonsterArchetype::Spider,
            MonsterArchetype::Golem,
            MonsterArchetype::Ghost,
            MonsterArchetype::Mimic,
            MonsterArchetype::Warlock,
            MonsterArchetype::Dragon,
        ]
    }

    pub fn base_name(&self) -> &'static str {
        match self {
            MonsterArchetype::Slime => "슬라임",
            MonsterArchetype::Bat => "박쥐",
            MonsterArchetype::Goblin => "고블린",
            MonsterArchetype::Skeleton => "스켈레톤",
            MonsterArchetype::Spider => "거미",
            MonsterArchetype::Golem => "골렘",
            MonsterArchetype::Ghost => "유령",
            MonsterArchetype::Mimic => "미믹",
            MonsterArchetype::Warlock => "흑마법사",
            MonsterArchetype::Dragon => "드래곤",
        }
    }

    pub fn base_stats(&self) -> (f32, f32, f32, Vec2) {
        // (HP, Speed, Damage, Size)
        match self {
            MonsterArchetype::Slime => (24.0, 95.0, 10.0, Vec2::new(24.0, 24.0)),
            MonsterArchetype::Bat => (16.0, 150.0, 8.0, Vec2::new(20.0, 20.0)),
            MonsterArchetype::Goblin => (35.0, 110.0, 14.0, Vec2::new(26.0, 26.0)),
            MonsterArchetype::Skeleton => (45.0, 85.0, 16.0, Vec2::new(28.0, 30.0)),
            MonsterArchetype::Spider => (28.0, 130.0, 12.0, Vec2::new(24.0, 22.0)),
            MonsterArchetype::Golem => (110.0, 60.0, 25.0, Vec2::new(38.0, 38.0)),
            MonsterArchetype::Ghost => (30.0, 100.0, 15.0, Vec2::new(26.0, 28.0)),
            MonsterArchetype::Mimic => (60.0, 75.0, 22.0, Vec2::new(28.0, 26.0)),
            MonsterArchetype::Warlock => (50.0, 80.0, 18.0, Vec2::new(28.0, 32.0)),
            MonsterArchetype::Dragon => (90.0, 115.0, 28.0, Vec2::new(36.0, 34.0)),
        }
    }
}

/// 5대 원소 속성 및 색상 틴트
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonsterElement {
    Normal,
    Fire,
    Ice,
    Thunder,
    Void,
}

impl MonsterElement {
    pub fn all() -> [MonsterElement; 5] {
        [
            MonsterElement::Normal,
            MonsterElement::Fire,
            MonsterElement::Ice,
            MonsterElement::Thunder,
            MonsterElement::Void,
        ]
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            MonsterElement::Normal => "",
            MonsterElement::Fire => "화염의 ",
            MonsterElement::Ice => "빙결의 ",
            MonsterElement::Thunder => "전격의 ",
            MonsterElement::Void => "심연의 ",
        }
    }

    pub fn color_tint(&self) -> Color {
        match self {
            MonsterElement::Normal => Color::srgb(1.0, 1.0, 1.0),
            MonsterElement::Fire => Color::srgb(1.0, 0.35, 0.2),
            MonsterElement::Ice => Color::srgb(0.3, 0.8, 1.0),
            MonsterElement::Thunder => Color::srgb(1.0, 0.95, 0.2),
            MonsterElement::Void => Color::srgb(0.75, 0.2, 0.9),
        }
    }
}

/// 8대 특수 접두사 변종 (Affix)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonsterAffix {
    None,
    Colossal,  // 거대한 (크기 1.8x, 체력 3x)
    Swift,     // 신속의 (속도 +75%)
    Volatile,  // 자폭하는 (사망 시 광역 폭발)
    Splitting, // 분열하는 (사망 시 소형 2마리 분열)
    Shielded,  // 보호막 (첫 3타 무효)
    Shooter,   // 원거리 탄환 발사
    Golden,    // 황금의 (대량 루비 보석 드랍)
}

impl MonsterAffix {
    pub fn all() -> [MonsterAffix; 8] {
        [
            MonsterAffix::None,
            MonsterAffix::Colossal,
            MonsterAffix::Swift,
            MonsterAffix::Volatile,
            MonsterAffix::Splitting,
            MonsterAffix::Shielded,
            MonsterAffix::Shooter,
            MonsterAffix::Golden,
        ]
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            MonsterAffix::None => "",
            MonsterAffix::Colossal => "거대한 ",
            MonsterAffix::Swift => "신속의 ",
            MonsterAffix::Volatile => "자폭하는 ",
            MonsterAffix::Splitting => "분열하는 ",
            MonsterAffix::Shielded => "방어막 ",
            MonsterAffix::Shooter => "저격하는 ",
            MonsterAffix::Golden => "황금 럭키 ",
        }
    }
}

/// 몬스터 엔티티 컴포넌트
#[derive(Component, Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub archetype: MonsterArchetype,
    pub element: MonsterElement,
    pub affix: MonsterAffix,
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub damage: f32,
    pub exp_value: f32,
    pub shield_hits: u32,
    pub shoot_timer: Option<Timer>,
    pub is_boss: bool,
}

/// 90초 타이탄 에픽 보스 컴포넌트
#[derive(Component, Debug, Clone)]
pub struct BossTitan {
    pub radial_timer: Timer,
    pub enraged: bool,
}

/// 보스 방사형 탄환 컴포넌트
#[derive(Component, Debug, Clone)]
pub struct BossProjectile {
    pub velocity: Vec2,
    pub damage: f32,
    pub lifetime: Timer,
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
    // 대시 (Dash) 시스템
    pub is_dashing: bool,
    pub dash_timer: Timer,
    pub dash_cooldown: Timer,
    pub dash_dir: Vec2,
    pub invulnerable: bool,
    // 4종 무기
    pub orbit_count: u32,
    pub orbit_speed: f32,
    pub orbit_damage: f32,
    pub arrow_cooldown: Timer,
    pub arrow_damage: f32,
    pub holy_water_cooldown: Timer,
    pub holy_water_damage: f32,
    pub axe_cooldown: Timer,
    pub axe_damage: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 120.0,
            max_health: 120.0,
            speed: 250.0,
            level: 1,
            exp: 0.0,
            next_level_exp: 15.0,
            magnet_radius: 130.0,
            is_dashing: false,
            dash_timer: Timer::from_seconds(0.18, TimerMode::Once),
            dash_cooldown: Timer::from_seconds(1.6, TimerMode::Once),
            dash_dir: Vec2::Y,
            invulnerable: false,
            orbit_count: 1,
            orbit_speed: 2.8,
            orbit_damage: 24.0,
            arrow_cooldown: Timer::from_seconds(0.85, TimerMode::Repeating),
            arrow_damage: 38.0,
            holy_water_cooldown: Timer::from_seconds(3.0, TimerMode::Repeating),
            holy_water_damage: 18.0,
            axe_cooldown: Timer::from_seconds(2.2, TimerMode::Repeating),
            axe_damage: 55.0,
        }
    }
}

/// 대시 잔상 (Ghost Trail) 컴포넌트
#[derive(Component, Debug, Clone)]
pub struct GhostTrail {
    pub lifetime: Timer,
}

/// 플로팅 데미지 텍스트 (Floating Combat Text - FCT)
#[derive(Component, Debug, Clone)]
pub struct FloatingDamageText {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub is_crit: bool,
}

/// 지면 성수 불꽃 장판 (Holy Water Fire Pool)
#[derive(Component, Debug, Clone)]
pub struct HolyWaterPool {
    pub lifetime: Timer,
    pub tick_timer: Timer,
    pub damage: f32,
    pub radius: f32,
}

/// 포물선 관통 회전 도끼 (Parabolic Axe)
#[derive(Component, Debug, Clone)]
pub struct ParabolicAxe {
    pub velocity: Vec2,
    pub gravity: f32,
    pub damage: f32,
    pub lifetime: Timer,
}

/// 궤도 마법 칼날
#[derive(Component, Debug, Clone)]
pub struct OrbitBlade {
    pub index: u32,
    pub radius: f32,
    pub angle: f32,
    pub angular_speed: f32,
    pub damage: f32,
}

/// 최단 거리 자동 유도 번개 화살
#[derive(Component, Debug, Clone)]
pub struct HomingArrow {
    pub target_pos: Vec2,
    pub speed: f32,
    pub damage: f32,
    pub lifetime: Timer,
}

/// 경험치 보석 (루비 vs 에메랄드)
#[derive(Component, Debug, Clone)]
pub struct ExpGem {
    pub value: f32,
    pub is_ruby: bool,
    pub is_attracted: bool,
}

/// 2D AABB 충돌체
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Collider2d {
    pub size: Vec2,
}

impl Collider2d {
    pub fn new(width: f32, height: f32) -> Self {
        Self { size: Vec2::new(width, height) }
    }

    pub fn intersects(&self, pos_a: Vec2, other: &Collider2d, pos_b: Vec2) -> bool {
        let half_a = self.size * 0.5;
        let half_b = other.size * 0.5;

        (pos_a.x - pos_b.x).abs() < (half_a.x + half_b.x)
            && (pos_a.y - pos_b.y).abs() < (half_a.y + half_b.y)
    }
}

/// 속도 벡터
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vec2);

/// 파티클 이펙트
#[derive(Component, Debug, Clone)]
pub struct Particle {
    pub lifetime: Timer,
    pub drag: f32,
    pub base_color: Color,
}

/// 페리스 애니메이션 타이머 & 인덱스
#[derive(Component, Debug, Clone)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Debug, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PlayerFacing {
    pub flip_x: bool,
}

// -------------------------------------------------------------------------
// HUD 및 UI 마커 컴포넌트
// -------------------------------------------------------------------------

#[derive(Component, Debug, Clone, Copy)]
pub struct HudHealthText;

#[derive(Component, Debug, Clone, Copy)]
pub struct HudExpText;

#[derive(Component, Debug, Clone, Copy)]
pub struct HudDashText;

#[derive(Component, Debug, Clone, Copy)]
pub struct HudTimeKillsText;

#[derive(Component, Debug, Clone, Copy)]
pub struct BossHpBarRoot;

#[derive(Component, Debug, Clone, Copy)]
pub struct BossHpBarFill;

#[derive(Component, Debug, Clone, Copy)]
pub struct BossHpBarText;

#[derive(Component, Debug, Clone, Copy)]
pub struct MenuUi;

#[derive(Component, Debug, Clone, Copy)]
pub struct LevelUpUi;

#[derive(Component, Debug, Clone, Copy)]
pub struct GameOverUi;

// -------------------------------------------------------------------------
// 리소스 정의
// -------------------------------------------------------------------------

/// 100종 몬스터 도감 및 처치 기록 리소스
#[derive(Resource, Serialize, Deserialize, Debug, Default, Clone)]
pub struct MonsterBestiary {
    pub discovered_names: HashSet<String>,
    pub total_kills: u32,
    pub boss_kills: u32,
}

/// 보스 출현 및 상태 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct BossManager {
    pub spawned: bool,
    pub defeated: bool,
    pub boss_entity: Option<Entity>,
}

/// 생존 통계 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct SurvivalStats {
    pub survival_time: f32,
    pub kills: u32,
    pub gems_collected: u32,
    pub damage_dealt: f64,
}

/// 적 스폰 타이머 리소스
#[derive(Resource, Debug, Clone)]
pub struct EnemySpawnTimer(pub Timer);

/// 로컬 영구 세이브 기록
#[derive(Resource, Serialize, Deserialize, Debug, Default, Clone)]
pub struct GameRecord {
    pub best_survival_time_sec: f32,
    pub max_kills: u32,
    pub discovered_monsters_count: usize,
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

/// 오디오 에셋 핸들 모음
#[derive(Resource, Debug, Clone)]
pub struct SoundAssets {
    pub hit: Handle<AudioSource>,
    pub crit_hit: Handle<AudioSource>,
    pub levelup: Handle<AudioSource>,
    pub gem: Handle<AudioSource>,
    pub arrow: Handle<AudioSource>,
    pub dash: Handle<AudioSource>,
    pub holy_water: Handle<AudioSource>,
    pub axe: Handle<AudioSource>,
    pub boss_spawn: Handle<AudioSource>,
    pub boss_shoot: Handle<AudioSource>,
    pub gameover: Handle<AudioSource>,
}

/// 제미나이 생성 이미지 에셋 핸들 모음
#[derive(Resource, Debug, Clone, Default)]
pub struct ImageAssets {
    pub hero_ferris: Option<Handle<Image>>,
    pub enemy_slime: Option<Handle<Image>>,
    pub enemy_bat: Option<Handle<Image>>,
    pub enemy_golem: Option<Handle<Image>>,
    pub boss_titan: Option<Handle<Image>>,
    pub arena_floor: Option<Handle<Image>>,
    pub gem_ruby: Option<Handle<Image>>,
    pub title_banner: Option<Handle<Image>>,
    pub holy_water: Option<Handle<Image>>,
}

/// 카메라 쉐이크 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct CameraShake {
    pub trauma: f32,
}

impl CameraShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }
}

/// 히트스톱 리소스
#[derive(Resource, Debug, Default, Clone)]
pub struct HitstopTimer {
    pub timer: Option<Timer>,
}

/// 레벨업 선택지 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeType {
    ExtraBlade,
    ArrowCooldown,
    HolyWaterPool,
    BouncingAxe,
    HealAndMaxHp,
    MagnetRadius,
    MoveSpeed,
}

impl UpgradeType {
    pub fn title(&self) -> &'static str {
        match self {
            UpgradeType::ExtraBlade => "마법 칼날 +1개 (공전 속도 +20%)",
            UpgradeType::ArrowCooldown => "유도 번개 화살 (발사 속도 +35%)",
            UpgradeType::HolyWaterPool => "성수 화염 장판 (지속 틱 데미지 광역 풀)",
            UpgradeType::BouncingAxe => "관통 회전 도끼 (중력 포물선 관통 비산)",
            UpgradeType::HealAndMaxHp => "체력 즉시 +40 회복 (최대 체력 +25)",
            UpgradeType::MagnetRadius => "경험치 자석 반경 +60px 확장",
            UpgradeType::MoveSpeed => "플레이어 이동 속도 +15% 증가",
        }
    }

    pub fn desc(&self) -> &'static str {
        match self {
            UpgradeType::ExtraBlade => "주변을 회전하며 근접 몬스터를 지속 절단하는 칼날을 추가합니다.",
            UpgradeType::ArrowCooldown => "가장 가까운 적을 탐색해 자동 발사하는 번개 화살 쿨다운을 단축합니다.",
            UpgradeType::HolyWaterPool => "주기적으로 적 무리에 성수를 던져 4초간 불타는 지면 장판을 생성합니다.",
            UpgradeType::BouncingAxe => "높이 솟구쳐 화면 아래로 회전 낙하하며 경로상의 모든 적을 관통합니다.",
            UpgradeType::HealAndMaxHp => "위기의 순간 체력을 회복하고 최대 체력 통을 확장합니다.",
            UpgradeType::MagnetRadius => "멀리 떨어진 보석들을 강력한 자석 힘으로 끌어당깁니다.",
            UpgradeType::MoveSpeed => "군집하는 몬스터들을 따돌리는 빠른 기동력을 확보합니다.",
        }
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct UpgradeChoices {
    pub choices: Vec<UpgradeType>,
}

#[derive(Resource, Debug, Clone)]
pub struct FerrisSpriteAssets {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

