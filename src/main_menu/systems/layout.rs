use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Creating main menu!");
    build_main_menu(&mut commands, &asset_server);
    println!("Created main menu!");
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &AssetServer) -> Entity {
    let main_menu_entity = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                row_gap: Val::Px(8.0),
                column_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.9, 0.0, 0.0)),
            MainMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(300.0),
                        height: Val::Px(120.0),
                        ..Node::DEFAULT
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|parent| {
                    // Image 1
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/ball_blue_large.png")),
                        Node {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                    ));

                    // Text
                    parent.spawn((
                        Text::new("Bevy Ball Game"),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        TextShadow::default(),
                    ));

                    // Image 2
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                        Node {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect::all(Val::Px(8.0)),
                            ..Node::DEFAULT
                        },
                    ));
                });

            // === Play Button ===
            parent
                .spawn((
                    Button,
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..Node::DEFAULT
                    },
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        TextShadow::default(),
                    ));
                });

            // === Quit Button ===
            parent
                .spawn((
                    Button,
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..Node::DEFAULT
                    },
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        TextShadow::default(),
                    ));
                });
        })
        .id();

    main_menu_entity
}
