use bevy::prelude::*;
use crate::components::{
    AppState, BossHpBarFill, BossHpBarText, BossManager, BossProjectile, BossTitan, Collider2d,
    Enemy, ExpGem, FloatingDamageText, GameOverUi, GameRecord, GhostTrail, HolyWaterPool,
    HomingArrow, HudDashText, HudExpText, HudHealthText, HudTimeKillsText, LevelUpUi, MenuUi,
    MonsterBestiary, OrbitBlade, ParabolicAxe, Particle, Player, PlayerStats, SurvivalStats,
    UpgradeChoices, UpgradeType,
};

/// 생존 시간 누적 시스템
pub fn survival_timer_system(time: Res<Time>, mut stats: ResMut<SurvivalStats>) {
    stats.survival_time += time.delta_secs();
}

/// 상단 HUD 및 보스 체력바 실시간 갱신 시스템
pub fn hud_and_boss_bar_system(
    stats: Res<SurvivalStats>,
    bestiary: Res<MonsterBestiary>,
    player_query: Query<&PlayerStats, With<Player>>,
    boss_query: Query<&Enemy, With<BossTitan>>,
    mut health_query: Query<&mut Text2d, (With<HudHealthText>, Without<HudExpText>, Without<HudDashText>, Without<HudTimeKillsText>, Without<BossHpBarText>)>,
    mut exp_query: Query<&mut Text2d, (With<HudExpText>, Without<HudHealthText>, Without<HudDashText>, Without<HudTimeKillsText>, Without<BossHpBarText>)>,
    mut dash_query: Query<&mut Text2d, (With<HudDashText>, Without<HudHealthText>, Without<HudExpText>, Without<HudTimeKillsText>, Without<BossHpBarText>)>,
    mut time_query: Query<&mut Text2d, (With<HudTimeKillsText>, Without<HudHealthText>, Without<HudExpText>, Without<HudDashText>, Without<BossHpBarText>)>,
    mut boss_bar_fill: Query<&mut Sprite, With<BossHpBarFill>>,
    mut boss_bar_text: Query<&mut Text2d, (With<BossHpBarText>, Without<HudHealthText>, Without<HudExpText>, Without<HudDashText>, Without<HudTimeKillsText>)>,
) {
    let Ok(player_stats) = player_query.single() else {
        return;
    };

    // 1. 체력 게이지
    let hp_ratio = (player_stats.health / player_stats.max_health).clamp(0.0, 1.0);
    let filled_hp = (hp_ratio * 10.0).round() as usize;
    let hp_bar = format!("[{}{}]", "#".repeat(filled_hp), "-".repeat(10 - filled_hp));
    for mut text in health_query.iter_mut() {
        text.0 = format!(
            "체력: {:.0}/{:.0} {}",
            player_stats.health, player_stats.max_health, hp_bar
        );
    }

    // 2. EXP 게이지
    let exp_ratio = (player_stats.exp / player_stats.next_level_exp).clamp(0.0, 1.0);
    let filled_exp = (exp_ratio * 10.0).round() as usize;
    let exp_bar = format!("[{}{}]", "#".repeat(filled_exp), "-".repeat(10 - filled_exp));
    for mut text in exp_query.iter_mut() {
        text.0 = format!(
            "Lv.{} EXP: {:.0}/{:.0} {}",
            player_stats.level, player_stats.exp, player_stats.next_level_exp, exp_bar
        );
    }

    // 3. 대시 쿨다운 표시
    for mut text in dash_query.iter_mut() {
        if player_stats.dash_cooldown.is_finished() {
            text.0 = "[Shift 대시: READY]".to_string();
        } else {
            let remain = player_stats.dash_cooldown.remaining_secs();
            text.0 = format!("[대시: {:.1}s]", remain);
        }
    }

    // 4. 생존 시간 및 도감 수집 수
    let total_secs = stats.survival_time as u32;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    for mut text in time_query.iter_mut() {
        text.0 = format!(
            "생존: {:02}:{:02} | 처치: {} (도감: {}/100종)",
            mins, secs, stats.kills, bestiary.discovered_names.len()
        );
    }

    // 5. 보스 체력바 게이지 갱신
    if let Ok(boss_enemy) = boss_query.single() {
        let boss_ratio = (boss_enemy.health / boss_enemy.max_health).clamp(0.0, 1.0);
        for mut sprite in boss_bar_fill.iter_mut() {
            sprite.custom_size = Some(Vec2::new(416.0 * boss_ratio, 16.0));
        }
        for mut text in boss_bar_text.iter_mut() {
            text.0 = format!(
                "⚠️ BOSS: 메모리 리크 타이탄 (HP: {:.0} / {:.0})",
                boss_enemy.health.max(0.0), boss_enemy.max_health
            );
        }
    }
}

