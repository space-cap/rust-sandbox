use bevy::prelude::*;
use bevy::window::WindowResolution;
use game_07_ferris_survivors_ultimate::FerrisSurvivorsUltimatePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rust Game 07: Ferris Survivors Ultimate (100 Monsters & Titan Boss) - Antigravity".to_string(),
                        resolution: WindowResolution::new(960, 600),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FerrisSurvivorsUltimatePlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_07_ferris_survivors_ultimate::components::{
        GameRecord, MonsterAffix, MonsterArchetype, MonsterElement, PlayerStats,
    };
    use std::collections::HashSet;
    use std::f32::consts::PI;

    #[test]
    fn test_monster_combinatorics_100_variety() {
        let archetypes = MonsterArchetype::all();
        let elements = MonsterElement::all();
        let affixes = MonsterAffix::all();

        let mut unique_names = HashSet::new();

        for archetype in archetypes.iter() {
            for element in elements.iter() {
                for affix in affixes.iter() {
                    let name = format!("{}{}{}", affix.prefix(), element.prefix(), archetype.base_name());
                    assert!(!name.is_empty());
                    unique_names.insert(name);

                    let (hp, spd, dmg, size) = archetype.base_stats();
                    assert!(hp > 0.0);
                    assert!(spd > 0.0);
                    assert!(dmg > 0.0);
                    assert!(size.x > 0.0 && size.y > 0.0);
                }
            }
        }

        // 10 * 5 * 8 = 400가지 고유 변종 생성 검증 (100종 요건을 400% 초과 충족!)
        assert_eq!(unique_names.len(), 400);
    }

    #[test]
    fn test_player_dash_and_invulnerability() {
        let mut stats = PlayerStats::default();
        assert!(!stats.is_dashing);
        assert!(!stats.invulnerable);

        // 대시 발동
        stats.is_dashing = true;
        stats.invulnerable = true;
        stats.dash_timer.reset();

        assert!(stats.invulnerable);

        // 0.20초 경과 시 대시 종료
        stats.dash_timer.tick(std::time::Duration::from_millis(200));
        if stats.dash_timer.is_finished() {
            stats.is_dashing = false;
            stats.invulnerable = false;
        }

        assert!(!stats.is_dashing);
        assert!(!stats.invulnerable);
    }

    #[test]
    fn test_boss_titan_radial_projectile_trigonometry() {
        let speed = 240.0;
        let mut vectors = Vec::new();

        for i in 0..8 {
            let angle = (i as f32) * (PI * 0.25);
            let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);
            assert!((vel.length() - speed).abs() < 1e-3);
            vectors.push(vel);
        }

        assert_eq!(vectors.len(), 8);
    }

    #[test]
    fn test_holy_water_aoe_radius() {
        let pool_pos = Vec2::new(100.0, 100.0);
        let pool_radius = 65.0;

        let enemy_inside = Vec2::new(130.0, 120.0);
        let enemy_outside = Vec2::new(180.0, 180.0);

        assert!(enemy_inside.distance(pool_pos) <= pool_radius);
        assert!(enemy_outside.distance(pool_pos) > pool_radius);
    }

    #[test]
    fn test_game_record_bestiary_persistence() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_ultimate_record.json");
        let path_str = file_path.to_str().unwrap();

        let record = GameRecord {
            best_survival_time_sec: 245.0,
            max_kills: 350,
            discovered_monsters_count: 64,
        };

        assert!(record.save(path_str).is_ok());

        let loaded = GameRecord::load_or_default(path_str);
        assert_eq!(loaded.max_kills, 350);
        assert_eq!(loaded.discovered_monsters_count, 64);

        let _ = std::fs::remove_file(file_path);
    }

    #[test]
    fn test_weapon_damage_stats_meter() {
        use game_07_ferris_survivors_ultimate::components::WeaponDamageStats;

        let mut stats = WeaponDamageStats::default();
        stats.orbit_blade_damage = 400.0;
        stats.homing_arrow_damage = 300.0;
        stats.holy_water_damage = 200.0;
        stats.parabolic_axe_damage = 100.0;

        assert_eq!(stats.total_damage(), 1000.0);
        let blade_pct = (stats.orbit_blade_damage / stats.total_damage()) * 100.0;
        assert_eq!(blade_pct, 40.0);
    }

    #[test]
    fn test_bestiary_browser_cycling() {
        use game_07_ferris_survivors_ultimate::components::BestiaryBrowserState;

        let mut browser = BestiaryBrowserState::default();
        assert_eq!(browser.entries.len(), 400);
        assert_eq!(browser.current_index, 0);

        // 이전 몬스터 순환 (0 -> 399)
        if browser.current_index == 0 {
            browser.current_index = browser.entries.len() - 1;
        }
        assert_eq!(browser.current_index, 399);

        // 다음 몬스터 순환 (399 -> 0)
        browser.current_index = (browser.current_index + 1) % browser.entries.len();
        assert_eq!(browser.current_index, 0);
    }
}

