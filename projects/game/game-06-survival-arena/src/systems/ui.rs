use bevy::prelude::*;
use crate::components::{
    AppState, Collider2d, Enemy, ExpGem, GameOverUi, GameRecord, HomingArrow,
    HudExpText, HudHealthText, HudTimeKillsText, LevelUpUi, MenuUi, OrbitBlade, Particle,
    Player, PlayerStats, SurvivalStats, UpgradeChoices, UpgradeType,
};

/// 게임 생존 시간 갱신 시스템
pub fn survival_timer_system(
    time: Res<Time>,
    mut stats: ResMut<SurvivalStats>,
) {
    stats.survival_time += time.delta_secs();
}

/// 상단 실시간 HUD (체력 바, EXP 바, 생존 시간 & 처치 수) 갱신 시스템
pub fn hud_update_system(
    stats: Res<SurvivalStats>,
    player_query: Query<&PlayerStats, With<Player>>,
    mut health_query: Query<&mut Text2d, (With<HudHealthText>, Without<HudExpText>, Without<HudTimeKillsText>)>,
    mut exp_query: Query<&mut Text2d, (With<HudExpText>, Without<HudHealthText>, Without<HudTimeKillsText>)>,
    mut time_query: Query<&mut Text2d, (With<HudTimeKillsText>, Without<HudHealthText>, Without<HudExpText>)>,
) {
    let Ok(player_stats) = player_query.single() else {
        return;
    };

    // 1. 체력 게이지 갱신 (10단계 바)
    let hp_ratio = (player_stats.health / player_stats.max_health).clamp(0.0, 1.0);
    let filled_hp = (hp_ratio * 10.0).round() as usize;
    let hp_bar = format!("[{}{}]", "#".repeat(filled_hp), "-".repeat(10 - filled_hp));
    for mut text in health_query.iter_mut() {
        text.0 = format!(
            "체력: {:.0}/{:.0} {}",
            player_stats.health, player_stats.max_health, hp_bar
        );
    }

    // 2. EXP 게이지 갱신 (10단계 바)
    let exp_ratio = (player_stats.exp / player_stats.next_level_exp).clamp(0.0, 1.0);
    let filled_exp = (exp_ratio * 10.0).round() as usize;
    let exp_bar = format!("[{}{}]", "#".repeat(filled_exp), "-".repeat(10 - filled_exp));
    for mut text in exp_query.iter_mut() {
        text.0 = format!(
            "Lv.{} EXP: {:.0}/{:.0} {}",
            player_stats.level, player_stats.exp, player_stats.next_level_exp, exp_bar
        );
    }

    // 3. 생존 시간 및 처치 수 갱신
    let total_secs = stats.survival_time as u32;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    for mut text in time_query.iter_mut() {
        text.0 = format!("생존: {:02}:{:02} | 처치: {}마리", mins, secs, stats.kills);
    }
}

// =========================================================================
// 시작 메뉴 (Menu State)
// =========================================================================

