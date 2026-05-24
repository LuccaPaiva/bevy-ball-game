use bevy::prelude::*;

pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenu;

use systems::*;

fn main() {
    App::new()
        //bevy plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppStates>()
        //.insert_state(AppStates::MainMenu)
        //my plugins
        .add_plugins(MainMenu)
        .add_plugins(GamePlugin)
        // Startup plugins
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Update, transition_to_game_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppStates {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
