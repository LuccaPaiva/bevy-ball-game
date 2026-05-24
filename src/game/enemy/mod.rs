use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.

use resources::*;
use systems::*;

use crate::{AppStates, game::SimulationState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>();
        //app.add_systems(Startup, spawn_enemies);
        // //Only when we enter game
        app.add_systems(OnEnter(AppStates::Game), spawn_enemies); //app.add_systems(Update, move_direct_confine_enemies);
        //app.add_systems(Update, enemy_movement);
        //app.add_systems(Update, update_enemy_direction.after(enemy_movement));
        //app.add_systems(Update, confine_enemy_movement.after(update_enemy_direction));
        //app.add_systems(Update, tick_enemy_spawn_timer);
        //app.add_systems(Update, spawn_enemies_over_time);
        app.add_systems(
            Update,
            (
                move_direct_confine_enemies,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time,
            )
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(SimulationState::Running)),
        );
        //Exit
        app.add_systems(OnExit(AppStates::Game), despawn_enemies);
    }
}
