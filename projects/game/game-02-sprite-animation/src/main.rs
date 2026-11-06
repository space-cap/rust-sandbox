use bevy::prelude::*;
use game_02_sprite_animation::SpriteAnimationPlugin;

fn main() {
    App::new()
        // 2D 윈도우 및 픽셀 아트용 Nearest 필터링 설정
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "🦀 Rust Game 02: 2D Sprite Animation (Ferris)".into(),
                        resolution: (960, 600).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                // 도트 그래픽이 뭉개지지 않고 선명하게 유지되도록 Nearest 텍스처 샘플링 적용
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.08, 0.10, 0.15)))
        .add_plugins(SpriteAnimationPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_02_sprite_animation::{
        components::{
            AnimationIndices, AnimationState, AnimationTimer, Boundary, GameStats, Player, Speed,
        },
        systems::input::player_input_system,
    };
    use std::time::Duration;

    #[test]
    fn test_animation_state_transition() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<ButtonInput<KeyCode>>();
        app.init_resource::<GameStats>();
        app.add_systems(Update, player_input_system);

        let entity = app
            .world_mut()
            .spawn((
                Player,
                Speed(200.0),
                AnimationState::Idle,
                AnimationIndices::IDLE,
                Boundary::new(-100.0, 100.0, -100.0, 100.0),
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: Handle::default(),
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        // 1. 초기 상태 검증: Idle
        let state = *app.world().get::<AnimationState>(entity).unwrap();
        assert_eq!(state, AnimationState::Idle);

        // 2. 키 입력 시뮬레이션: KeyA (왼쪽 이동)
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyA);
        app.update();

        // 3. 상태가 Run으로 전환되고, flip_x가 true인지 검증
        let state = *app.world().get::<AnimationState>(entity).unwrap();
        let sprite = app.world().get::<Sprite>(entity).unwrap();
        let indices = *app.world().get::<AnimationIndices>(entity).unwrap();

        assert_eq!(state, AnimationState::Run);
        assert!(sprite.flip_x, "좌측 이동 시 flip_x는 true여야 함");
        assert_eq!(indices, AnimationIndices::RUN);
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, AnimationIndices::RUN.first);

        // 4. 키에서 손을 뗐을 때 (Release) 다시 Idle로 복귀하는지 검증
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyA);
        app.update();

        let state = *app.world().get::<AnimationState>(entity).unwrap();
        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(state, AnimationState::Idle);
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, AnimationIndices::IDLE.first);
    }

    #[test]
    fn test_frame_cycling_advancement() {
        let indices = AnimationIndices::IDLE; // 0..3
        let mut timer = AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));
        let mut sprite = Sprite {
            texture_atlas: Some(TextureAtlas {
                layout: Handle::default(),
                index: 0,
            }),
            ..default()
        };

        // 0.05초 경과 (타이머 미완료 상태 검증)
        timer.tick(Duration::from_millis(50));
        assert!(!timer.just_finished());
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 0);

        // 추가 0.06초 경과 (총 0.11초 -> 1프레임 경과)
        timer.tick(Duration::from_millis(60));
        assert!(timer.just_finished());

        // 프레임 인덱스 전진 처리
        if let Some(ref mut atlas) = sprite.texture_atlas {
            if atlas.index < indices.first || atlas.index >= indices.last {
                atlas.index = indices.first;
            } else {
                atlas.index += 1;
            }
        }
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 1);

        // 마지막 프레임(3) 도달 시 첫 번째 프레임(0)으로 순환 래핑 검증
        if let Some(ref mut atlas) = sprite.texture_atlas {
            atlas.index = 3;
            if atlas.index < indices.first || atlas.index >= indices.last {
                atlas.index = indices.first;
            } else {
                atlas.index += 1;
            }
        }
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 0);
    }
}
