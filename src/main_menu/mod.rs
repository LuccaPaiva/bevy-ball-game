pub mod components;
pub mod styles;
pub mod systems;

use bevy::prelude::App;
use bevy::prelude::*;
use systems::interactions::*;
use systems::layout::*;

use crate::AppState;

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        // Systems
        app.add_systems(
            Update,
            (interact_with_play_button, interact_with_quit_button)
                .run_if(in_state(AppState::MainMenu)),
        );
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
