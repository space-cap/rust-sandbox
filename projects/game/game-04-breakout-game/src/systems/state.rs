use bevy::prelude::*;
use crate::components::{
    AppState, GameOverElement, GameScore, MenuElement, PausedElement, VictoryElement,
};

// ==========================================
// 1. Menu 상태 시스템
// ==========================================
pub fn setup_menu_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        MenuElement,
        Text2d::new("Rust Game 04: Breakout (벽돌깨기)\n\n[Space] 또는 [Enter] 키를 눌러 게임을 시작하세요!\n\n조작: [A, D] 또는 [방향키] 패들 이동 | [P] 일시정지"),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.95, 1.0)),
        Transform::from_xyz(0.0, 30.0, 3.0),
    ));
}

pub fn cleanup_menu_overlay(mut commands: Commands, query: Query<Entity, With<MenuElement>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn menu_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::Playing);
    }
}

// ==========================================
// 2. Pause 상태 시스템
// ==========================================
pub fn setup_pause_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        PausedElement,
        Text2d::new("⏸️ 일시정지 (PAUSED)\n\n[P] 또는 [Space] 키를 누르면 계속 진행됩니다."),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Transform::from_xyz(0.0, 30.0, 3.0),
    ));
}

pub fn cleanup_pause_overlay(mut commands: Commands, query: Query<Entity, With<PausedElement>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn pause_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) || keyboard.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Playing);
    }
}

pub fn playing_pause_toggle_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        next_state.set(AppState::Paused);
    }
}

// ==========================================
// 3. GameOver 상태 시스템
// ==========================================
pub fn setup_gameover_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<GameScore>,
) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        GameOverElement,
        Text2d::new(format!(
            "💥 GAME OVER (게임 오버)\n\n최종 점수: {}점\n남은 벽돌: {}개\n\n[R] 또는 [Space] 키를 눌러 재도전하세요!",
            score.score, score.remaining_bricks
        )),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        Transform::from_xyz(0.0, 30.0, 3.0),
    ));
}

pub fn cleanup_gameover_overlay(
    mut commands: Commands,
    query: Query<Entity, With<GameOverElement>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn gameover_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<GameScore>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) || keyboard.just_pressed(KeyCode::Space) {
        score.score = 0;
        score.lives = 3;
        next_state.set(AppState::Playing);
    }
}

// ==========================================
// 4. Victory 상태 시스템
// ==========================================
pub fn setup_victory_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<GameScore>,
) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        VictoryElement,
        Text2d::new(format!(
            "🎉 VICTORY! ALL BRICKS CLEARED!\n\n완벽합니다! 모든 벽돌을 파괴했습니다!\n최종 점수: {}점\n\n[R] 또는 [Space] 키를 눌러 다시 도전하세요!",
            score.score
        )),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.3, 1.0, 0.4)),
        Transform::from_xyz(0.0, 30.0, 3.0),
    ));
}

pub fn cleanup_victory_overlay(
    mut commands: Commands,
    query: Query<Entity, With<VictoryElement>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn victory_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<GameScore>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) || keyboard.just_pressed(KeyCode::Space) {
        score.score = 0;
        score.lives = 3;
        next_state.set(AppState::Playing);
    }
}

