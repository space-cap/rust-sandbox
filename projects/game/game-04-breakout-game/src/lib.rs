pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::{AppState, GameScore};
use systems::{
    ball::ball_movement_system,
    collision::breakout_collision_system,
    hud::hud_update_system,
    paddle::paddle_movement_system,
    setup::{setup_game, spawn_stage_entities},
    state::{
        cleanup_gameover_overlay, cleanup_menu_overlay, cleanup_pause_overlay,
        cleanup_victory_overlay, gameover_input_system, menu_input_system, pause_input_system,
        playing_pause_toggle_system, setup_gameover_overlay, setup_menu_overlay,
        setup_pause_overlay, setup_victory_overlay, victory_input_system,
    },
};

/// 벽돌깨기 게임 플러그인
pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<GameScore>()
            .add_systems(Startup, setup_game)
            // 1. Menu 상태
            .add_systems(OnEnter(AppState::Menu), setup_menu_overlay)
            .add_systems(OnExit(AppState::Menu), cleanup_menu_overlay)
            .add_systems(Update, menu_input_system.run_if(in_state(AppState::Menu)))
            // 2. Playing 상태 (스테이지 엔티티 스폰 및 물리 루프)
            .add_systems(OnEnter(AppState::Playing), spawn_stage_entities)
            .add_systems(
                Update,
                (
                    paddle_movement_system,
                    ball_movement_system,
                    breakout_collision_system,
                    playing_pause_toggle_system,
                )
                    .run_if(in_state(AppState::Playing)),
            )
            // 3. Paused 상태
            .add_systems(OnEnter(AppState::Paused), setup_pause_overlay)
            .add_systems(OnExit(AppState::Paused), cleanup_pause_overlay)
            .add_systems(
                Update,
                pause_input_system.run_if(in_state(AppState::Paused)),
            )
            // 4. GameOver 상태
            .add_systems(OnEnter(AppState::GameOver), setup_gameover_overlay)
            .add_systems(OnExit(AppState::GameOver), cleanup_gameover_overlay)
            .add_systems(
                Update,
                gameover_input_system.run_if(in_state(AppState::GameOver)),
            )
            // 5. Victory 상태
            .add_systems(OnEnter(AppState::Victory), setup_victory_overlay)
            .add_systems(OnExit(AppState::Victory), cleanup_victory_overlay)
            .add_systems(
                Update,
                victory_input_system.run_if(in_state(AppState::Victory)),
            )
            // 상시 실행: HUD 갱신
            .add_systems(Update, hud_update_system);
    }
}

