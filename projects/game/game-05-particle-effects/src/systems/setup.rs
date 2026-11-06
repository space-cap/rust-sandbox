use bevy::prelude::*;
use crate::components::{
    Boundary, CannonCooldown, Collider2d, HudText, PlayerCannon, SoundAssets, TargetDummy,
};

/// 게임 초기화: 2D 카메라, 사운드, 플레이어 캐논, 타깃 더미 및 안내 HUD 스폰
pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 1. 2D 카메라
    commands.spawn(Camera2d);

    // 2. 사운드 에셋 리소스 등록
    commands.insert_resource(SoundAssets {
        laser: asset_server.load("audio/laser.wav"),
        explosion: asset_server.load("audio/explosion.wav"),
    });

    // 3. 한글 폰트 로드
    let font = asset_server.load("fonts/malgun.ttf");

    // 4. 플레이어 에너지 캐논 스폰
    commands.spawn((
        PlayerCannon,
        Collider2d::new(42.0, 42.0),
        CannonCooldown(Timer::from_seconds(0.20, TimerMode::Once)),
        Boundary::new(-420.0, 420.0, -260.0, -80.0),
        Sprite::from_color(
            Color::srgb(0.2, 0.85, 1.0), // 네온 사이언
            Vec2::new(42.0, 42.0),
        ),
        Transform::from_xyz(0.0, -210.0, 1.0),
    ));

    // 5. 타깃 더미 5개 스폰 (상단 필드 가로 배치)
    let target_spacing = 160.0;
    for i in 0..5 {
        let x = (i as f32 - 2.0) * target_spacing;
        commands.spawn((
            TargetDummy {
                health: 3,
                max_health: 3,
            },
            Collider2d::new(50.0, 50.0),
            Sprite::from_color(
                Color::srgb(1.0, 0.45, 0.2), // 화염 오렌지
                Vec2::new(50.0, 50.0),
            ),
            Transform::from_xyz(x, 110.0, 1.0),
        ));
    }

    // 6. 상단 안내 헤더 텍스트
    commands.spawn((
        Text2d::new("Rust Game 05: Particle Bursts & Camera Shake (Juice & Polish)\n[A, D] / [방향키] 이동 | [Space] 쇼크웨이브 포 발사 | [G] 디버그 기즈모 On/Off"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.95, 1.0)),
        Transform::from_xyz(0.0, 255.0, 2.0),
    ));

    // 7. 하단 실시간 HUD 텍스트 (타깃 파괴, 파티클 수, 카메라 쉐이크 강도, 기즈모 상태)
    commands.spawn((
        HudText,
        Text2d::new("타깃 파괴: 0대 | 발사 수: 0발 | 활성 파티클: 0개 | 쉐이크 Trauma: 0.00 | 기즈모: [ON]"),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Transform::from_xyz(0.0, -265.0, 2.0),
    ));
}

