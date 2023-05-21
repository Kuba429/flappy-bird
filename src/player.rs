use bevy::prelude::*;

use crate::{obstacle::Obstacle, GameState};

#[derive(Component)]
pub struct Player {
    pub velocity: f32,
}

#[derive(Resource)]
pub struct PlayerRotation(f32);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerRotation { 0: 0.0 })
            .add_startup_system(spawn_player)
            .add_systems((
                handle_jump.run_if(in_state(GameState::Running)),
                fall.run_if(in_state(GameState::Running)),
                keep_on_screen.run_if(in_state(GameState::Running)),
                check_for_collision.run_if(in_state(GameState::Running)),
            ));
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("bird.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3 {
                x: -250.0,
                y: 0.0,
                z: 0.0,
            }),
            texture: texture_handle,
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: 17.0 * 3.0,
                    y: 12.0 * 3.0,
                }),
                ..Default::default()
            },
            ..Default::default()
        },
        Player { velocity: 0.0 },
    ));
}
// map function inspired by arduino
fn map<T>(x: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Copy,
{
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

pub fn fall(
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
    mut rotation: ResMut<PlayerRotation>,
) {
    let (mut transform, mut player) = query.get_single_mut().unwrap();
    player.velocity -= time.delta_seconds() * 10.0;
    if player.velocity < -5.0 {
        player.velocity = -5.0;
    }
    transform.translation.y += player.velocity;
    transform.rotate_z(-rotation.0);
    let degrees = map(player.velocity, -5.0, 5.0, -40.0, 40.0);
    rotation.0 = (degrees).to_radians();
    transform.rotate_z(rotation.0);
}

pub fn handle_jump(
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut rotation: ResMut<PlayerRotation>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Up) {
        let (mut transform, mut player) = query.get_single_mut().unwrap();
        player.velocity = 5.0;
        transform.translation.y += player.velocity;
        transform.rotate_z(-rotation.0);
        rotation.0 = 90.0_f32.to_radians();
        transform.rotate_z(rotation.0);
    }
}

pub fn keep_on_screen(mut query: Query<(&mut Transform, &Sprite), With<Player>>) {
    let (mut transform, sprite) = query.get_single_mut().unwrap();
    let sprite_size = sprite.custom_size.unwrap();

    let top_bound = crate::WINDOW_HEIGHT / 2.0 - (sprite_size.y / 2.0);
    let bottom_bound = crate::WINDOW_HEIGHT / -2.0 + (sprite_size.y / 2.0);

    if transform.translation.y > top_bound {
        transform.translation.y = top_bound;
    }
    if transform.translation.y < bottom_bound {
        transform.translation.y = bottom_bound;
    }
}

pub fn check_for_collision(
    mut commands: Commands,
    player_query: Query<(&Transform, &Sprite), With<Player>>,
    obstacle_query: Query<(&Transform, &Sprite), With<Obstacle>>,
) {
    let (player_transform, player_sprite) = player_query.get_single().unwrap();
    let player_size = player_sprite.custom_size.unwrap();
    let player_translation = player_transform.translation;
    obstacle_query
        .iter()
        .for_each(|(obstacle_transform, obstacle_sprite)| {
            let obstacle_translation = obstacle_transform.translation;
            let obstacle_size = obstacle_sprite.custom_size.unwrap();

            if !(player_translation.x + (player_size.x / 2.0)
                > obstacle_translation.x - (obstacle_size.x / 2.0)
                && player_translation.x - (player_size.x / 2.0)
                    < obstacle_translation.x + (obstacle_size.x / 2.0))
            {
                // not collision on x axis
                return;
            }
            if !(player_translation.y + (player_size.y / 2.0)
                >= obstacle_translation.y - (obstacle_size.y / 2.0)
                && player_translation.y - (player_size.y / 2.0)
                    <= obstacle_translation.y + (obstacle_size.y / 2.0))
            {
                // no collision on y axis
                return;
            }
            // collision
            commands.insert_resource(NextState(Some(GameState::GameOver)))
        })
}
