use bevy::prelude::*;
use crate::components::{
    AnimationIndices, AnimationState, Boundary, GameStats, Player, Speed,
};

/// 키보드 입력을 처리하여 플레이어 이동, 애니메이션 상태 전환 및 좌우 반전(flip_x) 수행
pub fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut stats: ResMut<GameStats>,
    mut query: Query<(
        &Speed,
        &Boundary,
        &mut Transform,
        &mut Sprite,
        &mut AnimationState,
        &mut AnimationIndices,
    ), With<Player>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    for (speed, boundary, mut transform, mut sprite, mut state, mut indices) in query.iter_mut() {
        if direction != Vec2::ZERO {
            let delta = direction * speed.0 * time.delta_secs();
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
            stats.total_distance += delta.length();

            // 왼쪽 이동 시 스프라이트 수평 반전 (flip_x), 오른쪽 이동 시 정방향
            if direction.x < -0.01 {
                sprite.flip_x = true;
            } else if direction.x > 0.01 {
                sprite.flip_x = false;
            }

            // 정지 -> 달리기(Run) 상태 전환
            if *state != AnimationState::Run {
                *state = AnimationState::Run;
                *indices = AnimationIndices::RUN;
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    atlas.index = AnimationIndices::RUN.first;
                }
            }
        } else {
            // 달리기 -> 대기(Idle) 상태 전환
            if *state != AnimationState::Idle {
                *state = AnimationState::Idle;
                *indices = AnimationIndices::IDLE;
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    atlas.index = AnimationIndices::IDLE.first;
                }
            }
        }

        boundary.clamp(&mut transform.translation);
    }
}

