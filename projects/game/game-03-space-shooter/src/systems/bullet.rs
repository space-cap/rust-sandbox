use bevy::prelude::*;
use crate::components::{LaserBullet, Velocity};

/// 레이저 총알 이동 및 화면 상단 이탈 시 자동 소멸(Despawn) 시스템
pub fn bullet_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<LaserBullet>>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        // 화면 밖(+310.0)으로 벗어나면 메모리 누수 방지를 위해 즉각 소멸
        if transform.translation.y > 310.0 {
            commands.entity(entity).despawn();
        }
    }
}

