use bevy::prelude::*;
use game_01_bevy_basics::GameBasicsPlugin;

fn main() {
    App::new()
        // 2D 게임 윈도우 및 기본 플러그인 설정
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🦀 Rust Game 01: Bevy ECS 2D Basics".into(),
                resolution: (960, 600).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // 어두운 배경색 설정 (RGB 0.1, 0.12, 0.18)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.10, 0.15)))
        // 게임 핵심 로직 플러그인 등록
        .add_plugins(GameBasicsPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_01_bevy_basics::{
        components::{Boundary, GameStats, Player, Speed},
        systems::movement::player_movement_system,
    };

    #[test]
    fn test_boundary_clamping() {
        let boundary = Boundary::new(-100.0, 100.0, -50.0, 50.0);

        // 경계 내부 좌표 테스트
        let mut inside = Vec3::new(20.0, -30.0, 0.0);
        boundary.clamp(&mut inside);
        assert_eq!(inside.x, 20.0);
        assert_eq!(inside.y, -30.0);

        // 경계 초과 좌표 클램핑 테스트
        let mut outside = Vec3::new(250.0, -999.0, 0.0);
        boundary.clamp(&mut outside);
        assert_eq!(outside.x, 100.0);
        assert_eq!(outside.y, -50.0);
    }

    #[test]
    fn test_headless_ecs_movement() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<ButtonInput<KeyCode>>();
        app.init_resource::<GameStats>();
        app.add_systems(Update, player_movement_system);

        let entity = app
            .world_mut()
            .spawn((
                Player,
                Speed(200.0),
                Boundary::new(-500.0, 500.0, -500.0, 500.0),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        // KeyCode::KeyD (오른쪽 이동) 키 입력 시뮬레이션
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyD);

        // ECS 월드 1 틱 업데이트 실행
        app.update();

        // 플레이어의 Transform 컴포넌트 조회 및 좌표 검증
        let transform = app.world().get::<Transform>(entity).expect("Player must exist");
        assert!(
            transform.translation.x >= 0.0,
            "Player X should be >= 0.0 after pressing right"
        );
    }
}
