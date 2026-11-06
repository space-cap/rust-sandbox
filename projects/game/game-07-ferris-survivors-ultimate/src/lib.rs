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
            // 1. 타이틀 씬 (Title Scene)
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::Title), systems::ui::setup_title_scene)
            .add_systems(OnExit(AppState::Title), systems::ui::cleanup_title_scene)
            .add_systems(
                Update,
                (
                    systems::ui::title_input_system,
                    systems::ui::title_logo_animation_system,
                )
                    .run_if(in_state(AppState::Title)),
            )
            // -------------------------------------------------------------
            // 2. 100종 몬스터 도감 씬 (Bestiary Scene)
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::Bestiary), systems::ui::setup_bestiary_scene)
            .add_systems(OnExit(AppState::Bestiary), systems::ui::cleanup_bestiary_scene)
            .add_systems(
                Update,
                systems::ui::bestiary_input_system.run_if(in_state(AppState::Bestiary)),
            )
            // -------------------------------------------------------------
            // 3. 조작법 & 튜토리얼 씬 (Tutorial Scene)
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::Tutorial), systems::ui::setup_tutorial_scene)
            .add_systems(OnExit(AppState::Tutorial), systems::ui::cleanup_tutorial_scene)
            .add_systems(
                Update,
                systems::ui::tutorial_input_system.run_if(in_state(AppState::Tutorial)),
            )
            // -------------------------------------------------------------
            // 4. 일시정지 씬 (Paused Scene - ESC)
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::Paused), systems::ui::setup_paused_scene)
            .add_systems(OnExit(AppState::Paused), systems::ui::cleanup_paused_scene)
            .add_systems(
                Update,
                systems::ui::paused_input_system.run_if(in_state(AppState::Paused)),
            )
            // -------------------------------------------------------------
            // 5. 보스 강림 컷씬 (Boss Intro Scene)
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::BossIntro), systems::ui::setup_boss_intro_scene)
            .add_systems(OnExit(AppState::BossIntro), systems::ui::cleanup_boss_intro_scene)
            .add_systems(
                Update,
                systems::ui::boss_intro_update_system.run_if(in_state(AppState::BossIntro)),
            )
            // -------------------------------------------------------------
            // 6. 보스 격파 승리 씬 (Victory Scene)
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::Victory), systems::ui::setup_victory_scene)
            .add_systems(OnExit(AppState::Victory), systems::ui::cleanup_victory_scene)
            .add_systems(
                Update,
                systems::ui::victory_input_system.run_if(in_state(AppState::Victory)),
            )
            // -------------------------------------------------------------
            // 7. 레벨업 모달 씬 (LevelUp Scene)
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::LevelUp), systems::ui::setup_levelup_ui)
            .add_systems(OnExit(AppState::LevelUp), systems::ui::cleanup_levelup_ui)
            .add_systems(
                Update,
                systems::ui::levelup_input_system.run_if(in_state(AppState::LevelUp)),
            )
            // -------------------------------------------------------------
            // 8. 게임 오버 씬 (GameOver Scene)
            // -------------------------------------------------------------
            .add_systems(OnEnter(AppState::GameOver), systems::ui::setup_gameover_ui)
            .add_systems(OnExit(AppState::GameOver), systems::ui::cleanup_gameover_ui)
            .add_systems(
                Update,
                systems::ui::gameover_input_system.run_if(in_state(AppState::GameOver)),
            )
            // -------------------------------------------------------------
            // 인게임 본 전투 시뮬레이션 시스템 (Playing 상태)
            // -------------------------------------------------------------
            .add_systems(
                Update,
                (
                    systems::ui::pause_toggle_system,
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
            // 전역 카메라 시스템
            // -------------------------------------------------------------
            .add_systems(Update, systems::shake::camera_shake_system);
    }
}
