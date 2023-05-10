mod player;
use bevy::{
    prelude::*,
    window::{WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use player::spawn_player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(bevy::window::Window {
                title: "Flappy Bird".to_string(),
                resizable: false,
                resolution: WindowResolution::new(800.0, 600.0),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
