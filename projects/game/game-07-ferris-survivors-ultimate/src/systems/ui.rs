use bevy::prelude::*;
use crate::components::{
    AppState, AudioSettings, BestiaryBrowserState, BestiarySceneRoot, BossHpBarFill, BossHpBarRoot,
    BossHpBarText, BossIntroSceneRoot, BossIntroTimer, BossManager, BossProjectile, BossTitan,
    CameraShake, Collider2d, Enemy, ExpGem, FloatingDamageText, GameOverUi, GameRecord, GhostTrail,
    HolyWaterPool, HomingArrow, HudDashText, HudExpText, HudHealthText, HudTimeKillsText,
    ImageAssets, LevelUpUi, MonsterAffix, MonsterArchetype, MonsterBestiary, OrbitBlade,
    ParabolicAxe, Particle, PausedSceneRoot, Player, PlayerStats, SoundAssets, SurvivalStats,
    TitleLogoBanner, TitleSceneRoot, TutorialSceneRoot, UpgradeChoices, UpgradeType,
    VictorySceneRoot, WeaponDamageStats,
};
use rand::Rng;
use std::f32::consts::PI;

/// 생존 시간 누적 시스템
pub fn survival_timer_system(time: Res<Time>, mut stats: ResMut<SurvivalStats>) {
    stats.survival_time += time.delta_secs();
}

/// 인게임 중 ESC 누르면 일시정지 씬으로 전환
pub fn pause_toggle_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}

/// 상단 HUD 및 보스 체력바 실시간 갱신 시스템
pub fn hud_and_boss_bar_system(
    stats: Res<SurvivalStats>,
    bestiary: Res<MonsterBestiary>,
    player_query: Query<&PlayerStats, With<Player>>,
    boss_query: Query<&Enemy, With<BossTitan>>,
    mut boss_bar_fill: Query<&mut Sprite, With<BossHpBarFill>>,
    mut text_query: Query<(
        &mut Text2d,
        Option<&HudHealthText>,
        Option<&HudExpText>,
        Option<&HudDashText>,
        Option<&HudTimeKillsText>,
        Option<&BossHpBarText>,
    )>,
) {
    let Ok(player_stats) = player_query.single() else {
        return;
    };

    let hp_ratio = (player_stats.health / player_stats.max_health).clamp(0.0, 1.0);
    let filled_hp = (hp_ratio * 10.0).round() as usize;
    let hp_bar = format!("[{}{}]", "#".repeat(filled_hp), "-".repeat(10 - filled_hp));

    let exp_ratio = (player_stats.exp / player_stats.next_level_exp).clamp(0.0, 1.0);
    let filled_exp = (exp_ratio * 10.0).round() as usize;
    let exp_bar = format!("[{}{}]", "#".repeat(filled_exp), "-".repeat(10 - filled_exp));

    let dash_str = if player_stats.dash_cooldown.is_finished() {
        "[Shift 대시: READY]".to_string()
    } else {
        format!("[대시: {:.1}s]", player_stats.dash_cooldown.remaining_secs())
    };

    let total_secs = stats.survival_time as u32;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    let time_str = format!(
        "생존: {:02}:{:02} | 처치: {} (도감: {}/100종)",
        mins, secs, stats.kills, bestiary.discovered_names.len()
    );

    let boss_info = boss_query.single().ok().map(|b| {
        let ratio = (b.health / b.max_health).clamp(0.0, 1.0);
        let text = format!("⚠️ BOSS: 메모리 리크 타이탄 (HP: {:.0} / {:.0})", b.health.max(0.0), b.max_health);
        (ratio, text)
    });

    if let Some((ratio, _)) = boss_info.as_ref() {
        for mut sprite in boss_bar_fill.iter_mut() {
            sprite.custom_size = Some(Vec2::new(416.0 * ratio, 16.0));
        }
    }

    for (mut text, is_hp, is_exp, is_dash, is_time, is_boss) in text_query.iter_mut() {
        if is_hp.is_some() {
            text.0 = format!("체력: {:.0}/{:.0} {}", player_stats.health, player_stats.max_health, hp_bar);
        } else if is_exp.is_some() {
            text.0 = format!("Lv.{} EXP: {:.0}/{:.0} {}", player_stats.level, player_stats.exp, player_stats.next_level_exp, exp_bar);
        } else if is_dash.is_some() {
            text.0 = dash_str.clone();
        } else if is_time.is_some() {
            text.0 = time_str.clone();
        } else if is_boss.is_some() {
            if let Some((_, b_text)) = boss_info.as_ref() {
                text.0 = b_text.clone();
            }
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
            float_txt.velocity.y -= 120.0 * time.delta_secs();

            let progress = float_txt.lifetime.fraction();
            let alpha = (1.0 - progress).clamp(0.0, 1.0);
            text_color.0 = text_color.0.with_alpha(alpha);
        }
    }
}

