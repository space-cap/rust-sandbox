use bevy::prelude::*;
use crate::components::{
    AnimationIndices, AnimationState, AnimationTimer, Boundary, HudInfoText, Player, Speed,
};

/// 게임 시작 시 카메라, 폰트, 스프라이트 아틀라스 및 플레이어 엔티티 생성
pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 1. 2D 카메라 스폰
    commands.spawn(Camera2d);

    // 2. 한글 맑은 고딕 폰트 로드
    let font = asset_server.load("fonts/malgun.ttf");

    // 3. 페리스 스프라이트 시트 이미지 및 텍스처 아틀라스 레이아웃 설정
    // 128x64 크기 (셀: 32x32, 가로 4칸 x 세로 2칸 = 총 8개 프레임)
    let texture = asset_server.load("sprites/ferris_sheet.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        4,
        2,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // 4. 플레이어 엔티티 스폰 (스프라이트 아틀라스 + 인덱스 + 타이머 + 상태 + 이동속도)
    commands.spawn((
        Player,
        Speed(280.0),
        AnimationState::Idle,
        AnimationIndices::IDLE,
        AnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
        Boundary::new(-430.0, 430.0, -250.0, 250.0),
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: AnimationIndices::IDLE.first,
            }),
            ..default()
        },
        // 픽셀 아트를 선명하게 2.5배 확대 (32px -> 80px)
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.5)),
    ));

    // 5. 상단 헤더 안내 텍스트
    commands.spawn((
        Text2d::new("Rust Game 02: 2D Sprite Sheet & Frame Animation\n[W, A, S, D] 또는 방향키로 페리스를 움직여보세요!"),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.95, 1.0)),
        Transform::from_xyz(0.0, 240.0, 1.0),
    ));

    // 6. 하단 실시간 HUD 텍스트 (상태, 현재 아틀라스 프레임 인덱스, 속도 등 표시)
    commands.spawn((
        HudInfoText,
        Text2d::new("상태: Idle (대기) | 프레임 Index: 0 | 속도: 0 px/s | 위치: (0.0, 0.0)"),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Transform::from_xyz(0.0, -255.0, 1.0),
    ));
}

