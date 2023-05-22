use bevy::prelude::*;

use crate::GameReset;

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
            .add_system(toggle_state)
            .add_system(update_text)
            .add_startup_system(spawn_game_state_info);
    }
}
fn toggle_state(
    mut commands: Commands,
    state: Res<State<GameState>>,
    mut ev_writer: EventWriter<GameReset>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::P) && !keyboard_input.just_pressed(KeyCode::Escape) {
        return;
    }
    commands.insert_resource(NextState(Some(match state.0 {
        GameState::Running => GameState::Pause,
        GameState::Pause => GameState::Running,
        GameState::GameOver => {
            ev_writer.send(GameReset {});
            GameState::Running
        }
    })))
}

#[derive(Component)]
struct StateInfoTop;
#[derive(Component)]
struct StateInfoBottom;

fn spawn_game_state_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Silkscreen-Regular.ttf");
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLUE),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Percent(50.0),
                    left: Val::Percent(50.0),
                    ..Default::default()
                },
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 130.0,
                        color: Color::BLACK,
                        font: font.clone(),
                        ..Default::default()
                    },
                )
                .with_text_alignment(TextAlignment::Center),
                StateInfoTop,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::BLACK,
                        font,
                        ..Default::default()
                    },
                ),
                StateInfoBottom,
            ));
        });
}

fn update_text(
    mut query_top: Query<&mut Text, (With<StateInfoTop>, Without<StateInfoBottom>)>,
    mut query_bottom: Query<&mut Text, (With<StateInfoBottom>, Without<StateInfoTop>)>,
    state: Res<State<GameState>>,
) {
    let (top, bottom) = match state.0 {
        GameState::Running => ("", ""),
        GameState::Pause => ("Paused", "Press Escape to resume"),
        GameState::GameOver => ("Game Over", "Press Escape to restart"),
    };
    query_top.get_single_mut().unwrap().sections[0].value = top.to_string();
    query_bottom.get_single_mut().unwrap().sections[0].value = bottom.to_string();
}
