use bevy::prelude::*;
use crate::components::{Collider2d, Enemy, EnemySpawnTimer, GameState, Velocity};
use rand::Rng;

/// 주기적으로 상단에서 외계 적기를 생성하는 시스템
pub fn enemy_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if game_state.is_game_over {
        return;
    }

    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.just_finished() {
        let mut rng = rand::thread_rng();
        let spawn_x = rng.gen_range(-410.0..410.0);
        let speed = rng.gen_range(150.0..240.0);
        let drift_x = rng.gen_range(-25.0..25.0);

        commands.spawn((
            Enemy,
            Velocity(Vec2::new(drift_x, -speed)),
            Collider2d::new(30.0, 30.0),
            Sprite::from_color(
                Color::srgb(1.0, 0.25, 0.2), // 크림슨 레드 외계 적기
                Vec2::new(30.0, 30.0),
            ),
            Transform::from_xyz(spawn_x, 300.0, 1.0),
        ));
    }
}

/// 적기 하강 이동 및 화면 하단 이탈 시 자동 소멸 시스템
pub fn enemy_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Enemy>>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        // 화면 하단(-310.0) 도달 시 소멸
        if transform.translation.y < -310.0 {
            commands.entity(entity).despawn();
        }
    }
}

