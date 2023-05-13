use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub falling_force: f32,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_systems((handle_jump, fall, keep_on_screen));
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
        Player { falling_force: 0.0 },
    ));
}

pub fn fall(mut query: Query<(&mut Transform, &mut Player), With<Player>>, time: Res<Time>) {
    let (mut transform, mut player) = query.get_single_mut().unwrap();
    player.falling_force -= time.delta_seconds() * 10.0;
    transform.translation.y += player.falling_force;
}

pub fn handle_jump(
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Up) {
        let (mut transform, mut player) = query.get_single_mut().unwrap();
        player.falling_force = 5.0;
        transform.translation.y += player.falling_force;
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