// =========================================================================
// 씬 1: 메인 타이틀 씬 (Title Scene)
// =========================================================================

pub fn setup_title_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Option<Res<ImageAssets>>,
    record: Option<Res<GameRecord>>,
) {
    let font = asset_server.load("fonts/malgun.ttf");

    // 배경 어두운 네이비 오버레이
    commands.spawn((
        TitleSceneRoot,
        Sprite::from_color(Color::srgba(0.03, 0.04, 0.10, 0.94), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    // 타이틀 로고 배너 이미지
    if let Some(img_assets) = images {
        if let Some(ref banner) = img_assets.title_banner {
            commands.spawn((
                TitleSceneRoot,
                TitleLogoBanner,
                Sprite {
                    image: banner.clone(),
                    custom_size: Some(Vec2::new(560.0, 160.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 135.0, 10.5),
            ));
        }
    }

    // 메뉴 버튼 리스트
    let menu_text = "⚔️ [ 1 / Space ] 전장 진입 (Game Start)\n\n\
                     📖 [ 2 ] 100종 몬스터 도감 열람 (Monster Codex)\n\n\
                     📜 [ 3 ] 조작법 & 4대 무기 가이드 (How to Play)\n\n\
                     🚪 [ Q ] 게임 종료 (Quit)";

    commands.spawn((
        TitleSceneRoot,
        Text2d::new(menu_text),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(17.0),
            ..default()
        },
        TextColor(Color::srgb(0.92, 0.94, 1.0)),
        Transform::from_xyz(0.0, -45.0, 11.0),
    ));

    // 하단 최고 기록 뱃지
    let (best_mins, best_secs, max_kills, discovered_count) = if let Some(ref rec) = record {
        (
            (rec.best_survival_time_sec as u32) / 60,
            (rec.best_survival_time_sec as u32) % 60,
            rec.max_kills,
            rec.discovered_monsters_count,
        )
    } else {
        (0, 0, 0, 0)
    };
    let badge_text = format!(
        "🏆 최고 기록: {:02}:{:02} | 최다 처치: {}마리 | 누적 도감 등록: {}/100종",
        best_mins, best_secs, max_kills, discovered_count
    );

    commands.spawn((
        TitleSceneRoot,
        Text2d::new(badge_text),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(13.5),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Transform::from_xyz(0.0, -250.0, 11.0),
    ));
}

pub fn title_logo_animation_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<TitleLogoBanner>>,
) {
    for mut tf in query.iter_mut() {
        let offset = (time.elapsed_secs() * 2.2).sin() * 4.0;
        tf.translation.y = 135.0 + offset;
    }
}

pub fn title_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Digit1)
        || keyboard.just_pressed(KeyCode::Numpad1)
        || keyboard.just_pressed(KeyCode::Space)
    {
        next_state.set(AppState::Playing);
    } else if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::Numpad2) {
        next_state.set(AppState::Bestiary);
    } else if keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Numpad3) {
        next_state.set(AppState::Tutorial);
    } else if keyboard.just_pressed(KeyCode::KeyQ) {
        std::process::exit(0);
    }
}

pub fn cleanup_title_scene(mut commands: Commands, query: Query<Entity, With<TitleSceneRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =========================================================================
// 씬 2: 100종 몬스터 도감 씬 (Bestiary Codex Scene)
// =========================================================================

pub fn setup_bestiary_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Option<Res<ImageAssets>>,
    bestiary: Res<MonsterBestiary>,
    browser: Res<BestiaryBrowserState>,
) {
    render_bestiary_card(&mut commands, &asset_server, images.as_deref(), &bestiary, &browser);
}

