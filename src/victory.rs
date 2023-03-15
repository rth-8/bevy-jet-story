use bevy::prelude::*;

use crate::{GameState, WINDOW_W2, WINDOW_H2, mainmenu::UiAssets, maze::Maze};

pub struct VictoryPlugin;

impl Plugin for VictoryPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Victory)
            .with_system(spawn_victory)
        )
        .add_system_set(SystemSet::on_update(GameState::Victory)
            .with_system(victory_keyboard_input)
        )
        .add_system_set(SystemSet::on_exit(GameState::Victory)
            .with_system(despawn_victory)
        );
    }
}

#[derive(Component)]
pub struct VictoryText;

fn spawn_victory(mut commands: Commands, ui_assets: Res<UiAssets>, mut maze: ResMut<Maze>) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "Mission accomplished!", TextStyle {
                font: ui_assets.font.clone(),
                font_size: 40.0,
                color: Color::RED,
            }).with_alignment(TextAlignment::CENTER),
        transform: Transform::from_xyz(WINDOW_W2, WINDOW_H2, 0.0),
        ..Default::default()
    }).insert(VictoryText);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            format!("Score: {:0>7}", maze.score), 
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 20.0,
                color: Color::RED,
            }).with_alignment(TextAlignment::CENTER),
        transform: Transform::from_xyz(WINDOW_W2, WINDOW_H2 - 60.0, 0.0),
        ..Default::default()
    }).insert(VictoryText);

    maze.loaded = false;
}

fn despawn_victory(mut commands: Commands, text_query: Query<Entity, With<VictoryText>>) {
    for entity in text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn victory_keyboard_input(mut keyboard: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.get_just_pressed().len() != 0 {
        keyboard.clear();
        state.set(GameState::Menu).expect("Victory: Failed to change state!");
    }
}
