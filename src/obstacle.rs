use bevy::prelude::*;

#[derive(Component)]
pub struct Obstacle(pub f32);

#[derive(Resource)]
pub struct ObstacleCount(pub u8);

pub fn spawn_obstacle(mut commands: Commands, mut obstacle_count_res: ResMut<ObstacleCount>) {
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
        Obstacle(0.0),
    ));
    obstacle_count_res.0 += 1;
}

pub fn move_obstacles(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Obstacle), With<Obstacle>>,
    time: Res<Time>,
) {
    query.for_each_mut(|(mut transform, obs)| {
        transform.translation.x -= 100.0 * time.delta_seconds();
        if transform.translation.x < crate::WINDOW_WIDTH / -2.0 {
            todo!("Despawn the obstacle");
        };
    });
}

pub fn regulate_obstacle_count(
    commands: Commands,
    _query: Query<&Transform, With<Obstacle>>,
    mut obstacle_count_res: ResMut<ObstacleCount>,
) {
    let target_obstacle_count = 1;
    let obstacle_count = obstacle_count_res.as_mut();

    if obstacle_count.0 < target_obstacle_count {
        spawn_obstacle(commands, obstacle_count_res);
    }
}
