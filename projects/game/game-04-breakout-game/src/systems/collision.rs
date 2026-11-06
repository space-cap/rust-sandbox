use bevy::prelude::*;
use crate::components::{
    AppState, Ball, Brick, Collider2d, GameScore, Paddle, SoundEffects, Velocity,
};

/// 공과 벽, 패들, 벽돌 간의 탄성 충돌 및 반사각 물리 시스템
pub fn breakout_collision_system(
    mut commands: Commands,
    mut score: ResMut<GameScore>,
    sounds: Res<SoundEffects>,
    mut next_state: ResMut<NextState<AppState>>,
    mut ball_query: Query<(&mut Velocity, &mut Transform, &Collider2d), With<Ball>>,
    paddle_query: Query<(&Transform, &Collider2d), (With<Paddle>, Without<Ball>)>,
    brick_query: Query<(Entity, &Transform, &Collider2d, &Brick), Without<Ball>>,
) {
    let Ok((mut ball_vel, ball_tf, ball_col)) = ball_query.single_mut() else {
        return;
    };

    let ball_pos = ball_tf.translation.truncate();

    // 1. 좌우 벽 및 천장 충돌 검사
    let left_wall = -390.0 + 7.0;
    let right_wall = 390.0 - 7.0;
    let ceiling = 260.0 - 7.0;

    let mut play_bounce = false;

    if ball_pos.x <= left_wall && ball_vel.0.x < 0.0 {
        ball_vel.0.x = ball_vel.0.x.abs();
        play_bounce = true;
    } else if ball_pos.x >= right_wall && ball_vel.0.x > 0.0 {
        ball_vel.0.x = -ball_vel.0.x.abs();
        play_bounce = true;
    }

    if ball_pos.y >= ceiling && ball_vel.0.y > 0.0 {
        ball_vel.0.y = -ball_vel.0.y.abs();
        play_bounce = true;
    }

    // 2. 패들 가변 반사각(스킬샷) 충돌 검사
    if let Ok((paddle_tf, paddle_col)) = paddle_query.single() {
        let paddle_pos = paddle_tf.translation.truncate();

        // 공이 하강 중일 때만 패들 충돌 허용 (내부 갇힘 방지)
        if ball_vel.0.y < 0.0 && ball_col.intersects(ball_pos, paddle_col, paddle_pos) {
            let half_w = paddle_col.size.x * 0.5;
            let offset = ((ball_pos.x - paddle_pos.x) / half_w).clamp(-0.92, 0.92);

            let speed = 440.0;
            ball_vel.0.x = speed * offset;
            ball_vel.0.y = (speed * speed - ball_vel.0.x * ball_vel.0.x).sqrt();
            play_bounce = true;
        }
    }

    if play_bounce {
        commands.spawn((
            AudioPlayer::new(sounds.bounce.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }

    // 3. 벽돌 파괴 및 법선 반사 충돌 검사
    for (brick_e, brick_tf, brick_col, brick) in brick_query.iter() {
        let brick_pos = brick_tf.translation.truncate();

        if ball_col.intersects(ball_pos, brick_col, brick_pos) {
            commands.entity(brick_e).despawn();

            // 충돌한 면(수평 vs 수직)에 따라 반사 방향 결정
            let dx = (ball_pos.x - brick_pos.x) / (brick_col.size.x * 0.5);
            let dy = (ball_pos.y - brick_pos.y) / (brick_col.size.y * 0.5);

            if dx.abs() > dy.abs() {
                ball_vel.0.x = -ball_vel.0.x;
            } else {
                ball_vel.0.y = -ball_vel.0.y;
            }

            score.score += brick.points;
            score.remaining_bricks = score.remaining_bricks.saturating_sub(1);

            // 벽돌 파괴 사운드 재생
            commands.spawn((
                AudioPlayer::new(sounds.brick_break.clone()),
                PlaybackSettings::DESPAWN,
            ));

            // 모든 벽돌을 파괴한 경우 승리(Victory) 상태 전환
            if score.remaining_bricks == 0 {
                next_state.set(AppState::Victory);
            }

            break; // 프레임당 1개 벽돌 처리
        }
    }
}
