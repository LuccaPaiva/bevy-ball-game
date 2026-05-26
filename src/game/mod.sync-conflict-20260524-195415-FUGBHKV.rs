use bevy::prelude::App;
use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::{AppStates, events::*};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>();
        app.add_systems(OnEnter(AppStates::Game), pause_simulation);
        //app.add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin));
        app.add_plugins(EnemyPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(ScorePlugin);
        app.add_plugins(StarPlugin);
        app.add_message::<GameOver>();
        app.add_systems(Update, toggle_simulation.run_if(in_state(AppStates::Game)));
        //app.add_systems(OnExit(AppStates::Game), resume_simulation);
        //app.add_systems(OnEnter(SimulationState::Running), resume_simulation);
        //app.add_systems(OnEnter(SimulationState::Paused), pause_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]

pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
