use bevy::prelude::*;
use crate::components::{
    AppState, CameraShake, ExpGem, Player, PlayerStats, SoundAssets, SurvivalStats, UpgradeChoices,
    UpgradeType,
};
use rand::seq::SliceRandom;

/// 마그넷 반경 인력 흡수 및 레벨업 판정 시스템
pub fn exp_gem_system(
    mut commands: Commands,
    time: Res<Time>,
    sounds: Res<SoundAssets>,
    mut stats: ResMut<SurvivalStats>,
    mut shake: ResMut<CameraShake>,
    mut next_state: ResMut<NextState<AppState>>,
    mut upgrade_choices: ResMut<UpgradeChoices>,
    mut player_query: Query<(&Transform, &mut PlayerStats), With<Player>>,
    mut gem_query: Query<(Entity, &mut Transform, &mut ExpGem), Without<Player>>,
) {
    let Ok((player_tf, mut player_stats)) = player_query.single_mut() else {
        return;
    };
    let player_pos = player_tf.translation.truncate();

    for (gem_e, mut gem_tf, mut gem) in gem_query.iter_mut() {
        let gem_pos = gem_tf.translation.truncate();
        let to_player = player_pos - gem_pos;
        let dist = to_player.length();

        if dist < player_stats.magnet_radius {
            gem.is_attracted = true;
        }

        if gem.is_attracted {
            let dir = to_player.normalize();
            let fly_speed = 520.0;
            gem_tf.translation.x += dir.x * fly_speed * time.delta_secs();
            gem_tf.translation.y += dir.y * fly_speed * time.delta_secs();

            if dist < 24.0 {
                commands.entity(gem_e).despawn();
                stats.gems_collected += 1;
                player_stats.exp += gem.value;

                commands.spawn((
                    AudioPlayer::new(sounds.gem.clone()),
                    PlaybackSettings::DESPAWN,
                ));

                if player_stats.exp >= player_stats.next_level_exp {
                    player_stats.exp -= player_stats.next_level_exp;
                    player_stats.level += 1;
                    player_stats.next_level_exp = (player_stats.next_level_exp * 1.35).round();

                    let mut all_upgrades = vec![
                        UpgradeType::ExtraBlade,
                        UpgradeType::ArrowCooldown,
                        UpgradeType::HolyWaterPool,
                        UpgradeType::BouncingAxe,
                        UpgradeType::HealAndMaxHp,
                        UpgradeType::MagnetRadius,
                        UpgradeType::MoveSpeed,
                    ];
                    let mut rng = rand::thread_rng();
                    all_upgrades.shuffle(&mut rng);
                    upgrade_choices.choices = all_upgrades.into_iter().take(3).collect();

                    commands.spawn((
                        AudioPlayer::new(sounds.levelup.clone()),
                        PlaybackSettings::DESPAWN,
                    ));

                    shake.add_trauma(0.40);
                    next_state.set(AppState::LevelUp);
                    return;
                }
            }
        }
    }
}

