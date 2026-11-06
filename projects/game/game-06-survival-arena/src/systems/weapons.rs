use bevy::prelude::*;
use crate::components::{
    CameraShake, Collider2d, Enemy, HomingArrow, OrbitBlade, Particle, Player, PlayerStats,
    SoundAssets, Velocity,
};
use rand::Rng;
use std::f32::consts::PI;

/// 궤도 마법 칼날 회전 이동 및 적 충돌 판정 시스템
pub fn orbit_blade_system(
    mut commands: Commands,
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    mut shake: ResMut<CameraShake>,
    player_query: Query<&Transform, With<Player>>,
    mut blade_query: Query<(&mut Transform, &mut OrbitBlade, &Collider2d), Without<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &Collider2d, &mut Enemy), (Without<Player>, Without<OrbitBlade>)>,
) {
    let Ok(player_tf) = player_query.single() else {
        return;
    };
    let center = player_tf.translation.truncate();
    let total_blades = blade_query.iter().count() as f32;

    let mut rng = rand::thread_rng();

    for (mut blade_tf, mut blade, blade_col) in blade_query.iter_mut() {
        blade.angle += blade.angular_speed * time.delta_secs();

        // 칼날 번호별 균등 각도 분배
        let phase = if total_blades > 0.0 {
            (blade.index as f32) * (2.0 * PI / total_blades)
        } else {
            0.0
        };

        let current_angle = blade.angle + phase;
        let blade_x = center.x + blade.radius * current_angle.cos();
        let blade_y = center.y + blade.radius * current_angle.sin();

        blade_tf.translation.x = blade_x;
        blade_tf.translation.y = blade_y;

        let blade_pos = Vec2::new(blade_x, blade_y);

        // 적 충돌 판정
        for (_enemy_e, enemy_tf, enemy_col, mut enemy) in enemy_query.iter_mut() {
            let enemy_pos = enemy_tf.translation.truncate();

            if blade_col.intersects(blade_pos, enemy_col, enemy_pos) {
                // 초당 데미지 스케일
                enemy.health -= blade.damage * time.delta_secs() * 3.5;

                // 스파크 파티클 1개 생성
                let vel = Vec2::new(rng.gen_range(-60.0..60.0), rng.gen_range(-60.0..60.0));
                commands.spawn((
                    Particle {
                        lifetime: Timer::from_seconds(0.18, TimerMode::Once),
                        drag: 0.90,
                        base_color: Color::srgb(0.3, 0.9, 1.0),
                    },
                    Velocity(vel),
                    Sprite::from_color(Color::srgb(0.3, 0.9, 1.0), Vec2::splat(4.0)),
                    Transform::from_xyz(enemy_pos.x, enemy_pos.y, 3.0),
                ));

                shake.add_trauma(0.04);
            }
        }
    }

    let _ = sounds;
}

/// 가장 가까운 적을 감지하여 자동 유도 번개 화살을 발사하는 시스템
pub fn homing_arrow_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    mut player_query: Query<(&Transform, &mut PlayerStats), With<Player>>,
    enemy_query: Query<(&Transform, &Enemy), Without<Player>>,
) {
    let Ok((player_tf, mut stats)) = player_query.single_mut() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    stats.arrow_cooldown.tick(time.delta());

    if stats.arrow_cooldown.is_finished() {
        // 사거리 내 가장 가까운 적 탐색
        let nearest = enemy_query.iter().min_by(|(tf_a, _), (tf_b, _)| {
            let dist_a = tf_a.translation.truncate().distance_squared(player_pos);
            let dist_b = tf_b.translation.truncate().distance_squared(player_pos);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some((enemy_tf, _)) = nearest {
            let enemy_pos = enemy_tf.translation.truncate();
            let to_enemy = enemy_pos - player_pos;

            if to_enemy.length() < 520.0 {
                let dir = to_enemy.normalize();

                commands.spawn((
                    HomingArrow {
                        target_pos: enemy_pos,
                        speed: 560.0,
                        damage: stats.arrow_damage,
                        lifetime: Timer::from_seconds(1.2, TimerMode::Once),
                    },
                    Velocity(dir * 560.0),
                    Collider2d::new(12.0, 12.0),
                    Sprite::from_color(
                        Color::srgb(1.0, 0.95, 0.2), // 번개 골드
                        Vec2::new(14.0, 6.0),
                    ),
                    Transform::from_xyz(player_pos.x, player_pos.y, 2.8)
                        .with_rotation(Quat::from_rotation_z(dir.y.atan2(dir.x))),
                ));

                commands.spawn((
                    AudioPlayer::new(sounds.arrow.clone()),
                    PlaybackSettings::DESPAWN,
                ));

                stats.arrow_cooldown.reset();
            }
        }
    }
}

/// 유도 화살 이동 및 적 명중 처리 시스템
pub fn homing_arrow_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    mut shake: ResMut<CameraShake>,
    mut arrow_query: Query<(Entity, &mut Transform, &mut HomingArrow, &Velocity, &Collider2d)>,
    mut enemy_query: Query<(Entity, &Transform, &Collider2d, &mut Enemy), Without<HomingArrow>>,
) {
    let mut rng = rand::thread_rng();

    for (arrow_e, mut arrow_tf, mut arrow, vel, arrow_col) in arrow_query.iter_mut() {
        arrow.lifetime.tick(time.delta());

        if arrow.lifetime.is_finished() {
            commands.entity(arrow_e).despawn();
            continue;
        }

        arrow_tf.translation.x += vel.0.x * time.delta_secs();
        arrow_tf.translation.y += vel.0.y * time.delta_secs();
        let arrow_pos = arrow_tf.translation.truncate();

        // 충돌 체크
        for (_enemy_e, enemy_tf, enemy_col, mut enemy) in enemy_query.iter_mut() {
            let enemy_pos = enemy_tf.translation.truncate();

            if arrow_col.intersects(arrow_pos, enemy_col, enemy_pos) {
                enemy.health -= arrow.damage;
                commands.entity(arrow_e).despawn();

                // 타격 파편 5개 비산
                for _ in 0..5 {
                    let angle = rng.gen_range(0.0..(2.0 * PI));
                    let speed = rng.gen_range(80.0..220.0);
                    let p_vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

                    commands.spawn((
                        Particle {
                            lifetime: Timer::from_seconds(0.2, TimerMode::Once),
                            drag: 0.91,
                            base_color: Color::srgb(1.0, 0.9, 0.3),
                        },
                        Velocity(p_vel),
                        Sprite::from_color(Color::srgb(1.0, 0.9, 0.3), Vec2::splat(4.0)),
                        Transform::from_xyz(enemy_pos.x, enemy_pos.y, 3.0),
                    ));
                }

                commands.spawn((
                    AudioPlayer::new(sounds.hit.clone()),
                    PlaybackSettings::DESPAWN,
                ));

                shake.add_trauma(0.12);
                break;
            }
        }
    }
}
