use bevy::prelude::*;
use crate::components::{
    CannonBall, Collider2d, DebugGizmoSettings, Particle, PlayerCannon, TargetDummy, Velocity,
};

/// [G] 키를 눌러 디버그 기즈모(충돌체 와이어프레임 & 속도 벡터) 토글
pub fn gizmo_toggle_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<DebugGizmoSettings>,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        settings.enabled = !settings.enabled;
    }
}

/// 디버그 기즈모 렌더링: 플레이어/타깃/탄환 AABB 박스 및 파티클 속도 벡터 시각화
pub fn debug_gizmo_render_system(
    mut gizmos: Gizmos,
    settings: Res<DebugGizmoSettings>,
    player_query: Query<(&Transform, &Collider2d), With<PlayerCannon>>,
    target_query: Query<(&Transform, &Collider2d), With<TargetDummy>>,
    ball_query: Query<(&Transform, &Collider2d), With<CannonBall>>,
    particle_query: Query<(&Transform, &Velocity), With<Particle>>,
) {
    if !settings.enabled {
        return;
    }

    // 1. 플레이어 충돌 박스 (민트 그린)
    for (tf, col) in player_query.iter() {
        let pos = tf.translation.truncate();
        gizmos.rect_2d(pos, col.size, Color::srgb(0.0, 1.0, 0.5));
    }

    // 2. 타깃 더미 충돌 박스 (네온 레드)
    for (tf, col) in target_query.iter() {
        let pos = tf.translation.truncate();
        gizmos.rect_2d(pos, col.size, Color::srgb(1.0, 0.2, 0.2));
    }

    // 3. 발사체 충돌 박스 (옐로우)
    for (tf, col) in ball_query.iter() {
        let pos = tf.translation.truncate();
        gizmos.rect_2d(pos, col.size, Color::srgb(1.0, 1.0, 0.0));
    }

    // 4. 파티클 속도 벡터 선 (사이어-블루 선)
    for (tf, vel) in particle_query.iter() {
        let start = tf.translation.truncate();
        let end = start + vel.0 * 0.04;
        gizmos.line_2d(start, end, Color::srgb(0.4, 0.7, 1.0));
    }
}

