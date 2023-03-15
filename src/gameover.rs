use bevy::prelude::*;

use crate::{GameState, WINDOW_W2, WINDOW_H2, mainmenu::UiAssets, maze::Maze};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::GameOver)
            .with_system(spawn_game_over)
        )
        .add_system_set(SystemSet::on_update(GameState::GameOver)
            .with_system(game_over_keyboard_input)
        )
        .add_system_set(SystemSet::on_exit(GameState::GameOver)
            .with_system(despawn_game_over)
        );
    }
}

#[derive(Component)]
pub struct GameOverText;

fn spawn_game_over(mut commands: Commands, ui_assets: Res<UiAssets>, mut maze: ResMut<Maze>) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "Game Over!", 
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 40.0,
                color: Color::RED,
            }).with_alignment(TextAlignment::CENTER),
        transform: Transform::from_xyz(WINDOW_W2, WINDOW_H2, 0.0),
        ..Default::default()
    }).insert(GameOverText);

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
    }).insert(GameOverText);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            format!("Bases: {:0>2} / {:0>2}", maze.bases, maze.bases_total), 
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 20.0,
                color: Color::RED,
            }).with_alignment(TextAlignment::CENTER),
        transform: Transform::from_xyz(WINDOW_W2, WINDOW_H2 - 90.0, 0.0),
        ..Default::default()
    }).insert(GameOverText);

    maze.loaded = false;
}

fn despawn_game_over(mut commands: Commands, text_query: Query<Entity, With<GameOverText>>) {
    for entity in text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn game_over_keyboard_input(mut keyboard: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.get_just_pressed().len() != 0 {
        keyboard.clear();
        state.set(GameState::Menu).expect("GameOver: Failed to change state!");
    }
}
