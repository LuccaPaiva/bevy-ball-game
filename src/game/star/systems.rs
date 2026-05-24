use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use crate::AppStates;

use super::components::*;
use super::resources::*;

use super::{NUMBER_OF_STARS, STAR_SIZE};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let mut random_x = random::<f32>() * window.width();
        let mut random_y = random::<f32>() * window.height();

        let half_star_size = STAR_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_star_size;
        let x_max = window.width() - half_star_size;
        let y_min = 0.0 + half_star_size;
        let y_max = window.height() - half_star_size;

        random_x = random_x.clamp(x_min, x_max);
        random_y = random_y.clamp(y_min, y_max);

        commands.spawn((
            DespawnOnExit(AppStates::Game),
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Star {},
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spin_stars(mut star_query: Query<&mut Transform, With<Star>>, time: Res<Time>) {
    for mut star_tranform in star_query.iter_mut() {
        star_tranform.rotate_z(time.delta_secs());
    }
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.just_finished() {
        let window = window_query.single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            DespawnOnExit(AppStates::Game),
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            /*
            Transform::from_xyz(
                random_x.clamp(
                    half_star_size / window.width(),
                    1. - half_star_size / window.width(),
                ),
                random_y.clamp(
                    half_star_size / window.height(),
                    1. - half_star_size / window.height(),
                ),
                0.0,
            ),
            */
            Transform::from_xyz(random_x, random_y, 0.0),
            Star {},
        ));
    }
}