pub fn render_bestiary_card(
    commands: &mut Commands,
    asset_server: &AssetServer,
    images: Option<&ImageAssets>,
    bestiary: &MonsterBestiary,
    browser: &BestiaryBrowserState,
) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        BestiarySceneRoot,
        Sprite::from_color(Color::srgba(0.02, 0.03, 0.09, 0.96), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    // 타이틀 헤더
    let total_unlocked = bestiary.discovered_names.len();
    let header_text = format!(
        "📖 100종 몬스터 도감 (Monster Bestiary Codex) - 해금: {} / 100종\n\
        ────────────────────────────────────────────────────────────",
        total_unlocked
    );

    commands.spawn((
        BestiarySceneRoot,
        Text2d::new(header_text),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(16.5),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.95, 1.0)),
        Transform::from_xyz(0.0, 235.0, 11.0),
    ));

    // 현재 선택된 몬스터 정보
    let (archetype, element, affix) = browser.entries[browser.current_index % browser.entries.len()];
    let monster_name = format!("{}{}{}", affix.prefix(), element.prefix(), archetype.base_name());
    let is_discovered = bestiary.discovered_names.contains(&monster_name);

    let (hp, spd, dmg, _) = archetype.base_stats();

    // 좌측: 몬스터 스프라이트 렌더링
    if is_discovered {
        let monster_img = if let Some(img_res) = images {
            match archetype {
                MonsterArchetype::Slime | MonsterArchetype::Spider | MonsterArchetype::Mimic => {
                    img_res.enemy_slime.clone()
                }
                MonsterArchetype::Bat | MonsterArchetype::Ghost | MonsterArchetype::Dragon => {
                    img_res.enemy_bat.clone()
                }
                MonsterArchetype::Golem
                | MonsterArchetype::Skeleton
                | MonsterArchetype::Goblin
                | MonsterArchetype::Warlock => img_res.enemy_golem.clone(),
            }
        } else {
            None
        };

        let tint = if affix == MonsterAffix::Golden {
            Color::srgb(1.0, 0.85, 0.1)
        } else {
            element.color_tint()
        };

        if let Some(img) = monster_img {
            commands.spawn((
                BestiarySceneRoot,
                Sprite {
                    image: img,
                    color: tint,
                    custom_size: Some(Vec2::new(130.0, 130.0)),
                    ..default()
                },
                Transform::from_xyz(-190.0, 25.0, 11.0),
            ));
        } else {
            commands.spawn((
                BestiarySceneRoot,
                Sprite::from_color(tint, Vec2::new(100.0, 100.0)),
                Transform::from_xyz(-190.0, 25.0, 11.0),
            ));
        }
    } else {
        // 미발견 몬스터: 물음표 실루엣 박스
        commands.spawn((
            BestiarySceneRoot,
            Sprite::from_color(Color::srgba(0.15, 0.18, 0.25, 0.8), Vec2::new(120.0, 120.0)),
            Transform::from_xyz(-190.0, 25.0, 11.0),
        ));
        commands.spawn((
            BestiarySceneRoot,
            Text2d::new("???"),
            TextFont {
                font: font.clone().into(),
                font_size: bevy::text::FontSize::Px(32.0),
                ..default()
            },
            TextColor(Color::srgb(0.5, 0.55, 0.65)),
            Transform::from_xyz(-190.0, 25.0, 11.5),
        ));
    }

    // 우측: 몬스터 상세 스펙 카드
    let card_text = if is_discovered {
        format!(
            "이름: {}\n\n\
            종족: {} | 속성: {}\n\
            어픽스: {}\n\n\
            기본 체력: {:.0} | 이동 속도: {:.0} | 공격력: {:.0}\n\n\
            상태: ✅ 조우 및 분석 완료! (도감 등록 완료)",
            monster_name,
            archetype.base_name(),
            element.prefix().trim(),
            if affix.prefix().is_empty() { "일반" } else { affix.prefix().trim() },
            hp, spd, dmg
        )
    } else {
        format!(
            "이름: ??? (미지의 변종)\n\n\
            종족: {} 계열 변종\n\n\
            상태: 🔒 미발견 (아직 전장에서 조우하지 못했습니다)\n\n\
            전장에서 해당 변종 몬스터를 조우하여 처치하면\n\
            고유 스프라이트와 속성 스펙이 도감에 해금됩니다!",
            archetype.base_name()
        )
    };

    commands.spawn((
        BestiarySceneRoot,
        Text2d::new(card_text),
        TextFont {
            font: font.clone().into(),
            font_size: bevy::text::FontSize::Px(16.0),
            ..default()
        },
        TextColor(if is_discovered {
            Color::srgb(0.95, 0.95, 1.0)
        } else {
            Color::srgb(0.65, 0.65, 0.75)
        }),
        Transform::from_xyz(100.0, 25.0, 11.0),
    ));

    // 하단 조작 가이드
    let nav_text = format!(
        "[ ← / A ] 이전 몬스터  |  [ → / D ] 다음 몬스터 (No. {} / {})  |  [ ESC / Space ] 타이틀 복귀",
        browser.current_index + 1,
        browser.entries.len()
    );

    commands.spawn((
        BestiarySceneRoot,
        Text2d::new(nav_text),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(14.0),
            ..default()
        },
        TextColor(Color::srgb(0.3, 0.9, 1.0)),
        Transform::from_xyz(0.0, -230.0, 11.0),
    ));
}