/// 플로팅 데미지 텍스트 상승 이동 및 알파 페이드아웃 시스템
pub fn floating_damage_text_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FloatingDamageText, &mut TextColor)>,
) {
    for (entity, mut tf, mut float_txt, mut text_color) in query.iter_mut() {
        float_txt.lifetime.tick(time.delta());

        if float_txt.lifetime.is_finished() {
            commands.entity(entity).despawn();
        } else {
            tf.translation.x += float_txt.velocity.x * time.delta_secs();
            tf.translation.y += float_txt.velocity.y * time.delta_secs();
            float_txt.velocity.y -= 120.0 * time.delta_secs(); // 중력 완화

            let progress = float_txt.lifetime.fraction();
            let alpha = (1.0 - progress).clamp(0.0, 1.0);
            text_color.0 = text_color.0.with_alpha(alpha);
        }
    }
}

// =========================================================================
// 메인 메뉴
// =========================================================================

pub fn setup_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        MenuUi,
        Sprite::from_color(Color::srgba(0.02, 0.03, 0.08, 0.90), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    commands.spawn((
        MenuUi,
        Text2d::new(
            "🦀 FERRIS SURVIVORS ULTIMATE ⚔️\n\
            (100종 몬스터 변종 도감 & 타이탄 보스전)\n\n\
            [W, A, S, D] 이동 | [Shift] / [Space] 고속 대시 & 무적 프레임(i-frame)!\n\n\
            ⚔️ 4종 무기: 공전 칼날 + 유도 화살 + 성수 불꽃 장판 + 관통 회전 도끼\n\
            👾 100종의 다양한 변종 몬스터 (자폭/분열/신속/거대/보호막/원거리/황금)\n\
            💀 90초 생존 시 에픽 보스 '메모리 리크 타이탄' 강림!\n\n\
            [ Space ] 키를 눌러 전장으로 진입하세요!"
        ),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.95, 1.0)),
        Transform::from_xyz(0.0, 0.0, 11.0),
    ));
}

pub fn menu_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Playing);
    }
}

pub fn cleanup_menu_ui(mut commands: Commands, query: Query<Entity, With<MenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =========================================================================
// 레벨업 3지선다 카드
// =========================================================================

pub fn setup_levelup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    choices: Res<UpgradeChoices>,
) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        LevelUpUi,
        Sprite::from_color(Color::srgba(0.04, 0.05, 0.15, 0.88), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    let mut ui_text = "✨ LEVEL UP! 전설 스킬을 선택하세요 ✨\n\n".to_string();

    for (i, upgrade) in choices.choices.iter().enumerate() {
        ui_text.push_str(&format!(
            "[{}] {}\n    {}\n\n",
            i + 1,
            upgrade.title(),
            upgrade.desc()
        ));
    }

    ui_text.push_str("키보드 [1], [2], [3] 키를 눌러 선택하세요!");

    commands.spawn((
        LevelUpUi,
        Text2d::new(ui_text),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.9, 0.2)),
        Transform::from_xyz(0.0, 0.0, 11.0),
    ));
}

pub fn levelup_input_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    choices: Res<UpgradeChoices>,
    mut next_state: ResMut<NextState<AppState>>,
    mut player_query: Query<(&Transform, &mut PlayerStats), With<Player>>,
    blade_query: Query<Entity, With<OrbitBlade>>,
) {
    let selected_index = if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Numpad1) {
        Some(0)
    } else if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::Numpad2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Numpad3) {
        Some(2)
    } else {
        None
    };

    if let Some(idx) = selected_index {
        if idx < choices.choices.len() {
            let choice = choices.choices[idx];
            let Ok((player_tf, mut stats)) = player_query.single_mut() else {
                return;
            };

            match choice {
                UpgradeType::ExtraBlade => {
                    stats.orbit_count += 1;
                    stats.orbit_speed *= 1.2;

                    for e in blade_query.iter() {
                        commands.entity(e).despawn();
                    }
                    for i in 0..stats.orbit_count {
                        commands.spawn((
                            OrbitBlade {
                                index: i,
                                radius: 80.0,
                                angle: 0.0,
                                angular_speed: stats.orbit_speed,
                                damage: stats.orbit_damage,
                            },
                            Collider2d::new(18.0, 18.0),
                            Sprite::from_color(Color::srgb(0.2, 0.9, 1.0), Vec2::new(18.0, 18.0)),
                            Transform::from_xyz(player_tf.translation.x, player_tf.translation.y, 2.5),
                        ));
                    }
                }
                UpgradeType::ArrowCooldown => {
                    let cur = stats.arrow_cooldown.duration().as_secs_f32();
                    stats.arrow_cooldown.set_duration(std::time::Duration::from_secs_f32((cur * 0.65).max(0.18)));
                }
                UpgradeType::HolyWaterPool => {
                    let cur = stats.holy_water_cooldown.duration().as_secs_f32();
                    stats.holy_water_cooldown.set_duration(std::time::Duration::from_secs_f32((cur * 0.75).max(1.2)));
                    stats.holy_water_damage += 8.0;
                }
                UpgradeType::BouncingAxe => {
                    let cur = stats.axe_cooldown.duration().as_secs_f32();
                    stats.axe_cooldown.set_duration(std::time::Duration::from_secs_f32((cur * 0.75).max(0.8)));
                    stats.axe_damage += 20.0;
                }
                UpgradeType::HealAndMaxHp => {
                    stats.max_health += 25.0;
                    stats.health = (stats.health + 40.0).min(stats.max_health);
                }
                UpgradeType::MagnetRadius => {
                    stats.magnet_radius += 60.0;
                }
                UpgradeType::MoveSpeed => {
                    stats.speed *= 1.15;
                }
            }

            next_state.set(AppState::Playing);
        }
    }
}

