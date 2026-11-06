pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::{AppState, CameraShake, HitstopTimer};

/// 히트스톱 중이 아닐 때만 true를 반환하는 런 조건
pub fn not_in_hitstop(hitstop: Res<HitstopTimer>) -> bool {
    hitstop.timer.is_none()
}

/// Phase 6: 2D 서바이벌 아레나 종합 실전 플러그인
pub struct SurvivalArenaPlugin;

impl Plugin for SurvivalArenaPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<CameraShake>()
            .init_resource::<HitstopTimer>()
            .add_systems(Startup, systems::setup::setup_game)
            // -------------------------------------------------------------
            // 상태 머신 생명주기 이벤트 (OnEnter / OnExit)
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::Menu), systems::ui::setup_menu_ui)
            .add_systems(OnExit(AppState::Menu), systems::ui::cleanup_menu_ui)
            .add_systems(OnEnter(AppState::LevelUp), systems::ui::setup_levelup_ui)
            .add_systems(OnExit(AppState::LevelUp), systems::ui::cleanup_levelup_ui)
            .add_systems(OnEnter(AppState::GameOver), systems::ui::setup_gameover_ui)
            .add_systems(OnExit(AppState::GameOver), systems::ui::cleanup_gameover_ui)
            // -------------------------------------------------------------
            // UI 입력 제어 시스템 (각 State별 격리)
            // -------------------------------------------------------------
            .add_systems(
                Update,
                systems::ui::menu_input_system.run_if(in_state(AppState::Menu)),
            )
            .add_systems(
                Update,
                systems::ui::levelup_input_system.run_if(in_state(AppState::LevelUp)),
            )
            .add_systems(
                Update,
                systems::ui::gameover_input_system.run_if(in_state(AppState::GameOver)),
            )
            // -------------------------------------------------------------
            // 전투 시뮬레이션 시스템 (Playing 상태에서만 실행)
            // -------------------------------------------------------------
            .add_systems(
                Update,
                (
                    systems::ui::survival_timer_system,
                    systems::ui::hud_update_system,
                    systems::player::player_movement_system,
                    systems::enemy::enemy_spawn_system,
                    systems::enemy::enemy_ai_and_player_damage_system,
                    systems::enemy::enemy_death_system,
                    systems::weapons::orbit_blade_system,
                    systems::weapons::homing_arrow_spawner_system,
                    systems::weapons::homing_arrow_movement_system.run_if(not_in_hitstop),
                    systems::gem::exp_gem_system,
                    systems::particle::particle_update_system.run_if(not_in_hitstop),
                    systems::shake::hitstop_system,
                )
                    .run_if(in_state(AppState::Playing)),
            )
            // -------------------------------------------------------------
            // 전역 시스템 (화면 쉐이크 감쇠)
            // -------------------------------------------------------------
            .add_systems(Update, systems::shake::camera_shake_system);
    }
}