pub fn bestiary_input_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut browser: ResMut<BestiaryBrowserState>,
    query: Query<Entity, With<BestiarySceneRoot>>,
    asset_server: Res<AssetServer>,
    images: Option<Res<ImageAssets>>,
    bestiary: Res<MonsterBestiary>,
) {
    let mut changed = false;

    if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft) {
        if browser.current_index == 0 {
            browser.current_index = browser.entries.len() - 1;
        } else {
            browser.current_index -= 1;
        }
        changed = true;
    } else if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight) {
        browser.current_index = (browser.current_index + 1) % browser.entries.len();
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Escape)
        || keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyB)
    {
        next_state.set(AppState::Title);
        return;
    }

    if changed {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        render_bestiary_card(
            &mut commands,
            &asset_server,
            images.as_deref(),
            &bestiary,
            &browser,
        );
    }
}

pub fn cleanup_bestiary_scene(mut commands: Commands, query: Query<Entity, With<BestiarySceneRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =========================================================================
// 씬 3: 조작법 & 튜토리얼 씬 (Tutorial Scene)
// =========================================================================

pub fn setup_tutorial_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        TutorialSceneRoot,
        Sprite::from_color(Color::srgba(0.02, 0.03, 0.09, 0.95), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    let guide_text = "📜 FERRIS SURVIVORS: 조작법 & 4대 무기 가이드 📜\n\
                     ────────────────────────────────────────────────────────────\n\n\
                     🎮 [기본 조작]\n\
                     • [W, A, S, D] 이동  |  [Shift] / [Space] 고속 대시 (0.2초간 완전 무적 회피!)\n\
                     • [ESC] 언제든지 인게임 일시정지 (Pause) 메뉴 오픈\n\n\
                     ⚔️ [4대 자동 무기 특성]\n\
                     1. 공전 마법 칼날 (Orbit Blade)  : 페리스 주변을 고속 회전하며 근접 몬스터 지속 절단\n\
                     2. 유도 번개 화살 (Homing Arrow) : 최단거리 적 자동 추적, 15% 치명타(CRIT!) 폭딜\n\
                     3. 성수 불꽃 장판 (Holy Water)  : 적 무리에 투척되어 4초간 신성한 틱 지속 데미지\n\
                     4. 관통 회전 도끼 (Parabolic Axe): 높은 포물선 궤적으로 회전 낙하하며 경로 전체 관통\n\n\
                     👾 [100종 몬스터 변종 기믹]\n\
                     • 신속(+75% 이속), 거대(HP 3배), 자폭(사망 폭발), 분열(2마리 소형 분열)\n\
                     • 보호막(첫 3회 피격 무효화), 원거리 탄환, 황금(처치 시 대량 루비 보석 드랍)\n\n\
                     💀 [90초 타이탄 보스 레이드]\n\
                     • 생존 90초 도달 시 대지진과 함께 '메모리 리크 타이탄' 강림 및 8방향 방사형 탄막 발사!\n\n\
                     [ ESC ] 또는 [ Space ] 키를 눌러 타이틀로 복귀하세요!";

    commands.spawn((
        TutorialSceneRoot,
        Text2d::new(guide_text),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        Transform::from_xyz(0.0, 0.0, 11.0),
    ));
}

pub fn tutorial_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Title);
    }
}

