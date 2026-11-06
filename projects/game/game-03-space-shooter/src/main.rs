use bevy::prelude::*;
use game_03_space_shooter::SpaceShooterPlugin;

fn main() {
    App::new()
        // 2D 게임 윈도우 설정
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Game 03: Space Shooter & AABB Collision".into(),
                resolution: (960, 600).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // 심우주 밤하늘 배경색 (다크 네이비 블랙)
        .insert_resource(ClearColor(Color::srgb(0.03, 0.04, 0.07)))
        .add_plugins(SpaceShooterPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_03_space_shooter::{
        components::{Collider2d, Enemy, GameState, LaserBullet, Velocity},
        systems::{bullet::bullet_movement_system, collision::collision_detection_system},
    };

    #[test]
    fn test_aabb_intersection_math() {
        let col_a = Collider2d::new(20.0, 20.0);
        let col_b = Collider2d::new(20.0, 20.0);

        // 1. 완전히 겹치는 경우
        assert!(col_a.intersects(Vec2::new(0.0, 0.0), &col_b, Vec2::new(5.0, 5.0)));

        // 2. 가장자리만 아슬아슬하게 닿은 경우 (dx = 19.0 < 20.0)
        assert!(col_a.intersects(Vec2::new(0.0, 0.0), &col_b, Vec2::new(19.0, 0.0)));

        // 3. 완전히 분리되어 충돌하지 않는 경우 (dx = 25.0 >= 20.0)
        assert!(!col_a.intersects(Vec2::new(0.0, 0.0), &col_b, Vec2::new(25.0, 0.0)));
    }

    #[test]
    fn test_headless_bullet_enemy_collision() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<GameState>();
        app.add_systems(Update, collision_detection_system);

        // 1. 총알 엔티티 스폰 (50.0, 50.0)
        let bullet = app
            .world_mut()
            .spawn((
                LaserBullet,
                Collider2d::new(10.0, 10.0),
                Transform::from_xyz(50.0, 50.0, 0.0),
            ))
            .id();

        // 2. 적기 엔티티 스폰 (52.0, 52.0 -> AABB 겹침)
        let enemy = app
            .world_mut()
            .spawn((
                Enemy,
                Collider2d::new(20.0, 20.0),
                Transform::from_xyz(52.0, 52.0, 0.0),
            ))
            .id();

        // 3. ECS 1 틱 업데이트 실행 (충돌 감지 시스템 발동)
        app.update();

        // 4. 게임 상태 갱신 검증: 점수 +100, 격추 수 1
        let state = app.world().resource::<GameState>();
        assert_eq!(state.score, 100);
        assert_eq!(state.enemies_killed, 1);

        // 5. 충돌한 두 엔티티가 안전하게 소멸(Despawn)되었는지 검증
        assert!(app.world().get_entity(bullet).is_err());
        assert!(app.world().get_entity(enemy).is_err());
    }

    #[test]
    fn test_bullet_offscreen_despawn() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, bullet_movement_system);

        // 화면 상단(+310.0)을 벗어난 위치에 있는 총알 스폰
        let bullet = app
            .world_mut()
            .spawn((
                LaserBullet,
                Velocity(Vec2::new(0.0, 500.0)),
                Transform::from_xyz(0.0, 315.0, 0.0),
            ))
            .id();

        app.update();

        // 화면 밖 총알이 즉시 소멸되었는지 검증
        assert!(app.world().get_entity(bullet).is_err());
    }
}

