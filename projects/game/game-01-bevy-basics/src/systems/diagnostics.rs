use bevy::prelude::*;
use crate::components::{GameStats, HudInfoText, Player, Speed};

/// 플레이어 위치 및 통계를 HUD 텍스트와 콘솔로 출력/갱신하는 시스템
pub fn diagnostics_system(
    mut stats: ResMut<GameStats>,
    player_query: Query<(&Transform, &Speed), With<Player>>,
    mut text_query: Query<&mut Text2d, With<HudInfoText>>,
) {
    stats.frame_count += 1;

    if let Ok((transform, speed)) = player_query.single() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        // 하단 HUD 텍스트 갱신
        for mut text in text_query.iter_mut() {
            **text = format!(
                "위치: (X: {:.1}, Y: {:.1}) | 속도: {:.0} px/s | 누적 이동: {:.1} px | 프레임: {}",
                x, y, speed.0, stats.total_distance_moved, stats.frame_count
            );
        }

        // 300 프레임마다 한 번씩 콘솔에 진단 로그 출력
        if stats.frame_count % 300 == 0 {
            info!(
                "📊 [ECS 진단] 프레임: {}, 플레이어 좌표: ({:.1}, {:.1}), 총 이동 거리: {:.1} px",
                stats.frame_count, x, y, stats.total_distance_moved
            );
        }
    }
}

