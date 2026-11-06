use bevy::prelude::*;
use crate::components::{AnimationState, GameStats, HudInfoText, Player, Speed};

/// HUD 텍스트 갱신 및 디버그 진단 시스템
pub fn diagnostics_system(
    mut stats: ResMut<GameStats>,
    player_query: Query<(&Transform, &Speed, &AnimationState, &Sprite), With<Player>>,
    mut text_query: Query<&mut Text2d, With<HudInfoText>>,
) {
    stats.frame_count += 1;

    if let Ok((transform, speed, state, sprite)) = player_query.single() {
        let atlas_index = sprite
            .texture_atlas
            .as_ref()
            .map(|a| a.index)
            .unwrap_or(0);
        let flip = if sprite.flip_x { "좌측 (반전)" } else { "우측 (정방향)" };
        let state_str = match state {
            AnimationState::Idle => "대기 (Idle)",
            AnimationState::Run => "달리기 (Run)",
        };

        for mut text in text_query.iter_mut() {
            **text = format!(
                "상태: {} | 프레임: #{} | 방향: {} | 좌표: ({:.1}, {:.1}) | 누적 이동: {:.1} px | 속도: {:.0} px/s",
                state_str,
                atlas_index,
                flip,
                transform.translation.x,
                transform.translation.y,
                stats.total_distance,
                speed.0
            );
        }

        if stats.frame_count % 300 == 0 {
            info!(
                "🏃 [Phase 2] 상태: {}, 아틀라스 인덱스: {}, 좌표: ({:.1}, {:.1})",
                state_str, atlas_index, transform.translation.x, transform.translation.y
            );
        }
    }
}

