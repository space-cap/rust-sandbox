pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::{CameraShake, DebugGizmoSettings, HitstopManager, JuiceStats};

/// 히트스톱(임팩트 프리즈) 중이 아닐 때만 true를 반환하는 런 조건
pub fn not_in_hitstop(hitstop: Res<HitstopManager>) -> bool {
    hitstop.timer.is_none()
}

/// Phase 5: 파티클 효과, 카메라 쉐이크, 디버그 기즈모 플러그인
pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraShake>()
            .init_resource::<HitstopManager>()
            .init_resource::<DebugGizmoSettings>()
            .init_resource::<JuiceStats>()
            .add_systems(Startup, systems::setup::setup_game)
            .add_systems(
                Update,
                (
                    systems::player::player_cannon_system,
                    systems::target::target_collision_system,
                    systems::target::target_respawn_system,
                    systems::player::cannon_ball_movement_system.run_if(not_in_hitstop),
                    systems::particle::particle_update_system.run_if(not_in_hitstop),
                    systems::shake::camera_shake_system,
                    systems::shake::hitstop_system,
                    systems::gizmos::gizmo_toggle_system,
                    systems::gizmos::debug_gizmo_render_system,
                    systems::hud::hud_update_system,
                ),
            );
    }
}

