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
            .add_system(toggle_state.run_if(not(in_state(GameState::GameOver))));
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
