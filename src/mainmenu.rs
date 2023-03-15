use bevy::{prelude::*, ui::FocusPolicy, app::AppExit};

use crate::{GameState, maze::Maze, player::Player};

pub struct MainMenuPlugin;

pub struct UiAssets {
    pub font: Handle<Font>,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(load_menu_resources)
        .add_system_set(SystemSet::on_enter(GameState::Menu)
            .with_system(spawn_main_menu)
        )
        .add_system_set(SystemSet::on_exit(GameState::Menu)
            .with_system(despawn_main_menu)
        )
        .add_system_set(SystemSet::on_pause(GameState::Menu)
            .with_system(despawn_main_menu)
        )
        .add_system_set(SystemSet::on_resume(GameState::Menu)
            .with_system(spawn_main_menu)
        )
        .add_system_set(SystemSet::on_update(GameState::Menu)
            .with_system(handle_menu_buttons)
            .with_system(keyboard_input)
        );
    }
}

pub fn load_menu_resources(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = UiAssets {
        font: assets.load("esp.ttf"),
    };
    commands.insert_resource(ui_assets);
}

pub fn spawn_button(commands: &mut Commands, ui_assets: &Res<UiAssets>, name: &str, text: &str) -> Entity {
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(50.0), Val::Percent(20.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::DARK_GRAY.into(),
        ..Default::default()
    })
    .insert(Name::from(name))
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::from_section(
                text, 
                TextStyle { 
                    font: ui_assets.font.clone(), 
                    font_size: 30.0, 
                    color: Color::rgb(0.9, 0.9, 0.9) 
                }
            ).with_alignment(TextAlignment::CENTER),
            focus_policy: FocusPolicy::Pass,
            ..Default::default()
        });
    })
    .id()
}

pub fn spawn_main_menu(mut commands: Commands, ui_assets: Res<UiAssets>, maze: Res<Maze>) {
    let panel = commands.spawn_bundle(NodeBundle{
        style: Style { 
            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
            flex_direction: FlexDirection::ColumnReverse,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    }).id();

    let mut items = Vec::new();

    if maze.loaded {
        items.push(spawn_button(&mut commands, &ui_assets, "ResumeButton", "Resume"));
    }
    items.push(spawn_button(&mut commands, &ui_assets, "StartButton", "New Game"));
    items.push(spawn_button(&mut commands, &ui_assets, "RedefineButton", "Redefine Keys"));
    items.push(spawn_button(&mut commands, &ui_assets, "ExitButton", "Exit"));

    commands.entity(panel).push_children(&items);
}

fn despawn_main_menu(mut commands: Commands, button_query: Query<Entity, With<Node>>) {
    for entity in button_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_menu_buttons(
    // mut commands: Commands,
    mut interaction_query: Query<(&Name, &Interaction, &mut UiColor), Changed<Interaction>>,
    mut state: ResMut<State<GameState>>,
    mut mouse: ResMut<Input<MouseButton>>,
    mut player: ResMut<Player>,
    mut maze: ResMut<Maze>,
    mut exit: EventWriter<AppExit>)
{
    for (name, interaction, mut color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                println!("Button clicked: {}", name.as_str());
                match name.as_str() {
                    "StartButton" => {
                        player.clear();
                        maze.clear();
                        mouse.clear();
                        state.set(GameState::Game).expect("MAIN MENU: Failed to change state!");
                    },
                    "ResumeButton" => {
                        mouse.clear();
                        if maze.loaded {
                            state.set(GameState::Game).expect("MAIN MENU: Failed to change state!");
                        }
                    },
                    "RedefineButton" => {
                        mouse.clear();
                        // state.set(GameState::RedefineKeys).expect("MAIN MENU: Failed to change state!");
                        state.push(GameState::RedefineKeys).expect("MAIN MENU: Failed to push state!");
                    },
                    "ExitButton" => {
                        exit.send(AppExit); //exit
                    },
                    _ => panic!("MAIN MENU: Unexpected button!")
                }
            },
            Interaction::Hovered => { 
                *color = UiColor(Color::GRAY);
            },
            Interaction::None => {
                *color = Color::DARK_GRAY.into();
            }
        }
    }
}

fn keyboard_input(
    keyboard: Res<Input<KeyCode>>, 
    state: ResMut<State<GameState>>, 
    mut exit: EventWriter<AppExit>) 
{
    if keyboard.just_pressed(KeyCode::Escape) && state.current() == &GameState::Menu {
        exit.send(AppExit); //exit
    }
}
