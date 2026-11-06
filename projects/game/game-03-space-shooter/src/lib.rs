pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::{EnemySpawnTimer, GameState};
use systems::{
    bullet::bullet_movement_system,
    collision::collision_detection_system,
    enemy::{enemy_movement_system, enemy_spawn_system},
    hud::hud_update_system,
    player::player_control_system,
    setup::setup_game,
    starfield::starfield_system,
};

/// 우주 슈팅 게임 플러그인
pub struct SpaceShooterPlugin;

impl Plugin for SpaceShooterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, setup_game)
            .add_systems(
                Update,
                (
                    player_control_system,
                    bullet_movement_system,
                    enemy_spawn_system,
                    enemy_movement_system,
                    collision_detection_system,
                    starfield_system,
                    hud_update_system,
                ),
            );
    }
}

