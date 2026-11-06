use bevy::prelude::*;
use crate::components::{
    AnimationIndices, AnimationTimer, Player, PlayerFacing, PlayerStats,
};

/// 플레이어 이동 입력, 아레나 경계 클램핑, 페리스 스프라이트 애니메이션 및 좌우 반전
pub fn player_movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &PlayerStats,
        &mut Transform,
        &mut Sprite,
        &mut PlayerFacing,
        &mut AnimationIndices,
        &mut AnimationTimer,
    ), With<Player>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    let is_moving = direction != Vec2::ZERO;
    if is_moving {
        direction = direction.normalize();
    }

    for (stats, mut transform, mut sprite, mut facing, mut indices, mut anim_timer) in query.iter_mut() {
        // 1. 이동 및 경계 제한
        let delta = direction * stats.speed * time.delta_secs();
        transform.translation.x = (transform.translation.x + delta.x).clamp(-430.0, 430.0);
        transform.translation.y = (transform.translation.y + delta.y).clamp(-260.0, 260.0);

        // 2. 좌우 반전
        if direction.x < -0.01 {
            facing.flip_x = true;
            sprite.flip_x = true;
        } else if direction.x > 0.01 {
            facing.flip_x = false;
            sprite.flip_x = false;
        }

        // 3. 상태에 따른 애니메이션 인덱스 변경 (Idle: 0~3, Run: 4~7)
        if is_moving {
            indices.first = 4;
            indices.last = 7;
        } else {
            indices.first = 0;
            indices.last = 3;
        }

        // 4. 애니메이션 타이머 틱
        anim_timer.0.tick(time.delta());
        if anim_timer.0.is_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                if atlas.index < indices.first || atlas.index >= indices.last {
                    atlas.index = indices.first;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}

