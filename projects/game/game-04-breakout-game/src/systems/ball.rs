use bevy::prelude::*;
use crate::components::{AppState, Ball, GameScore, SoundEffects, Velocity};

/// 공 이동 및 하단 추락(목숨 차감 / 게임오버) 판정 시스템
pub fn ball_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut score: ResMut<GameScore>,
    sounds: Res<SoundEffects>,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        // 공이 바닥으로 추락했을 때
        if transform.translation.y < -265.0 {
            if score.lives > 1 {
                score.lives -= 1;
                // 공 재배치
                transform.translation = Vec3::new(0.0, -200.0, 1.0);
                velocity.0 = Vec2::new(220.0, 340.0);

                // 피격 사운드
                commands.spawn((
                    AudioPlayer::new(sounds.bounce.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            } else {
                score.lives = 0;
                // 게임 오버 상태 전환
                next_state.set(AppState::GameOver);

                commands.spawn((
                    AudioPlayer::new(sounds.game_over.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}

