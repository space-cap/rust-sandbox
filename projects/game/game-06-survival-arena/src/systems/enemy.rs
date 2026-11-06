use bevy::prelude::*;
use crate::components::{
    AppState, CameraShake, Collider2d, Enemy, EnemySpawnTimer, ExpGem, Particle, Player,
    PlayerStats, SoundAssets, SurvivalStats, Velocity,
};
use rand::Rng;
use std::f32::consts::PI;

/// 시간에 따른 주기적 적 군집 스폰 시스템 (생존 시간이 길어질수록 스폰 가속)
pub fn enemy_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    survival_stats: Res<SurvivalStats>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
) {
    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.is_finished() {
        let mut rng = rand::thread_rng();

        // 아레나 4개 외곽 변 중 하나에서 스폰
        let side = rng.gen_range(0..4);
        let (spawn_x, spawn_y) = match side {
            0 => (rng.gen_range(-460.0..460.0), 300.0),  // 상단
            1 => (rng.gen_range(-460.0..460.0), -300.0), // 하단
            2 => (-480.0, rng.gen_range(-280.0..280.0)), // 좌측
            _ => (480.0, rng.gen_range(-280.0..280.0)),  // 우측
        };

        // 생존 시간에 따른 점진적 체력 상승
        let time_bonus = (survival_stats.survival_time * 0.08).min(40.0);
        let base_hp = 30.0 + time_bonus;

        commands.spawn((
            Enemy {
                health: base_hp,
                max_health: base_hp,
                speed: rng.gen_range(90.0..130.0),
                damage: 15.0,
                exp_value: 5.0,
            },
            Collider2d::new(26.0, 26.0),
            Sprite::from_color(
                Color::srgb(0.85, 0.15, 0.4), // 크림슨 퍼플 버그 몬스터
                Vec2::new(24.0, 24.0),
            ),
            Transform::from_xyz(spawn_x, spawn_y, 1.8),
        ));

        // 난이도에 따른 스폰 간격 조정 (1.0초 -> 최저 0.35초)
        let new_interval = (1.0 - (survival_stats.survival_time * 0.005)).max(0.35);
        spawn_timer.0.set_duration(std::time::Duration::from_secs_f32(new_interval));
        spawn_timer.0.reset();
    }
}

/// 플레이어를 향한 벡터 스티어링 AI 및 플레이어 충돌 시 피해 시스템
pub fn enemy_ai_and_player_damage_system(
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    mut commands: Commands,
    mut shake: ResMut<CameraShake>,
    mut next_state: ResMut<NextState<AppState>>,
    mut player_query: Query<(&Transform, &Collider2d, &mut PlayerStats), With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Collider2d, &Enemy), Without<Player>>,
) {
    let Ok((player_tf, player_col, mut player_stats)) = player_query.single_mut() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    for (mut enemy_tf, enemy_col, enemy) in enemy_query.iter_mut() {
        let enemy_pos = enemy_tf.translation.truncate();
        let to_player = player_pos - enemy_pos;

        if to_player.length_squared() > 1.0 {
            let dir = to_player.normalize();
            enemy_tf.translation.x += dir.x * enemy.speed * time.delta_secs();
            enemy_tf.translation.y += dir.y * enemy.speed * time.delta_secs();
        }

        // 플레이어 접촉 충돌 데미지
        if enemy_col.intersects(enemy_tf.translation.truncate(), player_col, player_pos) {
            let damage = enemy.damage * time.delta_secs();
            player_stats.health -= damage;
            shake.add_trauma(0.15 * time.delta_secs());

            if player_stats.health <= 0.0 {
                player_stats.health = 0.0;
                commands.spawn((
                    AudioPlayer::new(sounds.gameover.clone()),
                    PlaybackSettings::DESPAWN,
                ));
                next_state.set(AppState::GameOver);
                return;
            }
        }
    }
}

/// 적 사망 감지, 폭발 파티클 비산, 킬 수 집계 및 경험치 보석 드랍
pub fn enemy_death_system(
    mut commands: Commands,
    sounds: Res<SoundAssets>,
    mut stats: ResMut<SurvivalStats>,
    mut shake: ResMut<CameraShake>,
    query: Query<(Entity, &Transform, &Enemy)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, tf, enemy) in query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
            stats.kills += 1;

            let pos = tf.translation.truncate();

            // 1. 사망 폭발 파티클 12개 비산
            for _ in 0..12 {
                let angle = rng.gen_range(0.0..(2.0 * PI));
                let speed = rng.gen_range(100.0..320.0);
                let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

                commands.spawn((
                    Particle {
                        lifetime: Timer::from_seconds(0.35, TimerMode::Once),
                        drag: 0.90,
                        base_color: Color::srgb(1.0, 0.3, 0.5),
                    },
                    Velocity(vel),
                    Sprite::from_color(Color::srgb(1.0, 0.3, 0.5), Vec2::splat(5.0)),
                    Transform::from_xyz(pos.x, pos.y, 2.9),
                ));
            }

            // 2. 80% 확률로 경험치 보석(Gem) 드랍
            if rng.gen_bool(0.85) {
                commands.spawn((
                    ExpGem {
                        value: enemy.exp_value,
                        is_attracted: false,
                    },
                    Collider2d::new(14.0, 14.0),
                    Sprite::from_color(
                        Color::srgb(0.2, 1.0, 0.8), // 청록 에메랄드 보석
                        Vec2::new(10.0, 10.0),
                    ),
                    Transform::from_xyz(pos.x, pos.y, 1.5)
                        .with_rotation(Quat::from_rotation_z(PI * 0.25)), // 다이아몬드 각도
                ));
            }

            shake.add_trauma(0.18);
        }
    }

    let _ = sounds;
}
