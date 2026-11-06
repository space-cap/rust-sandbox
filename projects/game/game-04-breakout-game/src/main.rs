use bevy::prelude::*;
use game_04_breakout_game::BreakoutPlugin;

fn main() {
    App::new()
        // 2D 게임 윈도우 설정
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Game 04: Breakout & Game States".into(),
                resolution: (960, 600).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // 어두운 아케이드 배경색
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.11)))
        .add_plugins(BreakoutPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_04_breakout_game::{
        components::{
            AppState, Ball, Brick, Collider2d, GameScore, SoundEffects, Velocity,
        },
        systems::{
            collision::breakout_collision_system,
            state::menu_input_system,
        },
    };

    #[test]
    fn test_paddle_variable_reflection_math() {
        let half_w: f32 = 60.0; // 패들 너비 120 / 2
        let speed: f32 = 440.0;

        // 1. 패들 정중앙 타격 (offset = 0.0) -> 수직 반사 (vx = 0, vy = speed)
        let ball_x: f32 = 0.0;
        let paddle_x: f32 = 0.0;
        let offset = ((ball_x - paddle_x) / half_w).clamp(-0.92, 0.92);
        let vx = speed * offset;
        let vy = (speed * speed - vx * vx).sqrt();

        assert_eq!(offset, 0.0);
        assert_eq!(vx, 0.0);
        assert_eq!(vy, speed);

        // 2. 패들 우측 모서리 타격 -> 우측 상향 예각 반사 (vx > 0, vy > 0)
        let ball_x: f32 = 45.0;
        let offset = ((ball_x - paddle_x) / half_w).clamp(-0.92, 0.92);
        let vx = speed * offset;
        let vy = (speed * speed - vx * vx).sqrt();

        assert!(offset > 0.0);
        assert!(vx > 0.0);
        assert!(vy > 0.0);

        // 3. 패들 좌측 모서리 타격 -> 좌측 상향 예각 반사 (vx < 0, vy > 0)
        let ball_x: f32 = -45.0;
        let offset = ((ball_x - paddle_x) / half_w).clamp(-0.92, 0.92);
        let vx = speed * offset;
        let vy = (speed * speed - vx * vx).sqrt();

        assert!(offset < 0.0);
        assert!(vx < 0.0);
        assert!(vy > 0.0);
    }

    #[test]
    fn test_brick_collision_and_score() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.init_state::<AppState>();
        app.insert_resource(GameScore {
            score: 0,
            lives: 3,
            total_bricks: 1,
            remaining_bricks: 1,
        });
        app.insert_resource(SoundEffects {
            bounce: Handle::default(),
            brick_break: Handle::default(),
            game_over: Handle::default(),
        });
        app.add_systems(Update, breakout_collision_system);

        // 1. 공 엔티티 스폰 (50.0, 50.0)
        app.world_mut().spawn((
            Ball,
            Velocity(Vec2::new(100.0, 200.0)),
            Collider2d::new(14.0, 14.0),
            Transform::from_xyz(50.0, 50.0, 0.0),
        ));

        // 2. 벽돌 엔티티 스폰 (52.0, 52.0 -> 충돌 위치)
        let brick = app
            .world_mut()
            .spawn((
                Brick { points: 50 },
                Collider2d::new(60.0, 20.0),
                Transform::from_xyz(52.0, 52.0, 0.0),
            ))
            .id();

        app.update();

        // 3. 점수 증가 및 벽돌 소멸 검증
        let score = app.world().resource::<GameScore>();
        assert_eq!(score.score, 50);
        assert_eq!(score.remaining_bricks, 0);
        assert!(app.world().get_entity(brick).is_err());
    }

    #[test]
    fn test_menu_state_transition() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.init_state::<AppState>();
        app.init_resource::<ButtonInput<KeyCode>>();
        app.add_systems(Update, menu_input_system.run_if(in_state(AppState::Menu)));

        // 1. 초기 상태는 Menu
        let state = app.world().resource::<State<AppState>>().get();
        assert_eq!(*state, AppState::Menu);

        // 2. Space 키 입력
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Space);
        app.update();

        // 3. NextState가 Playing으로 예약되었는지 검증
        let next_state = app.world().resource::<NextState<AppState>>();
        match next_state {
            NextState::Pending(s) => assert_eq!(*s, AppState::Playing),
            _ => panic!("NextState should be Pending(Playing)"),
        }
    }
}
