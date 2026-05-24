use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{AppStates, events::*, game::SimulationState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d::default(),
        Camera::default(),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppStates>>,
    mut next_state: ResMut<NextState<AppStates>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppStates::Game {
            next_state.set(AppStates::Game);
            println!("Enter App state Game!");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppStates>>,
    mut next_state: ResMut<NextState<AppStates>>,
    mut next_state_sim: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppStates::MainMenu {
            next_state.set(AppStates::MainMenu);
            next_state_sim.set(SimulationState::Paused);
            println!("Enter App state Main Menu!");
        }
    }
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
    mut next_state: ResMut<NextState<AppStates>>,
) {
    for message in game_over_event_reader.read() {
        println!("Your final score is: {}", message.score.to_string());
    }
    next_state.set(AppStates::GameOver);
}
