use bevy::prelude::*;
use crate::components::{CameraShake, HitstopManager};
use rand::Rng;

/// 트라우마 2차 곡선 모델 기반 카메라 쉐이크 및 감쇠 시스템
pub fn camera_shake_system(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let Ok(mut camera_tf) = camera_query.single_mut() else {
        return;
    };

    if shake.trauma > 0.001 {
        // 비선형 2차 곡선 (Trauma^2): 강한 충격일수록 진동이 기하급수적으로 폭발
        let shake_intensity = shake.trauma * shake.trauma;
        let mut rng = rand::thread_rng();

        let max_displacement = 18.0;
        let offset_x = (rng.gen::<f32>() * 2.0 - 1.0) * max_displacement * shake_intensity;
        let offset_y = (rng.gen::<f32>() * 2.0 - 1.0) * max_displacement * shake_intensity;

        camera_tf.translation.x = offset_x;
        camera_tf.translation.y = offset_y;

        // 초당 1.6의 감쇠율로 자연스러운 원위치 수렴
        shake.trauma = (shake.trauma - 1.6 * time.delta_secs()).max(0.0);
    } else {
        // 쉐이크 종료 시 원점 복귀
        camera_tf.translation.x = 0.0;
        camera_tf.translation.y = 0.0;
    }
}

/// 히트스톱(시간 순간 정지) 타이머 틱 관리 시스템
pub fn hitstop_system(
    time: Res<Time>,
    mut hitstop: ResMut<HitstopManager>,
) {
    if let Some(ref mut timer) = hitstop.timer {
        timer.tick(time.delta());
        if timer.is_finished() {
            hitstop.timer = None;
        }
    }
}

