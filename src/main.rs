mod player;
use bevy::{
    prelude::*,
    window::{WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use player::{fall, handle_jump, keep_on_screen, spawn_player};

pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WINDOW_WIDTH: f32 = 800.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(bevy::window::Window {
                title: "Flappy Bird".to_string(),
                resizable: false,
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(handle_jump)
        .add_system(fall)
        .add_system(keep_on_screen)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
