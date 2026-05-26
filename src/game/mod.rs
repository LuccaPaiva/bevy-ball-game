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

use bevy::prelude::*;

use crate::AppState;
use crate::events::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Events
        app.add_message::<GameOver>();
        // States
        app.init_state::<SimulationState>();
        // OnEnter Systems
        app.add_systems(OnEnter(AppState::Game), pause_simulation);
        // My Plugins
        app.add_plugins(EnemyPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(ScorePlugin);
        app.add_plugins(StarPlugin);
        // Systems
        app.add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
        // Exit State Systems
        //app.add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
