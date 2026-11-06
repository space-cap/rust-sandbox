use bevy::prelude::*;
use crate::components::{Particle, Velocity};

/// 파티클 수명, 공기 저항, 위치 이동 및 투명도 페이드아웃 시스템
pub fn particle_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Velocity, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut velocity, mut transform, mut sprite) in query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.is_finished() {
            commands.entity(entity).despawn();
        } else {
            velocity.0 *= particle.drag;
            transform.translation.x += velocity.0.x * time.delta_secs();
            transform.translation.y += velocity.0.y * time.delta_secs();

            let progress = particle.lifetime.fraction();
            let alpha = (1.0 - progress).clamp(0.0, 1.0);
            let scale_factor = (1.0 - progress * 0.5).max(0.1);

            transform.scale = Vec3::splat(scale_factor);
            sprite.color = particle.base_color.with_alpha(alpha);
        }
    }
}

