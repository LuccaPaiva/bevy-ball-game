use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{AppState, events::*, game::SimulationState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d::default(),
        Camera::default(),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
        println!("Game Over");
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: MessageReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for message in game_over_event_reader.read() {
        next_app_state.set(AppState::GameOver);
        println!("App in AppState::GameOver");
        println!("Your final score is: {}", message.score.to_string());
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            app_state_next_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut sim_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            app_state_next_state.set(AppState::MainMenu);
            sim_state.set(SimulationState::Paused);
            println!("Entered AppState::MainMenu");
        }
    }
}
