use bevy::prelude::*;
use crate::components::{
    CameraShake, Collider2d, Enemy, FloatingDamageText, HolyWaterPool, HomingArrow,
    ImageAssets, OrbitBlade, ParabolicAxe, Particle, Player, PlayerStats, SoundAssets, Velocity,
};
use rand::Rng;
use std::f32::consts::PI;

/// 궤도 마법 칼날 회전 및 적 지속 타격 시스템
pub fn orbit_blade_system(
    mut commands: Commands,
    time: Res<Time>,
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

        for (_enemy_e, enemy_tf, enemy_col, mut enemy) in enemy_query.iter_mut() {
            let enemy_pos = enemy_tf.translation.truncate();

            if blade_col.intersects(blade_pos, enemy_col, enemy_pos) {
                // 방어막 체크
                if enemy.shield_hits > 0 {
                    enemy.shield_hits -= 1;
                    continue;
                }

                let dmg = blade.damage * time.delta_secs() * 3.8;
                enemy.health -= dmg;

                // 스파크 파티클
                let p_vel = Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0));
                commands.spawn((
                    Particle {
                        lifetime: Timer::from_seconds(0.15, TimerMode::Once),
                        drag: 0.90,
                        base_color: Color::srgb(0.3, 0.9, 1.0),
                    },
                    Velocity(p_vel),
                    Sprite::from_color(Color::srgb(0.3, 0.9, 1.0), Vec2::splat(4.0)),
                    Transform::from_xyz(enemy_pos.x, enemy_pos.y, 3.0),
                ));

                shake.add_trauma(0.03);
            }
        }
    }
}

