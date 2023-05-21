use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Pause,
    GameOver,
}

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_system(toggle_state.run_if(not(in_state(GameState::GameOver))))
            .add_system(update_text)
            .add_startup_system(spawn_game_state_info);
    }
}
fn toggle_state(
    mut commands: Commands,
    state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::P) && !keyboard_input.just_pressed(KeyCode::Escape) {
        return;
    }
    commands.insert_resource(NextState(Some(match state.0 {
        GameState::Running => GameState::Pause,
        _ => GameState::Running,
    })))
}

#[derive(Component)]
struct StateInfo;

fn spawn_game_state_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Test",
            TextStyle {
                color: Color::BLACK,
                font_size: 100.0,
                font: asset_server.load("Silkscreen-Regular.ttf"),
                ..Default::default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }),
        StateInfo,
    ));
}

fn update_text(mut query: Query<&mut Text, With<StateInfo>>, state: Res<State<GameState>>) {
    let new_string = match state.0 {
        GameState::Running => "",
        GameState::Pause => "Paused",
        GameState::GameOver => "Game Over",
    };
    query.get_single_mut().unwrap().sections[0].value = new_string.to_string();
}
