use bevy::prelude::*;
use crate::components::{
    Boundary, Collider2d, HudText, Player, Speed, Star, Velocity, WeaponCooldown,
};
use rand::Rng;

/// 게임 초기화: 2D 카메라, 배경 별무리(Starfield), 플레이어 우주선, HUD 생성
pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 1. 2D 카메라 스폰
    commands.spawn(Camera2d);

    // 2. 한글 맑은 고딕 폰트 로드
    let font = asset_server.load("fonts/malgun.ttf");

    // 3. 우주 배경 별무리(Starfield) 70개 사전 생성 (원근감 구현)
    let mut rng = rand::thread_rng();
    for _ in 0..70 {
        let x = rng.gen_range(-460.0..460.0);
        let y = rng.gen_range(-290.0..290.0);
        let speed = rng.gen_range(60.0..180.0);
        let size = if speed > 130.0 { 3.0 } else { 2.0 };
        let alpha = if speed > 130.0 { 0.9 } else { 0.5 };

        commands.spawn((
            Star,
            Velocity(Vec2::new(0.0, -speed)),
            Sprite::from_color(
                Color::srgba(0.85, 0.9, 1.0, alpha),
                Vec2::new(size, size),
            ),
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    // 4. 플레이어 우주선 스폰 (우주선 형태의 날렵한 스프라이트)
    commands.spawn((
        Player,
        Speed(380.0),
        Collider2d::new(34.0, 34.0),
        WeaponCooldown(Timer::from_seconds(0.18, TimerMode::Once)),
        Boundary::new(-440.0, 440.0, -260.0, 260.0),
        Sprite::from_color(
            Color::srgb(0.15, 0.85, 0.95), // 네온 사이언 우주선
            Vec2::new(36.0, 36.0),
        ),
        Transform::from_xyz(0.0, -220.0, 1.0),
    ));

    // 5. 상단 헤더 안내 텍스트
    commands.spawn((
        Text2d::new("Rust Game 03: Space Shooter & AABB Collision\n[W, A, S, D] 우주선 조작 | [Space] 트윈 레이저 발사 | [R] 재시작"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.95, 1.0)),
        Transform::from_xyz(0.0, 255.0, 2.0),
    ));

    // 6. 하단 실시간 HUD 텍스트 (점수, 쉴드, 처치 수)
    commands.spawn((
        HudText,
        Text2d::new("점수: 0 | 쉴드: [###] (3/3) | 처치 수: 0 | 상태: 전투 비행 중"),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Transform::from_xyz(0.0, -265.0, 2.0),
    ));
}

