use bevy::prelude::*;
use bevy::window::WindowResolution;
use game_06_survival_arena::SurvivalArenaPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rust Game 06: Ferris Survivors (2D Arena) - Antigravity".to_string(),
                        resolution: WindowResolution::new(960, 600),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SurvivalArenaPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_06_survival_arena::components::{
        Collider2d, GameRecord, PlayerStats, UpgradeType,
    };
    use std::f32::consts::PI;

    #[test]
    fn test_collider_intersection() {
        let col_player = Collider2d::new(36.0, 36.0);
        let col_enemy = Collider2d::new(26.0, 26.0);

        // 충돌 위치
        assert!(col_player.intersects(Vec2::new(0.0, 0.0), &col_enemy, Vec2::new(20.0, 20.0)));
        // 비충돌 위치
        assert!(!col_player.intersects(Vec2::new(0.0, 0.0), &col_enemy, Vec2::new(80.0, 80.0)));
    }

    #[test]
    fn test_enemy_steering_vector_normalization() {
        let player_pos = Vec2::new(100.0, 50.0);
        let enemy_pos = Vec2::new(40.0, -30.0);

        let to_player = player_pos - enemy_pos;
        assert!(to_player.length() > 0.0);

        let dir = to_player.normalize();
        assert!((dir.length() - 1.0).abs() < 1e-4);

        // 방향 벡터로 1초 이동 시 플레이어에 더 가까워지는지 검증
        let speed = 100.0;
        let new_enemy_pos = enemy_pos + dir * speed * 0.1;
        assert!(new_enemy_pos.distance(player_pos) < enemy_pos.distance(player_pos));
    }

    #[test]
    fn test_orbit_blade_circular_trigonometry() {
        let center = Vec2::new(0.0, 0.0);
        let radius: f32 = 80.0;

        // angle = 0 -> (80, 0)
        let angle_0: f32 = 0.0;
        let pos_0 = center + Vec2::new(radius * angle_0.cos(), radius * angle_0.sin());
        assert!((pos_0.x - 80.0).abs() < 1e-4);
        assert!(pos_0.y.abs() < 1e-4);

        // angle = PI / 2 -> (0, 80)
        let angle_90 = PI * 0.5;
        let pos_90 = center + Vec2::new(radius * angle_90.cos(), radius * angle_90.sin());
        assert!(pos_90.x.abs() < 1e-4);
        assert!((pos_90.y - 80.0).abs() < 1e-4);
    }

    #[test]
    fn test_player_stats_level_up_and_upgrade() {
        let mut stats = PlayerStats::default();
        assert_eq!(stats.level, 1);
        assert_eq!(stats.exp, 0.0);
        assert_eq!(stats.next_level_exp, 15.0);

        // 경험치 획득 및 레벨업
        stats.exp += 20.0;
        if stats.exp >= stats.next_level_exp {
            stats.exp -= stats.next_level_exp;
            stats.level += 1;
            stats.next_level_exp = (stats.next_level_exp * 1.35).round();
        }

        assert_eq!(stats.level, 2);
        assert_eq!(stats.exp, 5.0);
        assert_eq!(stats.next_level_exp, 20.0); // 15 * 1.35 = 20.25 -> round 20

        // 스킬 업그레이드 적용 검증 (이동 속도 15% 증가)
        let prev_speed = stats.speed;
        let upgrade = UpgradeType::MoveSpeed;
        match upgrade {
            UpgradeType::MoveSpeed => stats.speed *= 1.15,
            _ => {}
        }
        assert!((stats.speed - prev_speed * 1.15).abs() < 1e-3);
    }

    #[test]
    fn test_game_record_serialization_and_persistence() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_ferris_survivors_record.json");
        let path_str = file_path.to_str().unwrap();

        let record = GameRecord {
            best_survival_time_sec: 185.5,
            max_kills: 120,
        };

        // 저장
        assert!(record.save(path_str).is_ok());

        // 로드
        let loaded = GameRecord::load_or_default(path_str);
        assert!((loaded.best_survival_time_sec - 185.5).abs() < 1e-3);
        assert_eq!(loaded.max_kills, 120);

        // 정리
        let _ = std::fs::remove_file(file_path);
    }
}