pub fn cleanup_tutorial_scene(mut commands: Commands, query: Query<Entity, With<TutorialSceneRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =========================================================================
// 씬 4: 인게임 일시정지 씬 (Paused Scene)
// =========================================================================

pub fn setup_paused_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<SurvivalStats>,
    audio: Res<AudioSettings>,
) {
    let font = asset_server.load("fonts/malgun.ttf");

    commands.spawn((
        PausedSceneRoot,
        Sprite::from_color(Color::srgba(0.02, 0.03, 0.07, 0.86), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 15.0),
    ));

    let sfx_str = if audio.sfx_enabled { "ON" } else { "OFF" };
    let mins = (stats.survival_time as u32) / 60;
    let secs = (stats.survival_time as u32) % 60;

    let text = format!(
        "⏸️ GAME PAUSED (일시정지)\n\n\
        생존 시간: {:02}:{:02}  |  처치 수: {}마리\n\n\
        [ 1 / ESC ] 전장으로 복귀 (Resume)\n\n\
        [ 2 / M ] 효과음(SFX): [{}]\n\n\
        [ Q ] 포기하고 메인 타이틀로 나가기 (Return to Title)",
        mins, secs, stats.kills, sfx_str
    );

    commands.spawn((
        PausedSceneRoot,
        Text2d::new(text),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.9, 0.3)),
        Transform::from_xyz(0.0, 0.0, 16.0),
    ));
}

pub fn paused_input_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut audio: ResMut<AudioSettings>,
    mut stats: ResMut<SurvivalStats>,
    mut boss_manager: ResMut<BossManager>,
    mut damage_stats: ResMut<WeaponDamageStats>,
    mut player_query: Query<(&mut Transform, &mut PlayerStats), With<Player>>,
    battle_entities: Query<Entity, Or<(
        With<Enemy>,
        With<OrbitBlade>,
        With<HomingArrow>,
        With<HolyWaterPool>,
        With<ParabolicAxe>,
        With<ExpGem>,
        With<Particle>,
        With<FloatingDamageText>,
        With<BossProjectile>,
        With<GhostTrail>,
    )>>,
    mut bar_root_query: Query<&mut Transform, (With<BossHpBarRoot>, Without<Player>)>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Playing);
    } else if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::KeyM) {
        audio.sfx_enabled = !audio.sfx_enabled;
    } else if keyboard.just_pressed(KeyCode::KeyQ) {
        // 전장 엔티티 정리 및 타이틀로 복귀
        reset_battlefield(
            &mut commands,
            &mut stats,
            &mut boss_manager,
            &mut damage_stats,
            &mut player_query,
            &battle_entities,
            &mut bar_root_query,
        );
        next_state.set(AppState::Title);
    }
}

pub fn cleanup_paused_scene(mut commands: Commands, query: Query<Entity, With<PausedSceneRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =========================================================================
// 씬 5: 보스 강림 컷씬 (Boss Intro Scene)
// =========================================================================

pub fn setup_boss_intro_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<BossIntroTimer>,
    sounds: Res<SoundAssets>,
    mut shake: ResMut<CameraShake>,
) {
    let font = asset_server.load("fonts/malgun.ttf");
    timer.0.reset();

    commands.spawn((
        BossIntroSceneRoot,
        Sprite::from_color(Color::srgba(0.25, 0.02, 0.04, 0.88), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 15.0),
    ));

    commands.spawn((
        BossIntroSceneRoot,
        Text2d::new(
            "⚠️ WARNING! EMERGENCY RAID! ⚠️\n\n\
            에픽 보스 '메모리 리크 타이탄' 이 강림합니다!\n\n\
            방사형 8방향 탄막과 파괴적인 충돌 공격을 피하십시오!\n\n\
            [ 2.5초 후 격돌 - Space 키로 즉시 스킵 ]"
        ),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(21.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.25, 0.25)),
        Transform::from_xyz(0.0, 0.0, 16.0),
    ));

    commands.spawn((
        AudioPlayer::new(sounds.boss_spawn.clone()),
        PlaybackSettings::DESPAWN,
    ));
    shake.add_trauma(0.85);
}

pub fn boss_intro_update_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut timer: ResMut<BossIntroTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.is_finished() || keyboard.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Playing);
    }
}