pub fn setup_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/malgun.ttf");

    // 반투명 오버레이
    commands.spawn((
        MenuUi,
        Sprite::from_color(Color::srgba(0.02, 0.03, 0.08, 0.88), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    // 타이틀
    commands.spawn((
        MenuUi,
        Text2d::new("🦀 Rust Game 06: Ferris Survivors (2D Arena) ⚔️\n\n[W, A, S, D] 또는 [방향키]: 이동 (공격은 자동 발사!)\n\n회전 칼날과 유도 번개 화살로 언데드 군집을 소탕하세요!\n\n[ Space ] 키를 눌러 전투 시작!"),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(20.0),
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
// 레벨업 3지선다 카드 선택 (LevelUp State)
// =========================================================================

pub fn setup_levelup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    choices: Res<UpgradeChoices>,
) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        LevelUpUi,
        Sprite::from_color(Color::srgba(0.05, 0.05, 0.15, 0.85), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    let mut ui_text = "✨ LEVEL UP! 스킬 카드를 선택하세요 ✨\n\n".to_string();

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

                    // 기존 칼날 제거 후 균등 배치로 재스폰
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
                            Sprite::from_color(
                                Color::srgb(0.2, 0.9, 1.0),
                                Vec2::new(18.0, 18.0),
                            ),
                            Transform::from_xyz(player_tf.translation.x, player_tf.translation.y, 2.5),
                        ));
                    }
                }
                UpgradeType::ArrowCooldown => {
                    let cur = stats.arrow_cooldown.duration().as_secs_f32();
                    stats.arrow_cooldown.set_duration(std::time::Duration::from_secs_f32((cur * 0.70).max(0.2)));
                }
                UpgradeType::HealAndMaxHp => {
                    stats.max_health += 20.0;
                    stats.health = (stats.health + 35.0).min(stats.max_health);
                }
                UpgradeType::MagnetRadius => {
                    stats.magnet_radius += 50.0;
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
// 게임 오버 (GameOver State)
// =========================================================================

pub fn setup_gameover_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<SurvivalStats>,
    mut record: ResMut<GameRecord>,
) {
    let font = asset_server.load("fonts/malgun.ttf");

    // 최고 기록 갱신 여부 확인 및 로컬 파일 영구 저장
    let is_new_record = stats.survival_time > record.best_survival_time_sec;
    if is_new_record {
        record.best_survival_time_sec = stats.survival_time;
    }
    if stats.kills > record.max_kills {
        record.max_kills = stats.kills;
    }
    let _ = record.save("assets/save/high_score.json");

    commands.spawn((
        GameOverUi,
        Sprite::from_color(Color::srgba(0.12, 0.02, 0.02, 0.90), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    let mins = (stats.survival_time as u32) / 60;
    let secs = (stats.survival_time as u32) % 60;
    let best_mins = (record.best_survival_time_sec as u32) / 60;
    let best_secs = (record.best_survival_time_sec as u32) % 60;

    let text = format!(
        "💀 GAME OVER 💀\n\n\
        생존 시간: {:02}:{:02} | 처치 수: {}마리 | 보석 수집: {}개\n\n\
        🏆 최고 기록: {:02}:{:02} (최대 {}마리 처치)\n\n\
        [ R ] 키를 눌러 다시 도전하기!",
        mins, secs, stats.kills, stats.gems_collected, best_mins, best_secs, record.max_kills
    );

    commands.spawn((
        GameOverUi,
        Text2d::new(text),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(20.0),
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
    mut player_query: Query<(&mut Transform, &mut PlayerStats), With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    blade_query: Query<Entity, With<OrbitBlade>>,
    arrow_query: Query<Entity, With<HomingArrow>>,
    gem_query: Query<Entity, With<ExpGem>>,
    particle_query: Query<Entity, With<Particle>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        // 기존 필드 엔티티 정리
        for e in enemy_query.iter() { commands.entity(e).despawn(); }
        for e in blade_query.iter() { commands.entity(e).despawn(); }
        for e in arrow_query.iter() { commands.entity(e).despawn(); }
        for e in gem_query.iter() { commands.entity(e).despawn(); }
        for e in particle_query.iter() { commands.entity(e).despawn(); }

        // 플레이어 상태 원상복구
        if let Ok((mut tf, mut p_stats)) = player_query.single_mut() {
            tf.translation = Vec3::new(0.0, 0.0, 2.0);
            *p_stats = PlayerStats::default();

            // 초기 칼날 재스폰
            commands.spawn((
                OrbitBlade {
                    index: 0,
                    radius: 80.0,
                    angle: 0.0,
                    angular_speed: 3.2,
                    damage: 22.0,
                },
                Collider2d::new(18.0, 18.0),
                Sprite::from_color(
                    Color::srgb(0.2, 0.9, 1.0),
                    Vec2::new(18.0, 18.0),
                ),
                Transform::from_xyz(80.0, 0.0, 2.5),
            ));
        }

        // 통계 리셋
        *stats = SurvivalStats::default();

        next_state.set(AppState::Playing);
    }
}

pub fn cleanup_gameover_ui(mut commands: Commands, query: Query<Entity, With<GameOverUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
