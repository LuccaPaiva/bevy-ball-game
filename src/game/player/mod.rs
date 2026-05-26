use bevy::prelude::App;
use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;
use crate::game::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player);
        app.add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                enemy_hit_player,
                player_hit_star,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
        /*
        app.add_systems(Update, player_movement);
        app.add_systems(Update, confine_player_movement.after(player_movement));
        app.add_systems(Update, enemy_hit_player);
        app.add_systems(Update, player_hit_star);
        */
    }
}
