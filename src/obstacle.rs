use bevy::prelude::*;

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource)]
pub struct LastObstacleDistance(pub f32);

pub fn spawn_obstacle(mut commands: Commands, mut last_obstacle_res: ResMut<LastObstacleDistance>) {
    let frequency = 2.0;
    if last_obstacle_res.0 < crate::WINDOW_WIDTH / frequency {
        return;
    };
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3 {
                x: crate::WINDOW_WIDTH / 2.0 + 50.0,
                y: 0.0,
                z: 0.0,
            }),
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2 { x: 50.0, y: 50.0 }),
                ..Default::default()
            },
            ..Default::default()
        },
        Obstacle,
    ));
    last_obstacle_res.0 = 0.0;
}

pub fn move_obstacles(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Sprite, Entity), With<Obstacle>>,
    mut last_obstacle_res: ResMut<LastObstacleDistance>,
    time: Res<Time>,
) {
    let distance = 100.0 * time.delta_seconds();
    last_obstacle_res.0 += distance;
    query.for_each_mut(|(mut transform, sprite, entity)| {
        let sprite_width = sprite.custom_size.unwrap().x;
        transform.translation.x -= distance;
        if transform.translation.x < crate::WINDOW_WIDTH / -2.0 - sprite_width {
            commands.entity(entity).despawn();
        };
    });
}