pub fn cleanup_boss_intro_scene(mut commands: Commands, query: Query<Entity, With<BossIntroSceneRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =========================================================================
// 씬 6: 보스 격파 승리 씬 (Victory Scene)
// =========================================================================

pub fn setup_victory_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<SurvivalStats>,
    bestiary: Res<MonsterBestiary>,
    damage_stats: Res<WeaponDamageStats>,
) {
    let font = asset_server.load("fonts/malgun.ttf");
    let mut rng = rand::thread_rng();

    commands.spawn((
        VictorySceneRoot,
        Sprite::from_color(Color::srgba(0.03, 0.08, 0.04, 0.92), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 15.0),
    ));

    // 120개 황금 축포 파티클 발사
    for _ in 0..120 {
        let angle = rng.gen_range(0.0..(2.0 * PI));
        let speed = rng.gen_range(150.0..580.0);
        let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        let p_color = if rng.gen_bool(0.5) {
            Color::srgb(1.0, 0.85, 0.2) // 황금
        } else {
            Color::srgb(0.2, 0.9, 1.0) // 청명한 에메랄드
        };

        commands.spawn((
            VictorySceneRoot,
            Particle {
                lifetime: Timer::from_seconds(1.2, TimerMode::Once),
                drag: 0.92,
                base_color: p_color,
            },
            crate::components::Velocity(vel),
            Sprite::from_color(p_color, Vec2::splat(6.0)),
            Transform::from_xyz(0.0, 0.0, 16.5),
        ));
    }

    let mins = (stats.survival_time as u32) / 60;
    let secs = (stats.survival_time as u32) % 60;
    let total_dmg = damage_stats.total_damage();

    let text = format!(
        "👑 VICTORY! 보스 격파 달성! 👑\n\n\
        '메모리 리크 타이탄'을 토벌하고 아레나의 질서를 되찾았습니다!\n\n\
        🏆 수여 칭호: [ 전설의 메모리 안전성 수호자 ]\n\n\
        ⏱️ 생존 시간: {:02}:{:02}  |  👾 총 처치 수: {}마리  |  💎 보석: {}개\n\
        ⚔️ 총 누적 피해량: {:.0} dmg  |  📖 도감 수집률: {}/100종\n\n\
        [ Space / Enter ] 키를 눌러 타이틀로 복귀하세요!",
        mins, secs, stats.kills, stats.gems_collected, total_dmg, bestiary.discovered_names.len()
    );

    commands.spawn((
        VictorySceneRoot,
        Text2d::new(text),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.88, 0.25)),
        Transform::from_xyz(0.0, 0.0, 17.0),
    ));
}

pub fn victory_input_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut stats: ResMut<SurvivalStats>,
    mut boss_manager: ResMut<BossManager>,
    mut damage_stats: ResMut<WeaponDamageStats>,
    mut player_query: Query<(&mut Transform, &mut PlayerStats), With<Player>>,
    battle_entities: Query<Entity, Or<(
        With<Enemy>,
        With<OrbitBlade>,
        With<HomingArrow>,
        With<HolyWaterPool>,
        With<ParabolicAxe>,
        With<ExpGem>,
        With<Particle>,
        With<FloatingDamageText>,
        With<BossProjectile>,
        With<GhostTrail>,
    )>>,
    mut bar_root_query: Query<&mut Transform, (With<BossHpBarRoot>, Without<Player>)>,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::Escape)
    {
        reset_battlefield(
            &mut commands,
            &mut stats,
            &mut boss_manager,
            &mut damage_stats,
            &mut player_query,
            &battle_entities,
            &mut bar_root_query,
        );
        next_state.set(AppState::Title);
    }
}

