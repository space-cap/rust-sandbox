use bevy::prelude::*;
use crate::components::{
    AppState, BossHpBarRoot, BossManager, BossProjectile, BossTitan, CameraShake, Collider2d,
    Enemy, EnemySpawnTimer, ExpGem, ImageAssets, MonsterAffix, MonsterArchetype, MonsterBestiary,
    MonsterElement, Particle, Player, PlayerStats, SoundAssets, SurvivalStats, Velocity,
};
use rand::Rng;
use std::f32::consts::PI;

/// 100종 몬스터 절차적 생성 및 주기적 군집 스폰 시스템
pub fn enemy_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    survival_stats: Res<SurvivalStats>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut bestiary: ResMut<MonsterBestiary>,
    images: Res<ImageAssets>,
) {
    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.is_finished() {
        let mut rng = rand::thread_rng();

        // 1. 아레나 외곽 4면 중 한 곳에서 스폰 좌표 산출
        let side = rng.gen_range(0..4);
        let (spawn_x, spawn_y) = match side {
            0 => (rng.gen_range(-460.0..460.0), 300.0),
            1 => (rng.gen_range(-460.0..460.0), -300.0),
            2 => (-480.0, rng.gen_range(-280.0..280.0)),
            _ => (480.0, rng.gen_range(-280.0..280.0)),
        };

        // 2. 10대 기본 종족 무작위 추첨
        let archetypes = MonsterArchetype::all();
        let archetype = archetypes[rng.gen_range(0..archetypes.len())];

        // 3. 5대 원소 속성 무작위 추첨 (70% 일반, 30% 특수 원소)
        let element = if rng.gen_bool(0.35) {
            let elements = MonsterElement::all();
            elements[rng.gen_range(1..elements.len())]
        } else {
            MonsterElement::Normal
        };

        // 4. 8대 특수 접두사 변종(Affix) 무작위 추첨 (25% 확률로 엘리트 변종 등장)
        let affix = if rng.gen_bool(0.28) {
            let affixes = MonsterAffix::all();
            affixes[rng.gen_range(1..affixes.len())]
        } else {
            MonsterAffix::None
        };

        // 5. 몬스터 고유 이름 조합 (예: "신속의 화염 슬라임", "거대한 심연의 골렘")
        let name = format!("{}{}{}", affix.prefix(), element.prefix(), archetype.base_name());
        bestiary.discovered_names.insert(name.clone());

        // 6. 스탯 계산
        let (base_hp, base_spd, base_dmg, base_size) = archetype.base_stats();
        let time_scale = (survival_stats.survival_time * 0.12).min(80.0);

        let hp_mult = match affix {
            MonsterAffix::Colossal => 2.8,
            MonsterAffix::Golden => 1.5,
            _ => 1.0,
        };
        let final_hp = (base_hp + time_scale) * hp_mult;

        let spd_mult = match affix {
            MonsterAffix::Swift => 1.65,
            MonsterAffix::Colossal => 0.75,
            _ => 1.0,
        };
        let final_spd = base_spd * spd_mult;

        let size_mult = match affix {
            MonsterAffix::Colossal => 1.6,
            _ => 1.0,
        };
        let final_size = base_size * size_mult;

        let shield_hits = if affix == MonsterAffix::Shielded { 3 } else { 0 };
        let shoot_timer = if affix == MonsterAffix::Shooter {
            Some(Timer::from_seconds(2.8, TimerMode::Repeating))
        } else {
            None
        };

        let color = if affix == MonsterAffix::Golden {
            Color::srgb(1.0, 0.85, 0.1) // 황금빛
        } else {
            element.color_tint()
        };

        // 종족에 맞는 제미나이 몬스터 이미지 할당
        let monster_image = match archetype {
            MonsterArchetype::Slime | MonsterArchetype::Spider | MonsterArchetype::Mimic => {
                images.enemy_slime.clone()
            }
            MonsterArchetype::Bat | MonsterArchetype::Ghost | MonsterArchetype::Dragon => {
                images.enemy_bat.clone()
            }
            MonsterArchetype::Golem | MonsterArchetype::Skeleton | MonsterArchetype::Goblin | MonsterArchetype::Warlock => {
                images.enemy_golem.clone()
            }
        };

        let sprite = if let Some(img) = monster_image {
            Sprite {
                image: img,
                color,
                custom_size: Some(final_size * 1.3),
                ..default()
            }
        } else {
            Sprite::from_color(color, final_size)
        };

        commands.spawn((
            Enemy {
                name,
                archetype,
                element,
                affix,
                health: final_hp,
                max_health: final_hp,
                speed: final_spd,
                damage: base_dmg,
                exp_value: if affix == MonsterAffix::Golden { 30.0 } else { 6.0 },
                shield_hits,
                shoot_timer,
                is_boss: false,
            },
            Collider2d::new(final_size.x, final_size.y),
            sprite,
            Transform::from_xyz(spawn_x, spawn_y, 1.8),
        ));

        // 점진적 스폰 간격 단축 (1.0초 -> 최저 0.28초)
        let interval = (1.0 - (survival_stats.survival_time * 0.007)).max(0.28);
        spawn_timer.0.set_duration(std::time::Duration::from_secs_f32(interval));
        spawn_timer.0.reset();
    }
}