/// 최단 거리 적 자동 유도 번개 화살 발사 시스템
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
        let nearest = enemy_query.iter().min_by(|(tf_a, _), (tf_b, _)| {
            let dist_a = tf_a.translation.truncate().distance_squared(player_pos);
            let dist_b = tf_b.translation.truncate().distance_squared(player_pos);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some((enemy_tf, _)) = nearest {
            let enemy_pos = enemy_tf.translation.truncate();
            let to_enemy = enemy_pos - player_pos;

            if to_enemy.length() < 550.0 {
                let dir = to_enemy.normalize();

                commands.spawn((
                    HomingArrow {
                        target_pos: enemy_pos,
                        speed: 580.0,
                        damage: stats.arrow_damage,
                        lifetime: Timer::from_seconds(1.2, TimerMode::Once),
                    },
                    Velocity(dir * 580.0),
                    Collider2d::new(14.0, 14.0),
                    Sprite::from_color(
                        Color::srgb(1.0, 0.95, 0.2), // 골드 옐로우
                        Vec2::new(16.0, 6.0),
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

/// 유도 화살 이동, 적 명중 및 크리티컬/플로팅 데미지 텍스트 팝업
pub fn homing_arrow_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    mut shake: ResMut<CameraShake>,
    asset_server: Res<AssetServer>,
    mut arrow_query: Query<(Entity, &mut Transform, &mut HomingArrow, &Velocity, &Collider2d), Without<Enemy>>,
    mut enemy_query: Query<(Entity, &Transform, &Collider2d, &mut Enemy), Without<HomingArrow>>,
) {
    let font = asset_server.load("fonts/malgun.ttf");
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

        for (_enemy_e, enemy_tf, enemy_col, mut enemy) in enemy_query.iter_mut() {
            let enemy_pos = enemy_tf.translation.truncate();

            if arrow_col.intersects(arrow_pos, enemy_col, enemy_pos) {
                // 방어막 판정
                if enemy.shield_hits > 0 {
                    enemy.shield_hits -= 1;
                    commands.entity(arrow_e).despawn();
                    break;
                }

                // 15% 치명타(CRIT!) 판정
                let is_crit = rng.gen_bool(0.15);
                let actual_damage = if is_crit {
                    arrow.damage * 2.0
                } else {
                    arrow.damage
                };

                enemy.health -= actual_damage;
                commands.entity(arrow_e).despawn();

                // 플로팅 데미지 텍스트 스폰
                let float_color = if is_crit {
                    Color::srgb(1.0, 0.2, 0.2)
                } else {
                    Color::srgb(1.0, 0.95, 0.3)
                };
                let float_text = if is_crit {
                    format!("CRIT! {:.0}", actual_damage)
                } else {
                    format!("{:.0}", actual_damage)
                };

                commands.spawn((
                    FloatingDamageText {
                        velocity: Vec2::new(rng.gen_range(-20.0..20.0), 85.0),
                        lifetime: Timer::from_seconds(0.65, TimerMode::Once),
                        is_crit,
                    },
                    Text2d::new(float_text),
                    TextFont {
                        font: font.clone().into(),
                        font_size: bevy::text::FontSize::Px(if is_crit { 18.0 } else { 13.0 }),
                        ..default()
                    },
                    TextColor(float_color),
                    Transform::from_xyz(enemy_pos.x, enemy_pos.y + 12.0, 5.0),
                ));

                if is_crit {
                    commands.spawn((
                        AudioPlayer::new(sounds.crit_hit.clone()),
                        PlaybackSettings::DESPAWN,
                    ));
                    shake.add_trauma(0.25);
                } else {
                    commands.spawn((
                        AudioPlayer::new(sounds.hit.clone()),
                        PlaybackSettings::DESPAWN,
                    ));
                    shake.add_trauma(0.10);
                }
                break;
            }
        }
    }
}

/// 성수 화염 장판 (Holy Water Pool) 주기적 투척 및 지면 지속 피해 시스템
pub fn holy_water_spawner_and_damage_system(
    mut commands: Commands,
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    images: Res<ImageAssets>,
    mut player_query: Query<(&Transform, &mut PlayerStats), With<Player>>,
    mut pool_query: Query<(Entity, &Transform, &mut HolyWaterPool)>,
    mut enemy_query: Query<(&Transform, &mut Enemy), Without<Player>>,
) {
    let Ok((player_tf, mut stats)) = player_query.single_mut() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();
    let mut rng = rand::thread_rng();

    stats.holy_water_cooldown.tick(time.delta());

    // 1. 성수 투척 스폰
    if stats.holy_water_cooldown.is_finished() {
        // 적이 있으면 적 근처, 없으면 플레이어 주변에 투척
        let target_target = if let Some((enemy_tf, _)) = enemy_query.iter().next() {
            let offset = Vec2::new(rng.gen_range(-40.0..40.0), rng.gen_range(-40.0..40.0));
            enemy_tf.translation.truncate() + offset
        } else {
            player_pos + Vec2::new(rng.gen_range(-80.0..80.0), rng.gen_range(-80.0..80.0))
        };

        let pool_sprite = if let Some(ref hw_img) = images.holy_water {
            Sprite {
                image: hw_img.clone(),
                custom_size: Some(Vec2::new(140.0, 140.0)),
                ..default()
            }
        } else {
            Sprite::from_color(
                Color::srgba(0.2, 0.7, 1.0, 0.45), // 성수 푸른 오라 장판
                Vec2::new(130.0, 130.0),
            )
        };

        commands.spawn((
            HolyWaterPool {
                lifetime: Timer::from_seconds(4.0, TimerMode::Once),
                tick_timer: Timer::from_seconds(0.35, TimerMode::Repeating),
                damage: stats.holy_water_damage,
                radius: 65.0,
            },
            pool_sprite,
            Transform::from_xyz(target_target.x, target_target.y, 0.8),
        ));

        commands.spawn((
            AudioPlayer::new(sounds.holy_water.clone()),
            PlaybackSettings::DESPAWN,
        ));

        stats.holy_water_cooldown.reset();
    }

    // 2. 장판 틱 데미지 처리
    for (pool_e, pool_tf, mut pool) in pool_query.iter_mut() {
        pool.lifetime.tick(time.delta());

        if pool.lifetime.is_finished() {
            commands.entity(pool_e).despawn();
            continue;
        }

        pool.tick_timer.tick(time.delta());
        if pool.tick_timer.is_finished() {
            let pool_pos = pool_tf.translation.truncate();

            for (enemy_tf, mut enemy) in enemy_query.iter_mut() {
                let dist = enemy_tf.translation.truncate().distance(pool_pos);
                if dist <= pool.radius {
                    enemy.health -= pool.damage;

                    // 불꽃 파티클
                    commands.spawn((
                        Particle {
                            lifetime: Timer::from_seconds(0.2, TimerMode::Once),
                            drag: 0.92,
                            base_color: Color::srgb(0.4, 0.8, 1.0),
                        },
                        Velocity(Vec2::new(rng.gen_range(-30.0..30.0), rng.gen_range(20.0..60.0))),
                        Sprite::from_color(Color::srgb(0.4, 0.8, 1.0), Vec2::splat(4.0)),
                        Transform::from_xyz(enemy_tf.translation.x, enemy_tf.translation.y, 2.5),
                    ));
                }
            }
        }
    }
}

/// 포물선 관통 회전 도끼 (Parabolic Axe) 투척 및 비산 시스템
pub fn parabolic_axe_system(
    mut commands: Commands,
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    mut player_query: Query<(&Transform, &mut PlayerStats), With<Player>>,
    mut axe_query: Query<(Entity, &mut Transform, &mut ParabolicAxe, &Collider2d), Without<Player>>,
    mut enemy_query: Query<(&Transform, &Collider2d, &mut Enemy), (Without<Player>, Without<ParabolicAxe>)>,
) {
    let Ok((player_tf, mut stats)) = player_query.single_mut() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();
    let mut rng = rand::thread_rng();

    stats.axe_cooldown.tick(time.delta());

    // 1. 도끼 발사
    if stats.axe_cooldown.is_finished() {
        let launch_vx = rng.gen_range(-160.0..160.0);
        let launch_vy = rng.gen_range(380.0..460.0);

        commands.spawn((
            ParabolicAxe {
                velocity: Vec2::new(launch_vx, launch_vy),
                gravity: 750.0,
                damage: stats.axe_damage,
                lifetime: Timer::from_seconds(2.2, TimerMode::Once),
            },
            Collider2d::new(26.0, 26.0),
            Sprite::from_color(
                Color::srgb(1.0, 0.45, 0.1), // 화염 블레이드 도끼
                Vec2::new(24.0, 24.0),
            ),
            Transform::from_xyz(player_pos.x, player_pos.y + 10.0, 3.2),
        ));

        commands.spawn((
            AudioPlayer::new(sounds.axe.clone()),
            PlaybackSettings::DESPAWN,
        ));

        stats.axe_cooldown.reset();
    }

    // 2. 도끼 포물선 궤적 이동 및 관통 판정
    for (axe_e, mut axe_tf, mut axe, axe_col) in axe_query.iter_mut() {
        axe.lifetime.tick(time.delta());

        if axe.lifetime.is_finished() || axe_tf.translation.y < -320.0 {
            commands.entity(axe_e).despawn();
            continue;
        }

        axe.velocity.y -= axe.gravity * time.delta_secs();
        axe_tf.translation.x += axe.velocity.x * time.delta_secs();
        axe_tf.translation.y += axe.velocity.y * time.delta_secs();
        axe_tf.rotate_z(14.0 * time.delta_secs()); // 고속 회전

        let axe_pos = axe_tf.translation.truncate();

        for (enemy_tf, enemy_col, mut enemy) in enemy_query.iter_mut() {
            let enemy_pos = enemy_tf.translation.truncate();

            if axe_col.intersects(axe_pos, enemy_col, enemy_pos) {
                enemy.health -= axe.damage * time.delta_secs() * 4.0;
            }
        }
    }
}