pub fn cleanup_victory_scene(mut commands: Commands, query: Query<Entity, With<VictorySceneRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =========================================================================
// 씬 7: 레벨업 3지선다 모달 씬 (LevelUp Scene)
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
// 씬 8: 게임 오버 & 무기별 데미지(DPS Chart) 정산 씬 (GameOver Scene)
// =========================================================================

pub fn setup_gameover_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<SurvivalStats>,
    bestiary: Res<MonsterBestiary>,
    mut record: ResMut<GameRecord>,
    damage_stats: Res<WeaponDamageStats>,
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
        Sprite::from_color(Color::srgba(0.12, 0.02, 0.02, 0.94), Vec2::new(960.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    let mins = (stats.survival_time as u32) / 60;
    let secs = (stats.survival_time as u32) % 60;
    let best_mins = (record.best_survival_time_sec as u32) / 60;
    let best_secs = (record.best_survival_time_sec as u32) % 60;

    let total_dmg = damage_stats.total_damage().max(1.0);
    let blade_pct = (damage_stats.orbit_blade_damage / total_dmg) * 100.0;
    let arrow_pct = (damage_stats.homing_arrow_damage / total_dmg) * 100.0;
    let holy_pct = (damage_stats.holy_water_damage / total_dmg) * 100.0;
    let axe_pct = (damage_stats.parabolic_axe_damage / total_dmg) * 100.0;

    let text = format!(
        "💀 GAME OVER 💀\n\n\
        생존 시간: {:02}:{:02} | 처치 수: {}마리 | 보석: {}개 | 도감 수집: {}/100종\n\n\
        📊 [4대 무기 데미지 기여도 분석 - DPS Chart]\n\
        • ⚔️ 공전 칼날 (Orbit Blade)   : {:>6.0} dmg ({:>4.1}%)\n\
        • ⚡ 유도 화살 (Homing Arrow)  : {:>6.0} dmg ({:>4.1}%)\n\
        • 💧 성수 장판 (Holy Water)    : {:>6.0} dmg ({:>4.1}%)\n\
        • 🪓 관통 도끼 (Parabolic Axe) : {:>6.0} dmg ({:>4.1}%)\n\n\
        🏆 최고 기록: {:02}:{:02} (최다 {}마리, 누적 도감: {}/100종)\n\n\
        [ R ] 전장 즉시 재도전  |  [ ESC ] 메인 타이틀 복귀",
        mins, secs, stats.kills, stats.gems_collected, bestiary.discovered_names.len(),
        damage_stats.orbit_blade_damage, blade_pct,
        damage_stats.homing_arrow_damage, arrow_pct,
        damage_stats.holy_water_damage, holy_pct,
        damage_stats.parabolic_axe_damage, axe_pct,
        best_mins, best_secs, record.max_kills, record.discovered_monsters_count
    );

    commands.spawn((
        GameOverUi,
        Text2d::new(text),
        TextFont {
            font: font.into(),
            font_size: bevy::text::FontSize::Px(16.5),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.45, 0.45)),
        Transform::from_xyz(0.0, 0.0, 11.0),
    ));
}

pub fn gameover_input_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut stats: ResMut<SurvivalStats>,
    mut boss_manager: ResMut<BossManager>,
    mut damage_stats: ResMut<WeaponDamageStats>,
    mut player_query: Query<(&mut Transform, &mut PlayerStats), With<Player>>,
    battle_entities: Query<Entity, Or<(
        With<Enemy>,
        With<OrbitBlade>,
        With<HomingArrow>,
        With<HolyWaterPool>,
        With<ParabolicAxe>,
        With<ExpGem>,
        With<Particle>,
        With<FloatingDamageText>,
        With<BossProjectile>,
        With<GhostTrail>,
    )>>,
    mut bar_root_query: Query<&mut Transform, (With<BossHpBarRoot>, Without<Player>)>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        reset_battlefield(
            &mut commands,
            &mut stats,
            &mut boss_manager,
            &mut damage_stats,
            &mut player_query,
            &battle_entities,
            &mut bar_root_query,
        );
        next_state.set(AppState::Playing);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        reset_battlefield(
            &mut commands,
            &mut stats,
            &mut boss_manager,
            &mut damage_stats,
            &mut player_query,
            &battle_entities,
            &mut bar_root_query,
        );
        next_state.set(AppState::Title);
    }
}

pub fn cleanup_gameover_ui(mut commands: Commands, query: Query<Entity, With<GameOverUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =========================================================================
// 공통: 전장 리셋 헬퍼 함수
// =========================================================================

pub fn reset_battlefield(
    commands: &mut Commands,
    stats: &mut ResMut<SurvivalStats>,
    boss_manager: &mut ResMut<BossManager>,
    damage_stats: &mut ResMut<WeaponDamageStats>,
    player_query: &mut Query<(&mut Transform, &mut PlayerStats), With<Player>>,
    battle_entities: &Query<Entity, Or<(
        With<Enemy>,
        With<OrbitBlade>,
        With<HomingArrow>,
        With<HolyWaterPool>,
        With<ParabolicAxe>,
        With<ExpGem>,
        With<Particle>,
        With<FloatingDamageText>,
        With<BossProjectile>,
        With<GhostTrail>,
    )>>,
    bar_root_query: &mut Query<&mut Transform, (With<BossHpBarRoot>, Without<Player>)>,
) {
    for e in battle_entities.iter() {
        commands.entity(e).despawn();
    }

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

    // 보스바 화면 밖으로 숨김
    for mut bar_tf in bar_root_query.iter_mut() {
        bar_tf.translation.y = 1000.0;
    }

    **stats = SurvivalStats::default();
    **boss_manager = BossManager::default();
    **damage_stats = WeaponDamageStats::default();
}
