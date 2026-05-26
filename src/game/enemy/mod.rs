use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.

use resources::*;
use systems::*;

use crate::{AppState, game::SimulationState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>();
        app.add_systems(OnEnter(AppState::Game), spawn_enemies);

        app.add_systems(
            Update,
            (
                move_direct_confine_enemies,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