/// 90초 타이탄 에픽 보스 스폰 & 상단 보스바 활성화 시스템
pub fn boss_spawn_system(
    mut commands: Commands,
    sounds: Res<SoundAssets>,
    images: Res<ImageAssets>,
    survival_stats: Res<SurvivalStats>,
    mut boss_manager: ResMut<BossManager>,
    mut shake: ResMut<CameraShake>,
    mut next_state: ResMut<NextState<AppState>>,
    mut bar_root_query: Query<&mut Transform, With<BossHpBarRoot>>,
) {
    if survival_stats.survival_time >= 90.0 && !boss_manager.spawned {
        boss_manager.spawned = true;
        next_state.set(AppState::BossIntro);

        let boss_sprite = if let Some(ref img) = images.boss_titan {
            Sprite {
                image: img.clone(),
                custom_size: Some(Vec2::new(110.0, 110.0)),
                ..default()
            }
        } else {
            Sprite::from_color(
                Color::srgb(0.8, 0.1, 0.2),
                Vec2::new(75.0, 75.0),
            )
        };

        // 보스 스폰
        let boss_entity = commands.spawn((
            BossTitan {
                radial_timer: Timer::from_seconds(4.2, TimerMode::Repeating),
                enraged: false,
            },
            Enemy {
                name: "⚠️ [BOSS] 메모리 리크 타이탄".to_string(),
                archetype: MonsterArchetype::Dragon,
                element: MonsterElement::Void,
                affix: MonsterAffix::Colossal,
                health: 1400.0,
                max_health: 1400.0,
                speed: 55.0,
                damage: 32.0,
                exp_value: 120.0,
                shield_hits: 0,
                shoot_timer: None,
                is_boss: true,
            },
            Collider2d::new(75.0, 75.0),
            boss_sprite,
            Transform::from_xyz(0.0, 230.0, 2.2),
        )).id();

        boss_manager.boss_entity = Some(boss_entity);

        // 상단 보스 체력바 가시화 (Y: 240.0 위치로 이동)
        for mut bar_tf in bar_root_query.iter_mut() {
            bar_tf.translation.y = 240.0;
        }

        // 보스 경보음 및 대지진 쉐이크
        commands.spawn((
            AudioPlayer::new(sounds.boss_spawn.clone()),
            PlaybackSettings::DESPAWN,
        ));
        shake.add_trauma(0.70);
    }
}

