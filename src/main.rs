#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::render::camera::WindowOrigin;
use bevy::sprite::collide_aabb::collide;

pub const BG_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

pub const WINDOW_W: f32 = 800.0;
pub const WINDOW_H: f32 = 600.0;
pub const WINDOW_W2: f32 = WINDOW_W / 2.0;
pub const WINDOW_H2: f32 = WINDOW_H / 2.0;

pub const INFO_BAR_H: f32 = 100.0;

pub const GRAVITY: Vec2 = Vec2::new(0.0, -60.0);

mod audio;
use audio::GameAudioPlugin;

mod mainmenu;
use mainmenu::MainMenuPlugin;

mod player;
use player::PlayerPlugin;

mod maze;
use maze::MazePlugin;

mod enemies;
use enemies::EnemiesPlugin;

mod infobar;
use infobar::InfoBarPlugin;

mod explosions;
use explosions::ExplosionsPlugin;

mod items;
use items::ItemsPlugin;

mod special;
use special::SpecialPlugin;

mod pause;
use pause::PausePlugin;

mod death;
use death::DeathScreenPlugin;

mod gameover;
use gameover::GameOverPlugin;

mod victory;
use victory::VictoryPlugin;

mod redefinekeys;
use redefinekeys::RedefineKeysPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Start,
    Menu,
    Game,
    Pause,
    Death,
    GameOver,
    Victory,
    RedefineKeys,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameDirection {
    Left,
    Right,
    None
}

pub struct GameKeys {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub fire: KeyCode,
    pub pause: KeyCode,
}

#[derive(Component)]
pub struct StartScreenImage;

fn main() {
    let window = WindowDescriptor {
        width: WINDOW_W,
        height: WINDOW_H,
        position: WindowPosition::At(Vec2::new(150.0,50.0)),
        title: String::from("RTH Jet-Story"),
        present_mode: PresentMode::Immediate,
        resizable: false,
        ..Default::default()
    };

    App::new()
    .add_state(GameState::Start)
    .insert_resource(ClearColor(BG_COLOR))
    .insert_resource(window)
    .insert_resource(GameKeys {
        up: KeyCode::Up,
        down: KeyCode::Down,
        left: KeyCode::Left,
        right: KeyCode::Right,
        fire: KeyCode::A,
        pause: KeyCode::P,
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(GameAudioPlugin)
    .add_plugin(MainMenuPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(MazePlugin)
    .add_plugin(EnemiesPlugin)
    .add_plugin(InfoBarPlugin)
    .add_plugin(ExplosionsPlugin)
    .add_plugin(ItemsPlugin)
    .add_plugin(SpecialPlugin)
    .add_plugin(PausePlugin)
    .add_plugin(DeathScreenPlugin)
    .add_plugin(GameOverPlugin)
    .add_plugin(VictoryPlugin)
    .add_plugin(RedefineKeysPlugin)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_start_screen)
    .add_system_set(SystemSet::on_exit(GameState::Start).with_system(despawn_start_screen))
    // .add_startup_system(spawn_axes)
    .add_system(main_keyboard_input)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    }); 
}

pub fn collision_check(target_pos: Vec3, target_size: Vec2, tile_pos: Vec3, tile_size: Vec2) -> bool {
    let collision = collide(
        target_pos, 
        target_size, 
        tile_pos, 
        tile_size
    );
    collision.is_some()
}

pub const H_PADDING: Vec2 = Vec2::new(2.0, 0.0);
pub const V_PADDING: Vec2 = Vec2::new(0.0, 2.0);

fn load_atlas(
    assets: &Res<AssetServer>, 
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    file_name: &str,
    tile_size: Vec2,
    rows: usize,
    cols: usize,
    padding: Option<Vec2>,) -> Handle<TextureAtlas>
{
    let image: Handle<Image> = assets.load(file_name);

    match padding {
        Some(padding) => {
            let atlas = TextureAtlas::from_grid_with_padding(image, tile_size, cols, rows, padding, Vec2::splat(0.0));
            return texture_atlases.add(atlas);
        },
        None => {
            let atlas = TextureAtlas::from_grid(image, tile_size, cols, rows);
            return texture_atlases.add(atlas);
        }
    }
}

fn main_keyboard_input(mut keyboard: ResMut<Input<KeyCode>>, game_input: Res<GameKeys>, mut state: ResMut<State<GameState>>) {
    if state.current() == &GameState::Start && keyboard.get_just_pressed().len() > 0 {
        keyboard.clear();
        state.set(GameState::Menu).expect("main: Failed to change state!");
    }

    if keyboard.just_pressed(game_input.pause) {
        if state.current() == &GameState::Game {
            state.push(GameState::Pause).expect("main: Failed to push PAUSE state!");
        }
        else if state.current() == &GameState::Pause {
            state.pop().expect("main: Failed to po PAUSE state!");
        }
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        if state.current() == &GameState::Game {
            keyboard.clear();
            state.set(GameState::Menu).expect("main: Failed to change state!");
        }
    }
}

fn spawn_start_screen(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("images/jet-story_loadscr.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(WINDOW_W, WINDOW_H)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(WINDOW_W2, WINDOW_H2, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(StartScreenImage);
}

fn despawn_start_screen(mut commands: Commands, query: Query<Entity, With<StartScreenImage>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// fn spawn_axes(mut commands: Commands, assets: Res<AssetServer>) {
//     commands.spawn_bundle(SpriteBundle {
//         texture: assets.load("images/xy_800x600.png"),
//         sprite: Sprite { 
//             // color: Color::rgba(0.0, 0.0, 0.0, 0.7),
//             custom_size: Some(Vec2::new(WINDOW_W, WINDOW_H)),
//             flip_x: false,
//             ..Default::default()
//         },
//         transform: Transform {
//             translation: Vec3::new(WINDOW_W2, WINDOW_H2, 999.0),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// }
