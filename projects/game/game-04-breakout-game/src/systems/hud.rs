use bevy::prelude::*;
use crate::components::{AppState, GameScore, HudText};

/// 실시간 게임 점수, 목숨 및 남은 벽돌 현황 HUD 갱신
pub fn hud_update_system(
    score: Res<GameScore>,
    state: Res<State<AppState>>,
    mut query: Query<&mut Text2d, With<HudText>>,
) {
    let hearts = match score.lives {
        3 => "[♥♥♥]",
        2 => "[♥♥ ]",
        1 => "[♥  ]",
        _ => "[   ]",
    };

    let state_str = match state.get() {
        AppState::Menu => "메뉴 대기",
        AppState::Playing => "게임 진행 중",
        AppState::Paused => "일시정지",
        AppState::GameOver => "게임 오버",
        AppState::Victory => "스테이지 클리어!",
    };

    for mut text in query.iter_mut() {
        **text = format!(
            "점수: {}점 | 목숨: {} ({}) | 남은 벽돌: {}/{}개 | 상태: {}",
            score.score,
            hearts,
            score.lives,
            score.remaining_bricks,
            score.total_bricks,
            state_str
        );
    }
}

