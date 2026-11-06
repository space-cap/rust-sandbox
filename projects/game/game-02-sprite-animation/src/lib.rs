pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::GameStats;
use systems::{
    animation::animate_sprite_system,
    diagnostics::diagnostics_system,
    input::player_input_system,
    setup::setup_game,
};

/// 2D 스프라이트 애니메이션 플러그인
pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameStats>()
            .add_systems(Startup, setup_game)
            .add_systems(
                Update,
                (player_input_system, animate_sprite_system, diagnostics_system),
            );
    }
}

