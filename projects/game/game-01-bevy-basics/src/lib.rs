pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::GameStats;
use systems::{diagnostics::diagnostics_system, movement::player_movement_system, setup::setup_game};

/// Bevy Basics 플러그인 정의
pub struct GameBasicsPlugin;

impl Plugin for GameBasicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameStats>()
            .add_systems(Startup, setup_game)
            .add_systems(Update, (player_movement_system, diagnostics_system));
    }
}

