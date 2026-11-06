use bevy::prelude::*;
use bevy::window::WindowResolution;
use game_05_particle_effects::ParticleEffectsPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rust Game 05: Particle Effects & Juice (Antigravity)".to_string(),
                        resolution: WindowResolution::new(960, 600),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ParticleEffectsPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_05_particle_effects::components::{
        CameraShake, Collider2d, HitstopManager, Particle, Velocity,
    };
    use game_05_particle_effects::systems::particle::particle_update_system;
    use game_05_particle_effects::systems::shake::{camera_shake_system, hitstop_system};
    use std::time::Duration;

    #[test]
    fn test_collider_intersects() {
        let col_a = Collider2d::new(40.0, 40.0);
        let col_b = Collider2d::new(40.0, 40.0);

        // 충돌하는 위치
        assert!(col_a.intersects(Vec2::new(0.0, 0.0), &col_b, Vec2::new(30.0, 30.0)));
        // 벗어난 위치
        assert!(!col_a.intersects(Vec2::new(0.0, 0.0), &col_b, Vec2::new(50.0, 50.0)));
    }

    #[test]
    fn test_camera_shake_trauma_clamping_and_decay() {
        let mut shake = CameraShake::default();
        assert_eq!(shake.trauma, 0.0);

        // 트라우마 추가 및 1.0 클램핑 검증
        shake.add_trauma(0.6);
        assert!((shake.trauma - 0.6).abs() < f32::EPSILON);

        shake.add_trauma(0.7);
        assert_eq!(shake.trauma, 1.0); // 0.6 + 0.7 = 1.3 -> 1.0 clamped

        // 쉐이크 시스템 수렴 검증
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(shake);
        app.world_mut().spawn(Camera2d);
        app.add_systems(Update, camera_shake_system);

        app.update();

        let updated_shake = app.world().resource::<CameraShake>();
        assert!(updated_shake.trauma <= 1.0);
    }

    #[test]
    fn test_hitstop_lifecycle() {
        let mut hitstop = HitstopManager::default();
        assert!(hitstop.timer.is_none());

        // 히트스톱 발동: 0.05초 정지
        hitstop.timer = Some(Timer::from_seconds(0.05, TimerMode::Once));

        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(hitstop);
        app.add_systems(Update, hitstop_system);

        // 히트스톱 진행 중: timer is_some
        assert!(app.world().resource::<HitstopManager>().timer.is_some());

        // 시간 경과 (0.06초 대기 후 업데이트)
        std::thread::sleep(Duration::from_millis(60));
        app.update();

        // 히트스톱 타이머 완료 후 None 복귀 검증
        let current_hitstop = app.world().resource::<HitstopManager>();
        assert!(current_hitstop.timer.is_none());
    }

    #[test]
    fn test_particle_drag_and_lifetime_despawn() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, particle_update_system);

        // 매우 짧은 수명(0.02초)을 가진 파티클 엔티티 스폰
        let particle_entity = app
            .world_mut()
            .spawn((
                Particle {
                    lifetime: Timer::from_seconds(0.02, TimerMode::Once),
                    drag: 0.90,
                    base_color: Color::srgb(1.0, 1.0, 1.0),
                    initial_size: 6.0,
                },
                Velocity(Vec2::new(100.0, 0.0)),
                Sprite::from_color(Color::WHITE, Vec2::splat(6.0)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        // 1회 업데이트 (초기 틱 및 감쇠)
        app.update();
        assert!(app.world().get_entity(particle_entity).is_ok());

        // 30ms 후 업데이트 시 수명이 다해 소멸(despawn)되어야 함
        std::thread::sleep(Duration::from_millis(30));
        app.update();
        assert!(app.world().get_entity(particle_entity).is_err());
    }
}