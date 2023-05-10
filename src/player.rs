use bevy::prelude::*;

#[derive(Component)]
struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3 {
                x: -250.0,
                y: 0.0,
                z: 0.0,
            }),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2 { x: 50.0, y: 50.0 }),
                ..Default::default()
            },
            ..Default::default()
        },
        Player,
    ));
}