/// 보스 탄막 발사 및 적 이동/플레이어 피격 시스템
pub fn enemy_ai_and_boss_system(
    mut commands: Commands,
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    mut shake: ResMut<CameraShake>,
    mut next_state: ResMut<NextState<AppState>>,
    mut player_query: Query<(&Transform, &Collider2d, &mut PlayerStats), With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Collider2d, &Enemy), (Without<Player>, Without<BossTitan>)>,
    mut boss_query: Query<(&mut Transform, &Collider2d, &mut Enemy, &mut BossTitan), (With<BossTitan>, Without<Player>)>,
) {
    let Ok((player_tf, player_col, mut player_stats)) = player_query.single_mut() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    // 1. 일반 적 이동 및 플레이어 접촉 충돌
    for (mut enemy_tf, enemy_col, enemy) in enemy_query.iter_mut() {
        let enemy_pos = enemy_tf.translation.truncate();
        let to_player = player_pos - enemy_pos;

        if to_player.length_squared() > 1.0 {
            let dir = to_player.normalize();
            enemy_tf.translation.x += dir.x * enemy.speed * time.delta_secs();
            enemy_tf.translation.y += dir.y * enemy.speed * time.delta_secs();
        }

        if enemy_col.intersects(enemy_tf.translation.truncate(), player_col, player_pos) {
            // 대시 무적 프레임 중이면 데미지 0 무효화!
            if !player_stats.invulnerable {
                player_stats.health -= enemy.damage * time.delta_secs();
                shake.add_trauma(0.18 * time.delta_secs());

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

    // 2. 보스 이동 및 8방향 방사형 탄막 발사 패턴
    for (mut boss_tf, boss_col, boss_enemy, mut boss) in boss_query.iter_mut() {
        let boss_pos = boss_tf.translation.truncate();
        let to_player = player_pos - boss_pos;

        if to_player.length_squared() > 1.0 {
            let dir = to_player.normalize();
            boss_tf.translation.x += dir.x * boss_enemy.speed * time.delta_secs();
            boss_tf.translation.y += dir.y * boss_enemy.speed * time.delta_secs();
        }

        // 8방향 탄막 발사 타이머 틱
        boss.radial_timer.tick(time.delta());
        if boss.radial_timer.is_finished() {
            commands.spawn((
                AudioPlayer::new(sounds.boss_shoot.clone()),
                PlaybackSettings::DESPAWN,
            ));

            for i in 0..8 {
                let angle = (i as f32) * (PI * 0.25);
                let vel = Vec2::new(angle.cos() * 240.0, angle.sin() * 240.0);

                commands.spawn((
                    BossProjectile {
                        velocity: vel,
                        damage: 18.0,
                        lifetime: Timer::from_seconds(3.5, TimerMode::Once),
                    },
                    Collider2d::new(14.0, 14.0),
                    Sprite::from_color(
                        Color::srgb(1.0, 0.1, 0.4), // 붉은 흑마법 탄환
                        Vec2::splat(12.0),
                    ),
                    Transform::from_xyz(boss_pos.x, boss_pos.y, 2.5),
                ));
            }

            shake.add_trauma(0.22);
        }

        // 보스 접촉 충돌
        if boss_col.intersects(boss_pos, player_col, player_pos) && !player_stats.invulnerable {
            player_stats.health -= boss_enemy.damage * time.delta_secs() * 1.5;
            shake.add_trauma(0.35 * time.delta_secs());
            if player_stats.health <= 0.0 {
                player_stats.health = 0.0;
                next_state.set(AppState::GameOver);
                return;
            }
        }
    }
}

/// 보스 탄환 이동 및 플레이어 충돌 판정 시스템
pub fn boss_projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut player_query: Query<(&Transform, &Collider2d, &mut PlayerStats), With<Player>>,
    mut proj_query: Query<(Entity, &mut Transform, &mut BossProjectile, &Collider2d), Without<Player>>,
) {
    let Ok((player_tf, player_col, mut player_stats)) = player_query.single_mut() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    for (entity, mut tf, mut proj, col) in proj_query.iter_mut() {
        proj.lifetime.tick(time.delta());

        if proj.lifetime.is_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        tf.translation.x += proj.velocity.x * time.delta_secs();
        tf.translation.y += proj.velocity.y * time.delta_secs();
        let proj_pos = tf.translation.truncate();

        if col.intersects(proj_pos, player_col, player_pos) {
            commands.entity(entity).despawn();

            if !player_stats.invulnerable {
                player_stats.health -= proj.damage;
                shake.add_trauma(0.20);
            }
        }
    }
}

/// 적 사망 처리, 자폭/분열 Affix 기믹 및 에메랄드/루비 보석 드랍 시스템
pub fn enemy_death_and_affix_system(
    mut commands: Commands,
    mut stats: ResMut<SurvivalStats>,
    mut bestiary: ResMut<MonsterBestiary>,
    mut shake: ResMut<CameraShake>,
    mut boss_manager: ResMut<BossManager>,
    images: Res<ImageAssets>,
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<(Entity, &Transform, &Enemy), Without<BossHpBarRoot>>,
    mut bar_root_query: Query<&mut Transform, (With<BossHpBarRoot>, Without<Enemy>)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, tf, enemy) in query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
            stats.kills += 1;
            bestiary.total_kills += 1;

            let pos = tf.translation.truncate();

            // 1. 보스 사망 시
            if enemy.is_boss {
                boss_manager.defeated = true;
                bestiary.boss_kills += 1;
                next_state.set(AppState::Victory);

                // 보스바 화면 밖으로 숨김
                for mut bar_tf in bar_root_query.iter_mut() {
                    bar_tf.translation.y = 1000.0;
                }

                // 대폭발 80개 파티클
                for _ in 0..80 {
                    let angle = rng.gen_range(0.0..(2.0 * PI));
                    let speed = rng.gen_range(120.0..550.0);
                    let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

                    commands.spawn((
                        Particle {
                            lifetime: Timer::from_seconds(0.65, TimerMode::Once),
                            drag: 0.90,
                            base_color: Color::srgb(1.0, 0.2, 0.2),
                        },
                        Velocity(vel),
                        Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::splat(7.0)),
                        Transform::from_xyz(pos.x, pos.y, 3.5),
                    ));
                }

                // 루비 보석 8개 드랍
                for _ in 0..8 {
                    let offset = Vec2::new(rng.gen_range(-40.0..40.0), rng.gen_range(-40.0..40.0));
                    let gem_sprite = if let Some(ref ruby_img) = images.gem_ruby {
                        Sprite {
                            image: ruby_img.clone(),
                            custom_size: Some(Vec2::splat(18.0)),
                            ..default()
                        }
                    } else {
                        Sprite::from_color(Color::srgb(1.0, 0.1, 0.3), Vec2::splat(14.0))
                    };

                    commands.spawn((
                        ExpGem {
                            value: 35.0,
                            is_ruby: true,
                            is_attracted: false,
                        },
                        Collider2d::new(16.0, 16.0),
                        gem_sprite,
                        Transform::from_xyz(pos.x + offset.x, pos.y + offset.y, 1.6),
                    ));
                }

                shake.add_trauma(0.85);
                continue;
            }

            // 2. 자폭(Volatile) 기믹: 사망 시 주위 폭발
            if enemy.affix == MonsterAffix::Volatile {
                shake.add_trauma(0.35);
                for _ in 0..25 {
                    let angle = rng.gen_range(0.0..(2.0 * PI));
                    let speed = rng.gen_range(140.0..400.0);
                    let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

                    commands.spawn((
                        Particle {
                            lifetime: Timer::from_seconds(0.4, TimerMode::Once),
                            drag: 0.91,
                            base_color: Color::srgb(1.0, 0.4, 0.1),
                        },
                        Velocity(vel),
                        Sprite::from_color(Color::srgb(1.0, 0.4, 0.1), Vec2::splat(6.0)),
                        Transform::from_xyz(pos.x, pos.y, 3.0),
                    ));
                }
            }

            // 3. 분열(Splitting) 기믹: 2마리 미니 분열
            if enemy.affix == MonsterAffix::Splitting {
                let mini_img = match enemy.archetype {
                    MonsterArchetype::Slime | MonsterArchetype::Spider | MonsterArchetype::Mimic => {
                        images.enemy_slime.clone()
                    }
                    MonsterArchetype::Bat | MonsterArchetype::Ghost | MonsterArchetype::Dragon => {
                        images.enemy_bat.clone()
                    }
                    MonsterArchetype::Golem | MonsterArchetype::Skeleton | MonsterArchetype::Goblin | MonsterArchetype::Warlock => {
                        images.enemy_golem.clone()
                    }
                };

                for i in 0..2 {
                    let offset_x = if i == 0 { -18.0 } else { 18.0 };
                    let mini_sprite = if let Some(ref img) = mini_img {
                        Sprite {
                            image: img.clone(),
                            color: enemy.element.color_tint(),
                            custom_size: Some(Vec2::splat(22.0)),
                            ..default()
                        }
                    } else {
                        Sprite::from_color(Color::srgb(0.5, 0.9, 0.3), Vec2::splat(16.0))
                    };

                    commands.spawn((
                        Enemy {
                            name: format!("미니 {}", enemy.name),
                            archetype: enemy.archetype,
                            element: enemy.element,
                            affix: MonsterAffix::None,
                            health: 15.0,
                            max_health: 15.0,
                            speed: enemy.speed * 1.3,
                            damage: 8.0,
                            exp_value: 3.0,
                            shield_hits: 0,
                            shoot_timer: None,
                            is_boss: false,
                        },
                        Collider2d::new(18.0, 18.0),
                        mini_sprite,
                        Transform::from_xyz(pos.x + offset_x, pos.y, 1.8),
                    ));
                }
            }

            // 4. 일반 사망 파티클
            for _ in 0..12 {
                let angle = rng.gen_range(0.0..(2.0 * PI));
                let speed = rng.gen_range(90.0..300.0);
                let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

                commands.spawn((
                    Particle {
                        lifetime: Timer::from_seconds(0.35, TimerMode::Once),
                        drag: 0.90,
                        base_color: Color::srgb(0.9, 0.25, 0.4),
                    },
                    Velocity(vel),
                    Sprite::from_color(Color::srgb(0.9, 0.25, 0.4), Vec2::splat(5.0)),
                    Transform::from_xyz(pos.x, pos.y, 3.0),
                ));
            }

            // 5. 보석 드랍 (황금 몬스터는 무조건 루비!)
            if enemy.affix == MonsterAffix::Golden {
                let ruby_sprite = if let Some(ref ruby_img) = images.gem_ruby {
                    Sprite {
                        image: ruby_img.clone(),
                        custom_size: Some(Vec2::splat(18.0)),
                        ..default()
                    }
                } else {
                    Sprite::from_color(Color::srgb(1.0, 0.15, 0.35), Vec2::splat(14.0))
                };

                commands.spawn((
                    ExpGem {
                        value: 30.0,
                        is_ruby: true,
                        is_attracted: false,
                    },
                    Collider2d::new(16.0, 16.0),
                    ruby_sprite,
                    Transform::from_xyz(pos.x, pos.y, 1.5),
                ));
            } else if rng.gen_bool(0.85) {
                commands.spawn((
                    ExpGem {
                        value: enemy.exp_value,
                        is_ruby: false,
                        is_attracted: false,
                    },
                    Collider2d::new(12.0, 12.0),
                    Sprite::from_color(Color::srgb(0.2, 1.0, 0.8), Vec2::splat(10.0)),
                    Transform::from_xyz(pos.x, pos.y, 1.5),
                ));
            }

            shake.add_trauma(0.12);
        }
    }
}

