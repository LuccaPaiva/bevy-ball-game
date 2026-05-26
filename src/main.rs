use bevy::prelude::*;

pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenu;

use events::*;
use systems::*;

fn main() {
    App::new()
        //Bevy plugins
        .add_plugins(DefaultPlugins)
        // States
        .init_state::<AppState>()
        // My plugins
        .add_plugins(GamePlugin)
        .add_plugins(MainMenu)
        //Messages
        .add_message::<GameOver>()
        // Startup systems
        .add_systems(Startup, spawn_camera)
        //Systems
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
