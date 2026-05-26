use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 32.0; // This is the star sprite size.

use resources::*;
use systems::*;

use crate::{AppStates, game::SimulationState};

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>();
        app.add_systems(OnEnter(AppStates::Game), spawn_stars);
        //app.add_systems(Update, tick_star_spawn_timer);
        //app.add_systems(Update, spawn_stars_over_time);
        //app.add_systems(Update, spin_stars);
        app.add_systems(
            Update,
            (tick_star_spawn_timer, spawn_stars_over_time, spin_stars)
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
