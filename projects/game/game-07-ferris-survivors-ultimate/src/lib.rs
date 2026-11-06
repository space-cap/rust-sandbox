pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::{AppState, CameraShake, HitstopTimer};

/// 히트스톱 중이 아닐 때만 true를 반환하는 런 조건
pub fn not_in_hitstop(hitstop: Res<HitstopTimer>) -> bool {
    hitstop.timer.is_none()
}

/// Phase 7: Ferris Survivors Ultimate 플러그인
pub struct FerrisSurvivorsUltimatePlugin;

impl Plugin for FerrisSurvivorsUltimatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<CameraShake>()
            .init_resource::<HitstopTimer>()
            .add_systems(Startup, systems::setup::setup_game)
            // -------------------------------------------------------------
            // 상태 머신 생명주기 이벤트
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::Menu), systems::ui::setup_menu_ui)
            .add_systems(OnExit(AppState::Menu), systems::ui::cleanup_menu_ui)
            .add_systems(OnEnter(AppState::LevelUp), systems::ui::setup_levelup_ui)
            .add_systems(OnExit(AppState::LevelUp), systems::ui::cleanup_levelup_ui)
            .add_systems(OnEnter(AppState::GameOver), systems::ui::setup_gameover_ui)
            .add_systems(OnExit(AppState::GameOver), systems::ui::cleanup_gameover_ui)
            // -------------------------------------------------------------
            // UI 입력 시스템
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
            // 본 게임 전투 시뮬레이션 시스템 (Playing 상태)
            // -------------------------------------------------------------
            .add_systems(
                Update,
                (
                    systems::ui::survival_timer_system,
                    systems::ui::hud_and_boss_bar_system,
                    systems::player::player_movement_and_dash_system,
                    systems::player::ghost_trail_update_system,
                    systems::enemy::enemy_spawn_system,
                    systems::enemy::boss_spawn_system,
                    systems::enemy::enemy_ai_and_boss_system,
                    systems::enemy::boss_projectile_system,
                    systems::enemy::enemy_death_and_affix_system,
                    systems::weapons::orbit_blade_system,
                    systems::weapons::homing_arrow_spawner_system,
                    systems::weapons::homing_arrow_movement_system.run_if(not_in_hitstop),
                    systems::weapons::holy_water_spawner_and_damage_system,
                    systems::weapons::parabolic_axe_system.run_if(not_in_hitstop),
                    systems::gem::exp_gem_system,
                    systems::ui::floating_damage_text_system,
                    systems::particle::particle_update_system.run_if(not_in_hitstop),
                    systems::shake::hitstop_system,
                )
                    .run_if(in_state(AppState::Playing)),
            )
            // -------------------------------------------------------------
            // 전역 시스템
            // -------------------------------------------------------------
            .add_systems(Update, systems::shake::camera_shake_system);
    }
}

