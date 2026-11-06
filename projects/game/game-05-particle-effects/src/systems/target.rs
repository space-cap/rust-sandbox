use bevy::prelude::*;
use crate::components::{
    CameraShake, CannonBall, Collider2d, HitstopManager, JuiceStats, Particle, SoundAssets,
    TargetDummy, Velocity,
};
use rand::Rng;
use std::f32::consts::PI;

/// 탄환과 타깃 더미 충돌 감지, 체력 차감, 폭발 파티클 버스트 및 히트스톱 트리거
pub fn target_collision_system(
    mut commands: Commands,
    sounds: Res<SoundAssets>,
    mut stats: ResMut<JuiceStats>,
    mut shake: ResMut<CameraShake>,
    mut hitstop: ResMut<HitstopManager>,
    ball_query: Query<(Entity, &Transform, &Collider2d), With<CannonBall>>,
    mut target_query: Query<(Entity, &Transform, &Collider2d, &mut TargetDummy, &mut Sprite)>,
) {
    let mut rng = rand::thread_rng();

    for (ball_e, ball_tf, ball_col) in ball_query.iter() {
        let ball_pos = ball_tf.translation.truncate();

        for (target_e, target_tf, target_col, mut target, mut sprite) in target_query.iter_mut() {
            let target_pos = target_tf.translation.truncate();

            if ball_col.intersects(ball_pos, target_col, target_pos) {
                // 탄환 소멸
                commands.entity(ball_e).despawn();

                target.health -= 1;

                if target.health <= 0 {
                    // ==========================================
                    // 1. 대폭발 파티클 버스트 (45개 방사형 파편 비산)
                    // ==========================================
                    commands.entity(target_e).despawn();
                    stats.targets_destroyed += 1;

                    for _ in 0..45 {
                        let angle = rng.gen_range(0.0..(2.0 * PI));
                        let speed = rng.gen_range(120.0..520.0);
                        let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                        let lifetime_sec = rng.gen_range(0.35..0.75);
                        let size = rng.gen_range(4.0..10.0);

                        // 폭발 컬러: 골드, 오렌지, 크림슨 레드, 퓨어 화이트
                        let color = match rng.gen_range(0..4) {
                            0 => Color::srgb(1.0, 0.9, 0.3), // 골드 옐로우
                            1 => Color::srgb(1.0, 0.4, 0.1), // 화염 오렌지
                            2 => Color::srgb(1.0, 0.15, 0.15), // 크림슨 레드
                            _ => Color::srgb(1.0, 1.0, 1.0), // 화이트 코어
                        };

                        commands.spawn((
                            Particle {
                                lifetime: Timer::from_seconds(lifetime_sec, TimerMode::Once),
                                drag: 0.91,
                                base_color: color,
                                initial_size: size,
                            },
                            Velocity(vel),
                            Sprite::from_color(color, Vec2::splat(size)),
                            Transform::from_xyz(target_pos.x, target_pos.y, 2.0),
                        ));
                        stats.particles_spawned += 1;
                    }

                    // 2. 강력한 카메라 쉐이크 부여 (0.55 Trauma)
                    shake.add_trauma(0.55);

                    // 3. 0.06초 히트스톱(시간 정지) 연출 (묵직한 타격감 전달)
                    hitstop.timer = Some(Timer::from_seconds(0.06, TimerMode::Once));

                    // 4. 폭발 오디오 재생
                    commands.spawn((
                        AudioPlayer::new(sounds.explosion.clone()),
                        PlaybackSettings::DESPAWN,
                    ));
                } else {
                    // 피격 시 10개의 작은 불꽃 스파크 튐 연출
                    for _ in 0..10 {
                        let angle = rng.gen_range(0.0..(2.0 * PI));
                        let speed = rng.gen_range(80.0..240.0);
                        let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

                        commands.spawn((
                            Particle {
                                lifetime: Timer::from_seconds(0.25, TimerMode::Once),
                                drag: 0.93,
                                base_color: Color::srgb(1.0, 0.8, 0.2),
                                initial_size: 4.0,
                            },
                            Velocity(vel),
                            Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(4.0)),
                            Transform::from_xyz(ball_pos.x, ball_pos.y, 2.0),
                        ));
                        stats.particles_spawned += 1;
                    }

                    // 피격 시 색상 밝아지는 플래시 피드백
                    let hp_ratio = target.health as f32 / target.max_health as f32;
                    sprite.color = Color::srgb(1.0, 0.2 + hp_ratio * 0.4, 0.1);

                    // 가벼운 피격 쉐이크
                    shake.add_trauma(0.18);
                }

                break;
            }
        }
    }
}

/// 타깃이 모두 파괴되었을 때 자동으로 재생성하는 리스폰 시스템
pub fn target_respawn_system(
    mut commands: Commands,
    query: Query<&TargetDummy>,
) {
    if query.iter().count() == 0 {
        let mut rng = rand::thread_rng();
        let target_spacing = 160.0;
        for i in 0..5 {
            let x = (i as f32 - 2.0) * target_spacing + rng.gen_range(-20.0..20.0);
            commands.spawn((
                TargetDummy {
                    health: 3,
                    max_health: 3,
                },
                Collider2d::new(50.0, 50.0),
                Sprite::from_color(
                    Color::srgb(1.0, 0.45, 0.2),
                    Vec2::new(50.0, 50.0),
                ),
                Transform::from_xyz(x, 110.0, 1.0),
            ));
        }
    }
}

