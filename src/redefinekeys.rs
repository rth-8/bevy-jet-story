use bevy::prelude::*;

use crate::{GameState, WINDOW_W2, mainmenu::UiAssets, WINDOW_H, GameKeys};

pub struct RedefineKeysPlugin;

impl Plugin for RedefineKeysPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::RedefineKeys).with_system(spawn_redefine_keys))
        .add_system_set(SystemSet::on_update(GameState::RedefineKeys).with_system(victory_keyboard_input))
        .add_system_set(SystemSet::on_exit(GameState::RedefineKeys).with_system(despawn_redefine_keys))
        ;
    }
}

#[derive(Component)]
pub struct RedefineKeysText(u8);

#[derive(Component)]
pub struct RedefineSelectedKeyText;

pub const REDEFINE_FONT_SIZE: f32 = 24.0;
pub const Y_POS: f32 = WINDOW_H - 80.0;

fn spawn_redefine_text(commands: &mut Commands, ui_assets: &Res<UiAssets>, index: u8, text: String, voffset: f32, alignment: TextAlignment) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            text, 
            TextStyle { font: ui_assets.font.clone(), font_size: REDEFINE_FONT_SIZE, color: Color::rgb(0.9, 0.9, 0.9)  }
        ).with_alignment(alignment),
        transform: Transform::from_xyz(WINDOW_W2, Y_POS + voffset, 0.0),
        ..Default::default()
    }).insert(RedefineKeysText(index));
}

fn spawn_redefine_selected_text(commands: &mut Commands, ui_assets: &Res<UiAssets>, text: String, voffset: f32) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            text, 
            TextStyle { font: ui_assets.font.clone(), font_size: REDEFINE_FONT_SIZE, color: Color::CYAN }
        ).with_alignment(TextAlignment::CENTER_LEFT),
        transform: Transform::from_xyz(WINDOW_W2, Y_POS + voffset, 0.0),
        ..Default::default()
    }).insert(RedefineSelectedKeyText);
}

fn spawn_redefine_keys(mut commands: Commands, ui_assets: Res<UiAssets>) {
    spawn_redefine_text(&mut commands, &ui_assets, 1, String::from("Press key for UP: "), 0.0, TextAlignment::CENTER_RIGHT);
}

fn despawn_redefine_keys(
    mut commands: Commands, 
    text_query: Query<Entity, With<RedefineKeysText>>,
    selection_query: Query<Entity, With<RedefineSelectedKeyText>>) 
{
    for entity in text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in selection_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn victory_keyboard_input(
    mut commands: Commands, 
    mut keyboard: ResMut<Input<KeyCode>>, 
    text_query: Query<&RedefineKeysText>, 
    ui_assets: Res<UiAssets>,
    mut game_input: ResMut<GameKeys>,
    mut state: ResMut<State<GameState>>) 
{
    if keyboard.get_just_pressed().len() == 1 {
        let key_opt = keyboard.get_just_pressed().next();
        if key_opt.is_some() {
            let key = key_opt.unwrap().clone();
            let key_name = format!("{:#?}", key);
            
            keyboard.clear();

            let cnt = text_query.iter().count();
            // println!("text_query cnt = {}", cnt);

            if cnt == 1 {
                game_input.up = key;
                for text_component in text_query.iter() {
                    if text_component.0 == 1 {
                        spawn_redefine_selected_text(&mut commands, &ui_assets, key_name.to_uppercase(), 0.0);
                    }
                }
                spawn_redefine_text(&mut commands, &ui_assets, 2, String::from("Press key for DOWN: "), 
                    -(REDEFINE_FONT_SIZE*2.0), TextAlignment::CENTER_RIGHT);
            }
            else if cnt == 2 {
                game_input.down = key;
                for text_component in text_query.iter() {
                    if text_component.0 == 2 {
                        spawn_redefine_selected_text(&mut commands, &ui_assets, key_name.to_uppercase(), -(REDEFINE_FONT_SIZE*2.0));
                    }
                }
                spawn_redefine_text(&mut commands, &ui_assets, 3, String::from("Press key for LEFT: "), 
                    -(REDEFINE_FONT_SIZE*4.0), TextAlignment::CENTER_RIGHT);
            }
            else if cnt == 3 {
                game_input.left = key;
                for text_component in text_query.iter() {
                    if text_component.0 == 3 {
                        spawn_redefine_selected_text(&mut commands, &ui_assets, key_name.to_uppercase(), -(REDEFINE_FONT_SIZE*4.0));
                    }
                }
                spawn_redefine_text(&mut commands, &ui_assets, 4, String::from("Press key for RIGHT: "), 
                    -(REDEFINE_FONT_SIZE*6.0), TextAlignment::CENTER_RIGHT);
            }
            else if cnt == 4 {
                game_input.right = key;
                for text_component in text_query.iter() {
                    if text_component.0 == 4 {
                        spawn_redefine_selected_text(&mut commands, &ui_assets, key_name.to_uppercase(), -(REDEFINE_FONT_SIZE*6.0));
                    }
                }
                spawn_redefine_text(&mut commands, &ui_assets, 5, String::from("Press key for FIRE: "), 
                    -(REDEFINE_FONT_SIZE*8.0), TextAlignment::CENTER_RIGHT);
            }
            else if cnt == 5 {
                game_input.fire = key;
                for text_component in text_query.iter() {
                    if text_component.0 == 5 {
                        spawn_redefine_selected_text(&mut commands, &ui_assets, key_name.to_uppercase(), -(REDEFINE_FONT_SIZE*8.0));
                    }
                }
                spawn_redefine_text(&mut commands, &ui_assets, 6, String::from("Press any key to return to main menu..."), 
                    -(REDEFINE_FONT_SIZE*12.0), TextAlignment::CENTER);
            }
            else {
                // state.set(GameState::Menu).expect("REDEFINE: Failed to change state!");
                state.pop().expect("REDEFINE: Failed to pop state!");
            }
        }
    }
}
