use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::{AppStates, game::SimulationState};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        //app.init_resource::<Score>();
        app.init_resource::<HighScores>();
        app.add_systems(OnEnter(AppStates::Game), insert_score);
        app.add_systems(OnExit(AppStates::Game), remove_score);
        /*
        app.add_systems(Update, update_score);
        app.add_systems(Update, update_high_scores);
        app.add_systems(Update, high_scores_updated); */
        app.add_systems(
            Update,
            (update_score, update_high_scores, high_scores_updated)
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
