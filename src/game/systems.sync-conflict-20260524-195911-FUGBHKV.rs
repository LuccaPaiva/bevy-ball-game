use bevy::prelude::*;

use crate::game::SimulationState;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    println!("Simulation Paused on Game Entered!");
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    println!("Simulation running!");
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    println!("Toggling state from");
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Running => {
                next_state.set(SimulationState::Paused);
                println!("Simulation Paused");
            }

            SimulationState::Paused => {
                next_state.set(SimulationState::Running);
                println!("Simulation Running");
            }
        }
    }
}
