use bevy::prelude::*;
use crate::components::{Boundary, HudInfoText, Player, Speed};

/// 게임 시작 시 실행되는 초기화 시스템
/// - 2D 카메라 생성
/// - 플레이어 스프라이트 개체 생성
/// - 게임 조작 안내 및 HUD 텍스트 생성 (맑은 고딕 한글 폰트 적용)
pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 윈도우 한글 맑은 고딕 폰트 로드
    let font = asset_server.load("fonts/malgun.ttf");

    // 1. 2D 카메라 스폰
    commands.spawn(Camera2d);

    // 2. 플레이어 개체 스폰 (스프라이트 + 위치 + 마커 + 속도 + 경계)
    commands.spawn((
        Player,
        Speed(350.0),
        Boundary::new(-440.0, 440.0, -250.0, 250.0),
        Sprite::from_color(Color::srgb(0.2, 0.75, 0.95), Vec2::new(44.0, 44.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 3. 상단 헤더 안내 텍스트 스폰 (한글 지원)
    commands.spawn((
        Text2d::new("Rust Game 01: Bevy ECS 2D Basics\n[W, A, S, D] 또는 방향키로 페리스 블록을 조작하세요!"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.95, 1.0)),
        Transform::from_xyz(0.0, 240.0, 1.0),
    ));

    // 4. 하단 실시간 HUD 텍스트 스폰 (한글 지원)
    commands.spawn((
        HudInfoText,
        Text2d::new("위치: (X: 0.0, Y: 0.0) | 속도: 350 px/s | 누적 이동: 0.0 px | 프레임: 0"),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Transform::from_xyz(0.0, -255.0, 1.0),
    ));
}
