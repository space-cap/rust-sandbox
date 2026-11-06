use bevy::prelude::*;
use crate::components::{CameraShake, HitstopTimer};
use rand::Rng;

/// 트라우마 2차 곡선 모델(Trauma^2) 카메라 흔들림 및 감쇠 시스템
pub fn camera_shake_system(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let Ok(mut camera_tf) = camera_query.single_mut() else {
        return;
    };

    if shake.trauma > 0.001 {
        let intensity = shake.trauma * shake.trauma;
        let mut rng = rand::thread_rng();

        let max_disp = 16.0;
        let offset_x = (rng.gen::<f32>() * 2.0 - 1.0) * max_disp * intensity;
        let offset_y = (rng.gen::<f32>() * 2.0 - 1.0) * max_disp * intensity;

        camera_tf.translation.x = offset_x;
        camera_tf.translation.y = offset_y;

        shake.trauma = (shake.trauma - 1.5 * time.delta_secs()).max(0.0);
    } else {
        camera_tf.translation.x = 0.0;
        camera_tf.translation.y = 0.0;
    }
}

/// 히트스톱 타이머 관리 시스템
pub fn hitstop_system(time: Res<Time>, mut hitstop: ResMut<HitstopTimer>) {
    if let Some(ref mut timer) = hitstop.timer {
        timer.tick(time.delta());
        if timer.is_finished() {
            hitstop.timer = None;
        }
    }
}
