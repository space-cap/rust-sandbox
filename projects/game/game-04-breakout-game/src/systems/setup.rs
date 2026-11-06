use bevy::prelude::*;
use crate::components::{
    Ball, Boundary, Brick, Collider2d, GameScore, HudText, Paddle, SoundEffects, Velocity,
};

/// 게임 엔진 시작 시 기본 카메라, 사운드 에셋 및 벽체(Wall) 생성
pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 1. 2D 카메라
    commands.spawn(Camera2d);

    // 2. 사운드 이펙트 리소스 등록
    commands.insert_resource(SoundEffects {
        bounce: asset_server.load("audio/bounce.wav"),
        brick_break: asset_server.load("audio/break.wav"),
        game_over: asset_server.load("audio/gameover.wav"),
    });

    // 3. 한글 폰트 로드
    let font = asset_server.load("fonts/malgun.ttf");

    // 4. 경기장 외곽 벽체 스폰 (좌측 벽, 우측 벽, 천장 벽)
    let wall_color = Color::srgb(0.25, 0.3, 0.4);

    // 좌측 벽
    commands.spawn((
        Collider2d::new(20.0, 560.0),
        Sprite::from_color(wall_color, Vec2::new(20.0, 560.0)),
        Transform::from_xyz(-400.0, 0.0, 0.0),
    ));

    // 우측 벽
    commands.spawn((
        Collider2d::new(20.0, 560.0),
        Sprite::from_color(wall_color, Vec2::new(20.0, 560.0)),
        Transform::from_xyz(400.0, 0.0, 0.0),
    ));

    // 천장 벽
    commands.spawn((
        Collider2d::new(820.0, 20.0),
        Sprite::from_color(wall_color, Vec2::new(820.0, 20.0)),
        Transform::from_xyz(0.0, 270.0, 0.0),
    ));

    // 5. 상단 HUD 텍스트 스폰
    commands.spawn((
        HudText,
        Text2d::new("점수: 0 | 목숨: [♥♥♥] (3) | 남은 벽돌: 0개 | [P] 일시정지"),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Transform::from_xyz(0.0, 245.0, 2.0),
    ));
}

/// 새 게임(Playing) 진입 시 패들, 공 및 40개의 컬러 벽돌 스폰
pub fn spawn_stage_entities(
    mut commands: Commands,
    mut score: ResMut<GameScore>,
    ball_query: Query<Entity, With<Ball>>,
    paddle_query: Query<Entity, With<Paddle>>,
    brick_query: Query<Entity, With<Brick>>,
) {
    // 기존 잔여 엔티티 정리
    for b in ball_query.iter() {
        commands.entity(b).despawn();
    }
    for p in paddle_query.iter() {
        commands.entity(p).despawn();
    }
    for br in brick_query.iter() {
        commands.entity(br).despawn();
    }

    // 1. 패들(플레이어) 스폰
    commands.spawn((
        Paddle,
        Collider2d::new(120.0, 16.0),
        Boundary::new(-330.0, 330.0),
        Sprite::from_color(
            Color::srgb(0.15, 0.85, 0.95), // 네온 사이언
            Vec2::new(120.0, 16.0),
        ),
        Transform::from_xyz(0.0, -230.0, 1.0),
    ));

    // 2. 공(Ball) 스폰
    commands.spawn((
        Ball,
        Velocity(Vec2::new(220.0, 340.0)),
        Collider2d::new(14.0, 14.0),
        Sprite::from_color(
            Color::srgb(1.0, 1.0, 1.0), // 화이트 볼
            Vec2::new(14.0, 14.0),
        ),
        Transform::from_xyz(0.0, -200.0, 1.0),
    ));

    // 3. 벽돌 40개 배치 (4행 x 10열)
    let rows = 4;
    let cols = 10;
    let brick_w = 70.0;
    let brick_h = 20.0;
    let spacing = 6.0;

    let colors = [
        (Color::srgb(0.95, 0.25, 0.25), 40), // 1행: 레드 (40점)
        (Color::srgb(0.95, 0.65, 0.20), 30), // 2행: 골드 (30점)
        (Color::srgb(0.25, 0.85, 0.35), 20), // 3행: 그린 (20점)
        (Color::srgb(0.30, 0.60, 0.95), 10), // 4행: 블루 (10점)
    ];

    let start_x = -((cols as f32 - 1.0) * (brick_w + spacing)) * 0.5;
    let start_y = 190.0;
    let mut count = 0;

    for r in 0..rows {
        let (color, pts) = colors[r];
        let y = start_y - (r as f32) * (brick_h + spacing);

        for c in 0..cols {
            let x = start_x + (c as f32) * (brick_w + spacing);

            commands.spawn((
                Brick { points: pts },
                Collider2d::new(brick_w, brick_h),
                Sprite::from_color(color, Vec2::new(brick_w, brick_h)),
                Transform::from_xyz(x, y, 0.5),
            ));
            count += 1;
        }
    }

    score.total_bricks = count;
    score.remaining_bricks = count;
}

