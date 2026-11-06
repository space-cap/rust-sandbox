use bevy::prelude::*;
use crate::components::{
    AnimationIndices, AnimationTimer, CameraShake, GhostTrail, Player, PlayerFacing,
    PlayerStats, SoundAssets,
};

/// 플레이어 이동 입력, 대시(Dash) 무적기 발동 및 잔상(Ghost Trail) 생성 시스템
pub fn player_movement_and_dash_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    sounds: Res<SoundAssets>,
    mut shake: ResMut<CameraShake>,
    mut query: Query<(
        &mut PlayerStats,
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

    for (mut stats, mut transform, mut sprite, mut facing, mut indices, mut anim_timer) in query.iter_mut() {
        // 1. 대시 쿨다운 및 대시 지속 타이머 갱신
        stats.dash_cooldown.tick(time.delta());
        stats.dash_timer.tick(time.delta());

        if stats.dash_timer.is_finished() {
            stats.is_dashing = false;
            stats.invulnerable = false;
        }

        // 2. 대시 발동 (Shift 또는 Space 키)
        let dash_requested = keyboard.just_pressed(KeyCode::ShiftLeft)
            || keyboard.just_pressed(KeyCode::ShiftRight)
            || keyboard.just_pressed(KeyCode::Space);

        if dash_requested && stats.dash_cooldown.is_finished() {
            stats.is_dashing = true;
            stats.invulnerable = true;
            stats.dash_timer.reset();
            stats.dash_cooldown.reset();

            // 이동 중이 아니면 마지막 바라보던 방향으로 대시
            stats.dash_dir = if is_moving {
                direction
            } else if facing.flip_x {
                Vec2::new(-1.0, 0.0)
            } else {
                Vec2::new(1.0, 0.0)
            };

            commands.spawn((
                AudioPlayer::new(sounds.dash.clone()),
                PlaybackSettings::DESPAWN,
            ));
            shake.add_trauma(0.12);
        }

        // 3. 실제 이동 속도 계산 (대시 중이면 3.2배 초고속 전진)
        let current_speed = if stats.is_dashing {
            stats.speed * 3.2
        } else {
            stats.speed
        };

        let move_dir = if stats.is_dashing {
            stats.dash_dir
        } else {
            direction
        };

        let delta = move_dir * current_speed * time.delta_secs();
        transform.translation.x = (transform.translation.x + delta.x).clamp(-430.0, 430.0);
        transform.translation.y = (transform.translation.y + delta.y).clamp(-250.0, 250.0);

        // 4. 대시 중일 때 푸른색 잔상(Ghost Trail) 생성
        if stats.is_dashing {
            commands.spawn((
                GhostTrail {
                    lifetime: Timer::from_seconds(0.22, TimerMode::Once),
                },
                Sprite::from_color(
                    Color::srgba(0.2, 0.8, 1.0, 0.65), // 푸른 잔상
                    Vec2::new(42.0, 42.0),
                ),
                Transform::from_xyz(transform.translation.x, transform.translation.y, 1.9),
            ));
        }

        // 5. 방향 전환 및 애니메이션
        if move_dir.x < -0.01 {
            facing.flip_x = true;
            sprite.flip_x = true;
        } else if move_dir.x > 0.01 {
            facing.flip_x = false;
            sprite.flip_x = false;
        }

        if is_moving || stats.is_dashing {
            indices.first = 4;
            indices.last = 7;
        } else {
            indices.first = 0;
            indices.last = 3;
        }

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

/// 대시 잔상 페이드아웃 및 자동 소멸 시스템
pub fn ghost_trail_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut GhostTrail, &mut Sprite)>,
) {
    for (entity, mut trail, mut sprite) in query.iter_mut() {
        trail.lifetime.tick(time.delta());

        if trail.lifetime.is_finished() {
            commands.entity(entity).despawn();
        } else {
            let progress = trail.lifetime.fraction();
            let alpha = ((1.0 - progress) * 0.65).clamp(0.0, 1.0);
            sprite.color = Color::srgba(0.2, 0.8, 1.0, alpha);
        }
    }
}

