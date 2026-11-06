use bevy::prelude::*;
use crate::components::{
    AnimationIndices, AnimationTimer, Collider2d, EnemySpawnTimer, FerrisSpriteAssets,
    GameRecord, HitstopTimer, HudExpText, HudHealthText, HudTimeKillsText, OrbitBlade,
    Player, PlayerFacing, PlayerStats, SoundAssets, SurvivalStats, UpgradeChoices,
};

/// 게임의 기본 환경(카메라, 에셋, 폰트, HUD) 및 초기 플레이어 엔티티 스폰
pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 1. 2D 카메라
    commands.spawn(Camera2d);

    // 2. 사운드 에셋 로드
    let sound_assets = SoundAssets {
        hit: asset_server.load("audio/hit.wav"),
        levelup: asset_server.load("audio/levelup.wav"),
        gem: asset_server.load("audio/gem.wav"),
        arrow: asset_server.load("audio/arrow.wav"),
        gameover: asset_server.load("audio/gameover.wav"),
    };
    commands.insert_resource(sound_assets);

    // 3. 페리스 스프라이트 아틀라스 로드 (8개 프레임: 32x32)
    let ferris_image = asset_server.load("sprites/ferris_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 8, 1, None, None);
    let ferris_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(FerrisSpriteAssets {
        image: ferris_image.clone(),
        layout: ferris_layout.clone(),
    });

    // 4. 로컬 최고 기록 로드 (JSON)
    let record = GameRecord::load_or_default("assets/save/high_score.json");
    commands.insert_resource(record);

    // 5. 기본 리소스 초기화
    commands.insert_resource(SurvivalStats::default());
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
    commands.insert_resource(HitstopTimer { timer: None });
    commands.insert_resource(UpgradeChoices::default());

    // 6. 한글 폰트 로드
    let font = asset_server.load("fonts/malgun.ttf");

    // 7. 아레나 외곽선 가이드라인 (배경 사각 테두리)
    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.12, 0.14, 0.22, 0.6),
            Vec2::new(900.0, 560.0),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 8. 플레이어(페리스) 스폰
    let player_stats = PlayerStats::default();
    let player_entity = commands
        .spawn((
            Player,
            player_stats,
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
        ))
        .id();

    // 9. 초기 궤도 마법 칼날 1개 스폰
    commands.spawn((
        OrbitBlade {
            index: 0,
            radius: 80.0,
            angle: 0.0,
            angular_speed: 3.2,
            damage: 22.0,
        },
        Collider2d::new(18.0, 18.0),
        Sprite::from_color(
            Color::srgb(0.2, 0.9, 1.0), // 네온 사이언 블레이드
            Vec2::new(18.0, 18.0),
        ),
        Transform::from_xyz(80.0, 0.0, 2.5),
    ));

    // 10. 상단 HUD 정보 텍스트 (체력, EXP, 생존 시간)
    // 체력 게이지
    commands.spawn((
        HudHealthText,
        Text2d::new("체력: 100 / 100 [##########]"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(0.2, 1.0, 0.4)),
        Transform::from_xyz(-280.0, 270.0, 5.0),
    ));

    // EXP 게이지
    commands.spawn((
        HudExpText,
        Text2d::new("Lv.1 EXP: 0 / 15 [----------]"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(0.3, 0.85, 1.0)),
        Transform::from_xyz(0.0, 270.0, 5.0),
    ));

    // 생존 시간 & 처치 수
    commands.spawn((
        HudTimeKillsText,
        Text2d::new("생존 시간: 00:00 | 처치 수: 0마리"),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.9, 0.2)),
        Transform::from_xyz(280.0, 270.0, 5.0),
    ));

    let _ = player_entity;
}

