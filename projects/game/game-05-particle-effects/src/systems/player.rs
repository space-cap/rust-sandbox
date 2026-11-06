use bevy::prelude::*;
use crate::components::{
    Boundary, CameraShake, CannonBall, CannonCooldown, Collider2d, JuiceStats, PlayerCannon,
    SoundAssets, Velocity,
};

/// 플레이어 캐논 이동, 발사 및 반동(Recoil) 쉐이크 시스템
pub fn player_cannon_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    mut stats: ResMut<JuiceStats>,
    mut shake: ResMut<CameraShake>,
    mut query: Query<(&Boundary, &mut CannonCooldown, &mut Transform), With<PlayerCannon>>,
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

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    for (boundary, mut cooldown, mut transform) in query.iter_mut() {
        let speed = 460.0;
        let delta = direction * speed * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        boundary.clamp(&mut transform.translation);

        cooldown.0.tick(time.delta());

        // 스페이스바 발사
        if keyboard.pressed(KeyCode::Space) && cooldown.0.is_finished() {
            let px = transform.translation.x;
            let py = transform.translation.y + 24.0;

            commands.spawn((
                CannonBall,
                Velocity(Vec2::new(0.0, 780.0)),
                Collider2d::new(12.0, 22.0),
                Sprite::from_color(
                    Color::srgb(0.9, 1.0, 0.3), // 네온 옐로우그린
                    Vec2::new(10.0, 20.0),
                ),
                Transform::from_xyz(px, py, 1.2),
            ));

            // 발사 사운드 재생
            commands.spawn((
                AudioPlayer::new(sounds.laser.clone()),
                PlaybackSettings::DESPAWN,
            ));

            // 발사 시 미세한 반동(Recoil) 쉐이크 부여 (Juice!)
            shake.add_trauma(0.12);
            stats.total_shots += 1;
            cooldown.0.reset();
        }
    }
}

/// 캐논 발사체 이동 및 화면 상단 이탈 시 소멸
pub fn cannon_ball_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<CannonBall>>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        if transform.translation.y > 310.0 {
            commands.entity(entity).despawn();
        }
    }
}