pub fn cleanup_levelup_ui(mut commands: Commands, query: Query<Entity, With<LevelUpUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =========================================================================
// 게임 오버 & 100종 몬스터 도감 통계
// =========================================================================

pub fn setup_gameover_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<SurvivalStats>,
    bestiary: Res<MonsterBestiary>,
    mut record: ResMut<GameRecord>,
) {
    let font = asset_server.load("fonts/malgun.ttf");

    if stats.survival_time > record.best_survival_time_sec {
        record.best_survival_time_sec = stats.survival_time;
    }
    if stats.kills > record.max_kills {
        record.max_kills = stats.kills;
    }
    if bestiary.discovered_names.len() > record.discovered_monsters_count {
        record.discovered_monsters_count = bestiary.discovered_names.len();
    }
    let _ = record.save("assets/save/ultimate_records.json");

    commands.spawn((
        GameOverUi,
        Sprite::from_color(Color::srgba(0.12, 0.02, 0.02, 0.92), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    let mins = (stats.survival_time as u32) / 60;
    let secs = (stats.survival_time as u32) % 60;
    let best_mins = (record.best_survival_time_sec as u32) / 60;
    let best_secs = (record.best_survival_time_sec as u32) % 60;

    let text = format!(
        "💀 GAME OVER 💀\n\n\
        생존 시간: {:02}:{:02} | 처치 수: {}마리 | 보석: {}개\n\
        📖 이번 판 조우한 몬스터 도감: {} / 100종!\n\n\
        🏆 최고 생존 기록: {:02}:{:02} (최다 {}마리, 누적 도감: {}/100종)\n\n\
        [ R ] 키를 눌러 전장으로 다시 진입하세요!",
        mins, secs, stats.kills, stats.gems_collected,
        bestiary.discovered_names.len(),
        best_mins, best_secs, record.max_kills, record.discovered_monsters_count
    );

    commands.spawn((
        GameOverUi,
        Text2d::new(text),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(19.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.35, 0.35)),
        Transform::from_xyz(0.0, 0.0, 11.0),
    ));
}

pub fn gameover_input_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut stats: ResMut<SurvivalStats>,
    mut boss_manager: ResMut<BossManager>,
    mut player_query: Query<(&mut Transform, &mut PlayerStats), With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    blade_query: Query<Entity, With<OrbitBlade>>,
    arrow_query: Query<Entity, With<HomingArrow>>,
    pool_query: Query<Entity, With<HolyWaterPool>>,
    axe_query: Query<Entity, With<ParabolicAxe>>,
    gem_query: Query<Entity, With<ExpGem>>,
    particle_query: Query<Entity, With<Particle>>,
    float_query: Query<Entity, With<FloatingDamageText>>,
    boss_proj_query: Query<Entity, With<BossProjectile>>,
    trail_query: Query<Entity, With<GhostTrail>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        for e in enemy_query.iter() { commands.entity(e).despawn(); }
        for e in blade_query.iter() { commands.entity(e).despawn(); }
        for e in arrow_query.iter() { commands.entity(e).despawn(); }
        for e in pool_query.iter() { commands.entity(e).despawn(); }
        for e in axe_query.iter() { commands.entity(e).despawn(); }
        for e in gem_query.iter() { commands.entity(e).despawn(); }
        for e in particle_query.iter() { commands.entity(e).despawn(); }
        for e in float_query.iter() { commands.entity(e).despawn(); }
        for e in boss_proj_query.iter() { commands.entity(e).despawn(); }
        for e in trail_query.iter() { commands.entity(e).despawn(); }

        if let Ok((mut tf, mut p_stats)) = player_query.single_mut() {
            tf.translation = Vec3::new(0.0, 0.0, 2.0);
            *p_stats = PlayerStats::default();

            commands.spawn((
                OrbitBlade {
                    index: 0,
                    radius: 80.0,
                    angle: 0.0,
                    angular_speed: 2.8,
                    damage: 24.0,
                },
                Collider2d::new(18.0, 18.0),
                Sprite::from_color(Color::srgb(0.2, 0.9, 1.0), Vec2::new(18.0, 18.0)),
                Transform::from_xyz(80.0, 0.0, 2.5),
            ));
        }

        *stats = SurvivalStats::default();
        *boss_manager = BossManager::default();

        next_state.set(AppState::Playing);
    }
}

pub fn cleanup_gameover_ui(mut commands: Commands, query: Query<Entity, With<GameOverUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

