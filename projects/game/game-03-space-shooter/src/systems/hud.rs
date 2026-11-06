use bevy::prelude::*;
use crate::components::{GameState, HudText, LaserBullet};

/// 실시간 게임 상태 및 점수 HUD 갱신 시스템
pub fn hud_update_system(
    game_state: Res<GameState>,
    bullet_query: Query<&LaserBullet>,
    mut text_query: Query<&mut Text2d, With<HudText>>,
) {
    let bullet_count = bullet_query.iter().count();

    for mut text in text_query.iter_mut() {
        if game_state.is_game_over {
            **text = format!(
                "💥 GAME OVER! 최종 점수: {}점 | 격추한 적기: {}대 | [R] 키를 눌러 재출격하세요!",
                game_state.score, game_state.enemies_killed
            );
        } else {
            let shield_bar = match game_state.shields {
                3 => "[###]",
                2 => "[##-]",
                1 => "[#--]",
                _ => "[---]",
            };

            **text = format!(
                "점수: {}점 | 쉴드: {} ({}/3) | 적기 격추: {}대 | 활성 레이저: {}발",
                game_state.score,
                shield_bar,
                game_state.shields,
                game_state.enemies_killed,
                bullet_count
            );
        }
    }
}

