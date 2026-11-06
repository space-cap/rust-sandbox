use bevy::prelude::*;
use crate::components::{CameraShake, DebugGizmoSettings, HudText, JuiceStats, Particle};

/// 실시간 게임 주스 및 통계 정보 HUD 텍스트 갱신 시스템
pub fn hud_update_system(
    stats: Res<JuiceStats>,
    shake: Res<CameraShake>,
    settings: Res<DebugGizmoSettings>,
    particle_query: Query<&Particle>,
    mut text_query: Query<&mut Text2d, With<HudText>>,
) {
    let particle_count = particle_query.iter().count();
    let gizmo_str = if settings.enabled { "ON" } else { "OFF" };

    for mut text in text_query.iter_mut() {
        text.0 = format!(
            "타깃 파괴: {}대 | 발사 수: {}발 | 활성 파티클: {}개 | 쉐이크 Trauma: {:.2} | 기즈모: [{}]",
            stats.targets_destroyed,
            stats.total_shots,
            particle_count,
            shake.trauma,
            gizmo_str
        );
    }
}

