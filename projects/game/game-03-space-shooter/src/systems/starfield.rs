use bevy::prelude::*;
use crate::components::{Star, Velocity};
use rand::Rng;

/// 우주 배경 별무리(Starfield) 스크롤 및 무한 루프 시스템
pub fn starfield_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Star>>,
) {
    let mut rng = rand::thread_rng();

    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();

        // 화면 하단(-300.0)을 지나면 화면 상단(+300.0)으로 재배치하여 무한 순환
        if transform.translation.y < -300.0 {
            transform.translation.y = 300.0;
            transform.translation.x = rng.gen_range(-460.0..460.0);
        }
    }
}

