use bevy::prelude::*;
use crate::components::{Boundary, Paddle};

/// 패들 키보드 조작 및 화면 경계 클램핑 시스템
pub fn paddle_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Boundary, &mut Transform), With<Paddle>>,
) {
    let mut direction = 0.0;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    if direction != 0.0 {
        let speed = 550.0;
        for (boundary, mut transform) in query.iter_mut() {
            transform.translation.x += direction * speed * time.delta_secs();
            boundary.clamp(&mut transform.translation);
        }
    }
}

