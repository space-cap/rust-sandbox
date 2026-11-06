use bevy::prelude::*;
use crate::components::{
    AnimationIndices, AnimationTimer, BossHpBarFill, BossHpBarRoot, BossHpBarText, BossManager,
    Collider2d, EnemySpawnTimer, FerrisSpriteAssets, GameRecord, HitstopTimer, HudDashText,
    HudExpText, HudHealthText, HudTimeKillsText, ImageAssets, MonsterBestiary, OrbitBlade,
    Player, PlayerFacing, PlayerStats, SoundAssets, SurvivalStats, UpgradeChoices,
};

/// 게임의 기본 환경(카메라, 에셋, 바닥 타일맵, HUD, 보스바) 초기화
pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 1. 2D 카메라
    commands.spawn(Camera2d);

    // 2. 사운드 에셋 로드 (11종)
    let sound_assets = SoundAssets {
        hit: asset_server.load("audio/hit.wav"),
        crit_hit: asset_server.load("audio/crit_hit.wav"),
        levelup: asset_server.load("audio/levelup.wav"),
        gem: asset_server.load("audio/gem.wav"),
        arrow: asset_server.load("audio/arrow.wav"),
        dash: asset_server.load("audio/dash.wav"),
        holy_water: asset_server.load("audio/holy_water.wav"),
        axe: asset_server.load("audio/axe.wav"),
        boss_spawn: asset_server.load("audio/boss_spawn.wav"),
        boss_shoot: asset_server.load("audio/boss_shoot.wav"),
        gameover: asset_server.load("audio/gameover.wav"),
    };
    commands.insert_resource(sound_assets);

    // 3. 제미나이 생성 이미지 에셋 핸들 초기화
    let image_assets = ImageAssets {
        hero_ferris: Some(asset_server.load("images/hero_ferris.png")),
        enemy_slime: Some(asset_server.load("images/enemy_slime.png")),
        enemy_bat: Some(asset_server.load("images/enemy_bat.png")),
        enemy_golem: Some(asset_server.load("images/enemy_golem.png")),
        boss_titan: Some(asset_server.load("images/boss_titan.png")),
        arena_floor: Some(asset_server.load("images/arena_floor.png")),
        gem_ruby: Some(asset_server.load("images/gem_ruby.png")),
        title_banner: Some(asset_server.load("images/title_banner.png")),
    };
    commands.insert_resource(image_assets.clone());

    // 4. 페리스 스프라이트 아틀라스 (128x64: 셀 32x32, 4열 2행 = 8프레임)
    let ferris_image = asset_server.load("sprites/ferris_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 2, None, None);
    let ferris_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(FerrisSpriteAssets {
        image: ferris_image.clone(),
        layout: ferris_layout.clone(),
    });

    // 5. 기본 리소스 등록
    commands.insert_resource(SurvivalStats::default());
    commands.insert_resource(MonsterBestiary::default());
    commands.insert_resource(BossManager::default());
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(0.9, TimerMode::Repeating)));
    commands.insert_resource(HitstopTimer { timer: None });
    commands.insert_resource(UpgradeChoices::default());

    // 6. 로컬 최고 기록 로드
    let record = GameRecord::load_or_default("assets/save/ultimate_records.json");
    commands.insert_resource(record);

    let font = asset_server.load("fonts/malgun.ttf");

    // 7. 아레나 바닥 타일맵 배경 (어두운 던전 석재 텍스처)
    if let Some(ref floor_img) = image_assets.arena_floor {
        commands.spawn((
            Sprite {
                image: floor_img.clone(),
                custom_size: Some(Vec2::new(920.0, 560.0)),
                color: Color::srgba(0.8, 0.85, 0.95, 0.9),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    } else {
        commands.spawn((
            Sprite::from_color(
                Color::srgba(0.08, 0.09, 0.15, 0.95),
                Vec2::new(920.0, 560.0),
            ),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }

    // 바닥 테두리 네온 라인
    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.25, 0.35, 0.6, 0.35),
            Vec2::new(900.0, 540.0),
        ),
        Transform::from_xyz(0.0, 0.0, 0.1),
    ));

    // 8. 플레이어 영웅 페리스 스폰
    commands.spawn((
        Player,
        PlayerStats::default(),
        Collider2d::new(36.0, 36.0),
        PlayerFacing { flip_x: false },
        Sprite {
            image: ferris_image,
            texture_atlas: Some(TextureAtlas {
                layout: ferris_layout,
                index: 0,
            }),
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 2.0),
        AnimationIndices { first: 0, last: 3 },
        AnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
    ));

    // 9. 초기 궤도 마법 칼날 1개 스폰
    commands.spawn((
        OrbitBlade {
            index: 0,
            radius: 80.0,
            angle: 0.0,
            angular_speed: 2.8,
            damage: 24.0,
        },
        Collider2d::new(18.0, 18.0),
        Sprite::from_color(
            Color::srgb(0.2, 0.9, 1.0),
            Vec2::new(18.0, 18.0),
        ),
        Transform::from_xyz(80.0, 0.0, 2.5),
    ));

    // 10. 상단 HUD 텍스트들
    // 체력 바
    commands.spawn((
        HudHealthText,
        Text2d::new("체력: 120/120 [##########]"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(0.2, 1.0, 0.4)),
        Transform::from_xyz(-330.0, 275.0, 5.0),
    ));

    // EXP 바
    commands.spawn((
        HudExpText,
        Text2d::new("Lv.1 EXP: 0/15 [----------]"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(0.3, 0.85, 1.0)),
        Transform::from_xyz(-100.0, 275.0, 5.0),
    ));

    // 대시 쿨다운 표시
    commands.spawn((
        HudDashText,
        Text2d::new("[Shift/Space 대시: READY]"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.7, 1.0)),
        Transform::from_xyz(120.0, 275.0, 5.0),
    ));

    // 생존 시간 & 처치 수
    commands.spawn((
        HudTimeKillsText,
        Text2d::new("생존: 00:00 | 처치: 0마리 (0/100종)"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.9, 0.2)),
        Transform::from_xyz(330.0, 275.0, 5.0),
    ));

    // 11. 상단 보스 체력바 컨테이너 (보스 등장 전까지 숨김 위치)
    commands.spawn((
        BossHpBarRoot,
        Sprite::from_color(
            Color::srgba(0.1, 0.05, 0.05, 0.8),
            Vec2::new(420.0, 20.0),
        ),
        Transform::from_xyz(0.0, 1000.0, 6.0), // 대기 위치
    )).with_children(|parent| {
        parent.spawn((
            BossHpBarFill,
            Sprite::from_color(
                Color::srgb(0.95, 0.15, 0.2), // 보스 블러드 레드 게이지
                Vec2::new(416.0, 16.0),
            ),
            Transform::from_xyz(0.0, 0.0, 0.1),
        ));
        parent.spawn((
            BossHpBarText,
            Text2d::new("⚠️ BOSS: 메모리 리크 타이탄 (HP: 1200 / 1200)"),
            TextFont {
                font: font.into(),
                font_size: bevy::text::FontSize::Px(13.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.9, 0.9)),
            Transform::from_xyz(0.0, 0.0, 0.2),
        ));
    });
}

