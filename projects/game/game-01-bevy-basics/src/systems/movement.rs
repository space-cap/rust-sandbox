use bevy::prelude::*;
use crate::components::{Boundary, GameStats, Player, Speed};

/// 키보드 입력을 감지하여 플레이어의 Transform 위치를 갱신하는 ECS 시스템
pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut stats: ResMut<GameStats>,
    mut query: Query<(&Speed, &Boundary, &mut Transform), With<Player>>,
) {
    let mut direction = Vec2::ZERO;

    // 키보드 입력 감지 (WASD 및 방향키 동시 지원)
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    // 대각선 이동 시 속도 초과 방지 (정규화)
    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    for (speed, boundary, mut transform) in query.iter_mut() {
        if direction != Vec2::ZERO {
            let movement_delta = direction * speed.0 * time.delta_secs();
            let distance = movement_delta.length();
            stats.total_distance_moved += distance;

            // 좌표 이동 적용
            transform.translation.x += movement_delta.x;
            transform.translation.y += movement_delta.y;

            // 이동 방향에 맞춰 미세한 틸트(회전) 애니메이션 효과 (시각적 피드백)
            let target_rotation = -direction.x * 0.15;
            transform.rotation = Quat::from_rotation_z(target_rotation);
        } else {
            // 정지 상태 시 회전 복귀
            transform.rotation = Quat::IDENTITY;
        }

        // 화면 밖 이탈 방지 (경계 클램핑)
        boundary.clamp(&mut transform.translation);
    }
}

