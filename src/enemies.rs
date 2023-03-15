use bevy::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use bevy_kira_audio::{AudioControl, AudioChannel};

use crate::items::{FellowItem, ITEM_H};
use crate::{WINDOW_W, WINDOW_H, INFO_BAR_H, H_PADDING, V_PADDING, GameState, GameDirection, load_atlas, collision_check};
use crate::maze::{Maze, WallComponent, AnimationTimer};
use crate::player::{Player, PLAYER_W, PLAYER_H, PlayerComponent};
use crate::explosions::ExplosionsImages;
use crate::audio::{Sounds, SfxChannel, Shooting01Channel, Shooting05Channel, Shooting06Channel, Shooting08Channel, Shooting09Channel};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_enemies_images)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_enemies_shot_images)
        .add_system_set(SystemSet::on_update(GameState::Game)
            .with_system(animate_sprite)
            .with_system(enemy_01_movement)
            .with_system(enemy_02_movement)
            .with_system(enemy_03_movement)
            .with_system(enemy_05_movement)
            .with_system(enemy_06_movement)
            .with_system(enemy_07_movement)
            .with_system(enemy_08_movement)
            .with_system(enemy_09_movement)
            .with_system(enemy_10_movement)
            .with_system(enemy_11_movement)
            .with_system(enemy_12_movement)
            .with_system(enemy_13_movement)
            .with_system(enemy_14_movement)
            .with_system(enemy_15_movement)
            .with_system(enemy_16_movement)
            .with_system(enemy_17_movement)
            .with_system(enemy_18_movement)
            .with_system(enemy_19_movement)
            .with_system(enemy_20_movement)
            .with_system(enemy_shot_movement)
        );
    }
}

pub struct EnemiesImages {
    pub enemy_00: Handle<TextureAtlas>, // base
    pub enemy_01: Handle<TextureAtlas>, // dish
    pub enemy_02: Handle<TextureAtlas>, // rod
    pub enemy_03: Handle<TextureAtlas>, // tank
    pub enemy_04: Handle<TextureAtlas>, // barrier
    pub enemy_05: Handle<TextureAtlas>, // missile launcher (up)
    pub enemy_06: Handle<TextureAtlas>, // bomb launcher (down)
    pub enemy_07: Handle<TextureAtlas>, // 
    pub enemy_08: Handle<TextureAtlas>, // missile
    pub enemy_09: Handle<TextureAtlas>, // missile launcher (horizontal)
    pub enemy_10: Handle<TextureAtlas>, // spawner
    pub enemy_11: Handle<TextureAtlas>, // 
    pub enemy_12: Handle<TextureAtlas>, // bolt drone
    pub enemy_13_left: Handle<TextureAtlas>, // shooting drone
    pub enemy_13_right: Handle<TextureAtlas>, // shooting drone
    pub enemy_14_left: Handle<TextureAtlas>, // 
    pub enemy_14_right: Handle<TextureAtlas>, // 
    pub enemy_15: Handle<TextureAtlas>, // 
    pub enemy_16: Handle<TextureAtlas>, // small ball
    pub enemy_17_left: Handle<TextureAtlas>, // 
    pub enemy_17_right: Handle<TextureAtlas>, // 
    pub enemy_18: Handle<TextureAtlas>, // big ball
    pub enemy_19: Handle<TextureAtlas>, // big hoovercraft
    pub enemy_20_v1: Handle<TextureAtlas>, // carrier
    pub enemy_20_v2: Handle<TextureAtlas>, // carrier
}

pub struct EnemiesShotImages {
    pub enemy_01_shot: Handle<TextureAtlas>,
    pub enemy_02_shot: Handle<TextureAtlas>,
    pub enemy_03_shot: Handle<TextureAtlas>,
    pub enemy_05_shot: Handle<TextureAtlas>,
    pub enemy_06_shot: Handle<TextureAtlas>,
    pub enemy_07_shot: Handle<TextureAtlas>,
    pub enemy_08_shot_left: Handle<TextureAtlas>,
    pub enemy_08_shot_right: Handle<TextureAtlas>,
    pub enemy_09_shot_left: Handle<TextureAtlas>,
    pub enemy_09_shot_right: Handle<TextureAtlas>,
    pub enemy_13_shot: Handle<TextureAtlas>,
}

pub const ENEMY_COLORS: [Color; 23] = [ 
    Color::WHITE,
    Color::AQUAMARINE,
    Color::BISQUE,
    Color::BLUE,
    Color::CRIMSON,
    Color::CYAN,
    Color::DARK_GREEN,
    Color::FUCHSIA,
    Color::GOLD,
    Color::GREEN,
    Color::OLIVE,
    Color::ORANGE,
    Color::ORANGE_RED,
    Color::PINK,
    Color::RED,
    Color::SALMON,
    Color::SEA_GREEN,
    Color::TEAL,
    Color::TOMATO,
    Color::TURQUOISE,
    Color::VIOLET,
    Color::YELLOW,
    Color::YELLOW_GREEN ];

pub struct Enemy {
    pub health: i16,
    pub enemy_seq: usize,
    pub room_seq: usize,
    pub enemy_type: usize,
    pub enemy_subtype: usize,
    pub first: bool,
    pub posx: f32,
    pub posy: f32,
    pub color: Color,
    pub velocity: Vec2,
    pub shooting_cooldown: u16,
    pub shooting_cooldown_max: u16,
    pub direction: GameDirection,
    pub is_from_10: bool,
    pub fellow_enemy: Option<FellowEnemy>,
    pub fellow_item: Option<FellowItem>,
}

pub struct FellowEnemy {
    pub health: i16,
    pub enemy_type: usize,
    pub enemy_subtype: usize,
    pub first: bool,
    pub posx: f32,
    pub posy: f32,
    pub color: Color,
    pub shooting_cooldown: u16,
    pub shooting_cooldown_max: u16,
    pub direction: GameDirection,
}

#[derive(Component)]
pub struct EnemyComponent {
    pub enemy_seq: usize,
    pub room_seq: usize,
    pub is_from_10: bool,
}

#[derive(Component)]
pub struct EnemyShotComponent {
    pub enemy_type: usize,
    pub size: Vec2,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct EnemyType01;

#[derive(Component)]
pub struct EnemyType02;

#[derive(Component)]
pub struct EnemyType03;

#[derive(Component)]
pub struct EnemyType03Fellow;

#[derive(Component)]
pub struct EnemyType05;

#[derive(Component)]
pub struct EnemyType06;

#[derive(Component)]
pub struct EnemyType07;

#[derive(Component)]
pub struct EnemyType08;

#[derive(Component)]
pub struct EnemyType09;

#[derive(Component)]
pub struct EnemyType10;

#[derive(Component)]
pub struct EnemyType10Fellow;

#[derive(Component)]
pub struct EnemyType11;

#[derive(Component)]
pub struct EnemyType12;

#[derive(Component)]
pub struct EnemyType13;

#[derive(Component)]
pub struct EnemyType14;

#[derive(Component)]
pub struct EnemyType15;

#[derive(Component)]
pub struct EnemyType16;

#[derive(Component)]
pub struct EnemyType17;

#[derive(Component)]
pub struct EnemyType18;

#[derive(Component)]
pub struct EnemyType19;

#[derive(Component)]
pub struct EnemyType20;

pub const ENEMY_NN_SIZE: Vec2 = Vec2::splat(50.0);
pub const ENEMY_07_SIZE: Vec2 = Vec2::new(50.0, 46.0);
pub const ENEMY_18_SIZE: Vec2 = Vec2::splat(100.0);
pub const ENEMY_19_SIZE: Vec2 = Vec2::new(94.0, 50.0);

pub fn load_enemies_images(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    println!("Load enemies images");
    commands.insert_resource(EnemiesImages {
        enemy_00: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_00.png", ENEMY_NN_SIZE, 1, 1, None),
        enemy_01: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_01.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_02: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_02.png", ENEMY_NN_SIZE, 1, 1, None),
        enemy_03: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_03.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_04: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_04.png", ENEMY_NN_SIZE, 1, 1, None),
        enemy_05: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_05.png", ENEMY_NN_SIZE, 1, 1, None),
        enemy_06: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_06.png", ENEMY_NN_SIZE, 1, 1, None),
        enemy_07: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_07.png", ENEMY_07_SIZE, 1, 4, Some(H_PADDING)),
        enemy_08: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_08.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_09: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_09.png", ENEMY_NN_SIZE, 1, 8, Some(H_PADDING)),
        enemy_10: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_10.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_11: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_11.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_12: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_12.png", ENEMY_NN_SIZE, 1, 4, Some(H_PADDING)),
        enemy_13_left: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_13_left.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_13_right: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_13_right.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_14_left: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_14_left.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_14_right: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_14_right.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_15: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_15.png", ENEMY_NN_SIZE, 1, 4, Some(H_PADDING)),
        enemy_16: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_16.png", ENEMY_NN_SIZE, 1, 4, Some(H_PADDING)),
        enemy_17_left: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_17_left.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_17_right: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_17_right.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_18: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_18.png", ENEMY_18_SIZE, 1, 1, Some(H_PADDING)),
        enemy_19: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_19.png", ENEMY_19_SIZE, 1, 2, Some(H_PADDING)),
        enemy_20_v1: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_20_v1.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
        enemy_20_v2: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_20_v2.png", ENEMY_NN_SIZE, 1, 2, Some(H_PADDING)),
    });
}

pub const SHOT_01_SIZE: Vec2 = ENEMY_NN_SIZE;
pub const SHOT_02_SIZE: Vec2 = Vec2::splat(18.0);
pub const SHOT_03_SIZE: Vec2 = Vec2::new(17.0, 6.0);
pub const SHOT_05_SIZE: Vec2 = Vec2::new(40.0, 50.0);
pub const SHOT_06_SIZE: Vec2 = Vec2::new(34.0, 50.0);
// pub const SHOT_07_SIZE: Vec2 = Vec2::splat(21.0);
pub const SHOT_07_SIZE: Vec2 = Vec2::new(18.0, 21.0);
pub const SHOT_08_SIZE: Vec2 = Vec2::splat(50.0);
pub const SHOT_09_SIZE: Vec2 = Vec2::new(50.0, 35.0);
pub const SHOT_13_SIZE: Vec2 = Vec2::new(17.0, 6.0);

pub fn load_enemies_shot_images(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    println!("Load enemies shot images");
    commands.insert_resource(EnemiesShotImages {
        enemy_01_shot: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_01_shot.png", SHOT_01_SIZE, 1, 4, Some(H_PADDING)),
        enemy_02_shot: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_02_shot.png", SHOT_02_SIZE, 1, 1, None),
        enemy_03_shot: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_03_shot.png", SHOT_03_SIZE, 1, 1, None),
        enemy_05_shot: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_05_shot.png", SHOT_05_SIZE, 1, 4, Some(H_PADDING)),
        enemy_06_shot: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_06_shot.png", SHOT_06_SIZE, 4, 1, Some(V_PADDING)),
        // enemy_07_shot: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_07_shot.png", SHOT_07_SIZE, 1, 2, Some(H_PADDING)),
        enemy_07_shot: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_07_shot.png", SHOT_07_SIZE, 1, 4, Some(H_PADDING)),
        enemy_08_shot_left: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_08_shot_left.png", SHOT_08_SIZE, 1, 1, None),
        enemy_08_shot_right: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_08_shot_right.png", SHOT_08_SIZE, 1, 1, None),
        enemy_09_shot_left: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_09_shot_left.png", SHOT_09_SIZE, 1, 2, Some(H_PADDING)),
        enemy_09_shot_right: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_09_shot_right.png", SHOT_09_SIZE, 1, 2, Some(H_PADDING)),
        enemy_13_shot: load_atlas(&assets, &mut texture_atlases, "images/enemies/enemy_13_shot.png", SHOT_13_SIZE, 1, 1, None),
    });
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>,)>,) 
{
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn get_enemy_from_10(enemies: &mut Vec<Enemy>, seq: usize) -> Option<&mut Enemy> {
    for enemy in enemies.iter_mut() {
        if enemy.enemy_seq == seq {
            return Some(enemy);
        }
    }
    None
}

fn process_movement(dt: f32, enemy: &mut Enemy, transform: &mut Transform, size: Vec2, walls_query: &Query<&WallComponent>) {
    enemy.velocity.x = enemy.velocity.x.clamp(-400.0, 400.0);
    enemy.velocity.y = enemy.velocity.y.clamp(-400.0, 400.0);

    let mut position = Vec2::splat(0.0);
    position += enemy.velocity;
    position *= dt;

    let target = transform.translation + Vec3::new(position.x, 0.0, 0.0);
    if !walls_query.iter().any(|&wall| collision_check(
        target, size, 
        wall.position, wall.size)) 
    {
        transform.translation = target;
    }
    else {
        enemy.velocity.x *= -1.0;
    }

    let target = transform.translation + Vec3::new(0.0, position.y, 0.0);
    if !walls_query.iter().any(|&wall| collision_check(
        target, size, 
        wall.position, wall.size)) 
    {
        transform.translation = target;
    }
    else {
        enemy.velocity.y *= -1.0;
    }

    if transform.translation.x < (size.x / 2.0) {
        transform.translation.x = size.x / 2.0;
        enemy.velocity.x *= -1.0;
    }

    if transform.translation.x > WINDOW_W - (size.x / 2.0) {
        transform.translation.x = WINDOW_W - size.x / 2.0;
        enemy.velocity.x *= -1.0;
    }

    if transform.translation.y > WINDOW_H - INFO_BAR_H - (size.y / 2.0) {
        transform.translation.y = WINDOW_H - INFO_BAR_H - size.y / 2.0;
        enemy.velocity.y *= -1.0;
    }

    if transform.translation.y < (size.y / 2.0) {
        transform.translation.y = size.y / 2.0;
        enemy.velocity.y *= -1.0;
    }

    enemy.posx = transform.translation.x;
    enemy.posy = transform.translation.y;
    enemy.first = false;
}

fn enemy_01_shooting(
    commands: &mut Commands, 
    enemy_cooldown: &mut u16,
    enemy_cooldown_max: u16,
    enemy_direction: GameDirection,
    enemy_posx: f32, enemy_posy: f32,
    player_posy: f32,
    enemies_shot_images: &Res<EnemiesShotImages>,) -> bool
{
    if player_posy > enemy_posy - 25.0 && player_posy < enemy_posy + 25.0
    {
        if *enemy_cooldown > 0 {
            *enemy_cooldown -= 1;
            return false;
        }

        let velocity: Vec2;
        let start_x: f32;
        match enemy_direction {
            GameDirection::Left => {
                velocity = Vec2::new(-300.0, 0.0);
                start_x = enemy_posx - 10.0;
            },
            GameDirection::Right => {
                velocity = Vec2::new(300.0, 0.0);
                start_x = enemy_posx + 10.0;
            },
            GameDirection::None => panic!("Unexpected direction!")
        }

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: enemies_shot_images.enemy_01_shot.clone(),
            sprite: TextureAtlasSprite {
                color: Color::WHITE,
                index: 0,
                custom_size: Some(SHOT_01_SIZE),
                ..Default::default()
            },
            transform: Transform { 
                translation: Vec3::new(start_x, enemy_posy, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EnemyShotComponent {
            enemy_type: 1,
            size: SHOT_01_SIZE,
            velocity,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

        *enemy_cooldown = enemy_cooldown_max;
        return true;
    }

    false
}

fn enemy_01_movement(
    mut commands: Commands,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&Transform, &EnemyComponent), With<EnemyType01>>,
    mut maze: ResMut<Maze>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx01: Res<AudioChannel<Shooting01Channel>>,
    mut sounds: ResMut<Sounds>,) 
{
    let player_transform = player_query.single();

    for (enemy_transfrom, component) in enemy_query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        if enemy_01_shooting(&mut commands, &mut enemy.shooting_cooldown, enemy.shooting_cooldown_max, enemy.direction,
            enemy_transfrom.translation.x, enemy_transfrom.translation.y, 
            player_transform.translation.y, &enemies_shot_images)
        {
            if !sfx01.is_playing_sound() {
                sfx01.play(sounds.enemy_01_shot.clone()).looped();
            }
            sounds.enemy_01_shot_counter += 1;
        }
    }
}

fn enemy_02_shooting(
    commands: &mut Commands, 
    enemy_cooldown: &mut u16,
    enemy_cooldown_max: u16,
    enemy_posx: f32, enemy_posy: f32,
    player_posx: f32, player_posy: f32,
    enemies_shot_images: &Res<EnemiesShotImages>,) -> bool
{
    if *enemy_cooldown > 0 {
        *enemy_cooldown -= 1;
        return false;
    }

    let mut velocity = Vec2::new(
        player_posx - enemy_posx, 
        player_posy - enemy_posy).normalize();
    velocity *= 200.0;

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: enemies_shot_images.enemy_02_shot.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(SHOT_02_SIZE),
            ..Default::default()
        },
        transform: Transform { 
            translation: Vec3::new(enemy_posx, enemy_posy + 10.0, 100.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(EnemyShotComponent {
        enemy_type: 2,
        size: SHOT_02_SIZE,
        velocity,
    });

    *enemy_cooldown = enemy_cooldown_max;
    true
}

fn enemy_02_movement(
    mut commands: Commands,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&Transform, &EnemyComponent), With<EnemyType02>>,
    mut maze: ResMut<Maze>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sounds: Res<Sounds>,) 
{
    let player_transform = player_query.single();

    for (enemy_transfrom, component) in enemy_query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        if enemy_02_shooting(&mut commands, &mut enemy.shooting_cooldown, enemy.shooting_cooldown_max, 
            enemy_transfrom.translation.x, enemy_transfrom.translation.y, 
            player_transform.translation.x, player_transform.translation.y, &enemies_shot_images)
        {
            sfx.play(sounds.enemy_02_shot.clone());
        }
    }
}

fn enemy_03_shooting(
    commands: &mut Commands, 
    enemy_cooldown: &mut u16,
    enemy_cooldown_max: u16,
    enemy_direction: GameDirection,
    enemy_posx: f32, enemy_posy: f32,
    player_posy: f32,
    enemies_shot_images: &Res<EnemiesShotImages>,) -> bool
{
    if player_posy > enemy_posy - 25.0 &&
       player_posy < enemy_posy + 25.0
    {
        if *enemy_cooldown > 0 {
            *enemy_cooldown -= 1;
            return false;
        }

        let velocity: Vec2;
        let start_x: f32;
        match enemy_direction {
            GameDirection::Left => {
                velocity = Vec2::new(-300.0, 0.0);
                start_x = enemy_posx - 15.0;
            },
            GameDirection::Right => {
                velocity = Vec2::new(300.0, 0.0);
                start_x = enemy_posx + 15.0;
            },
            GameDirection::None => panic!("Unexpected direction!")
        }

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: enemies_shot_images.enemy_03_shot.clone(),
            sprite: TextureAtlasSprite {
                color: Color::WHITE,
                index: 0,
                custom_size: Some(SHOT_03_SIZE),
                ..Default::default()
            },
            transform: Transform { 
                translation: Vec3::new(start_x, enemy_posy + 4.0, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EnemyShotComponent {
            enemy_type: 3,
            size: SHOT_03_SIZE,
            velocity,
        });

        *enemy_cooldown = enemy_cooldown_max;
        return true;
    }
    
    false
}

fn enemy_03_movement(
    mut commands: Commands,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&Transform, &EnemyComponent, &mut TextureAtlasSprite), With<EnemyType03>>,
    mut maze: ResMut<Maze>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sounds: Res<Sounds>,) 
{
    let player_transform = player_query.single();

    for (enemy_transfrom, component, mut enemy_sprite) in enemy_query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        if enemy_transfrom.translation.x > player_transform.translation.x {
            enemy_sprite.index = 0;
            enemy.direction = GameDirection::Left;
        }
        else {
            enemy_sprite.index = 1;
            enemy.direction = GameDirection::Right;
        }

        if enemy_03_shooting(&mut commands, &mut enemy.shooting_cooldown, enemy.shooting_cooldown_max, enemy.direction,
            enemy_transfrom.translation.x, enemy_transfrom.translation.y, 
            player_transform.translation.y, &enemies_shot_images)
        {
            sfx.play(sounds.enemy_03_13_shot.clone());
        }
    }
}

fn enemy_05_shooting(
    commands: &mut Commands, 
    enemy_cooldown: &mut u16,
    enemy_cooldown_max: u16,
    enemy_posx: f32, enemy_posy: f32,
    player_posx: f32,
    enemies_shot_images: &Res<EnemiesShotImages>,) -> bool
{
    if player_posx > enemy_posx - 25.0 &&
       player_posx < enemy_posx + 25.0
    {
        if *enemy_cooldown > 0 {
            *enemy_cooldown -= 1;
            return false;
        }

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: enemies_shot_images.enemy_05_shot.clone(),
            sprite: TextureAtlasSprite {
                color: Color::WHITE,
                index: 0,
                custom_size: Some(SHOT_05_SIZE),
                ..Default::default()
            },
            transform: Transform { 
                translation: Vec3::new(enemy_posx, enemy_posy + 25.0, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EnemyShotComponent {
            enemy_type: 5,
            size: SHOT_05_SIZE,
            velocity: Vec2::new(0.0, 300.0),
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

        *enemy_cooldown = enemy_cooldown_max;
        return true;
    }

    false
}

fn enemy_05_movement(
    mut commands: Commands,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&Transform, &EnemyComponent), With<EnemyType05>>,
    mut maze: ResMut<Maze>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx: Res<AudioChannel<Shooting05Channel>>,
    mut sounds: ResMut<Sounds>,) 
{
    let player_transform = player_query.single();

    for (enemy_transfrom, component) in enemy_query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        if enemy_05_shooting(&mut commands, &mut enemy.shooting_cooldown, enemy.shooting_cooldown_max,
            enemy_transfrom.translation.x, enemy_transfrom.translation.y, 
            player_transform.translation.x, &enemies_shot_images)
        {
            sfx.stop();
            sfx.play(sounds.enemy_05_shot.clone()).looped();
            sounds.enemy_05_shot_counter += 1;
        }
    }
}

fn enemy_06_movement(
    mut commands: Commands,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&Transform, &EnemyComponent), With<EnemyType06>>,
    mut maze: ResMut<Maze>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx: Res<AudioChannel<Shooting06Channel>>,
    mut sounds: ResMut<Sounds>,) 
{
    let player_transform = player_query.single();

    for (enemy_transfrom, component) in enemy_query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        if player_transform.translation.x > enemy_transfrom.translation.x - 25.0 &&
           player_transform.translation.x < enemy_transfrom.translation.x + 25.0
        {
            if enemy.shooting_cooldown > 0 {
                enemy.shooting_cooldown -= 1;
                continue;
            }

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: enemies_shot_images.enemy_06_shot.clone(),
                sprite: TextureAtlasSprite {
                    color: Color::WHITE,
                    index: 0,
                    custom_size: Some(SHOT_06_SIZE),
                    ..Default::default()
                },
                transform: Transform { 
                    translation: Vec3::new(enemy_transfrom.translation.x, enemy_transfrom.translation.y - 25.0, 100.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(EnemyShotComponent {
                enemy_type: 6,
                size: SHOT_06_SIZE,
                velocity: Vec2::new(0.0, -200.0),
            })
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

            sfx.stop();
            sfx.play(sounds.enemy_06_shot.clone()).looped();
            sounds.enemy_06_shot_counter += 1;

            enemy.shooting_cooldown = enemy.shooting_cooldown_max;
        }
    }
}

fn enemy_07_shooting(
    commands: &mut Commands, 
    enemy_cooldown: &mut u16,
    enemy_cooldown_max: u16,
    enemy_posx: f32, enemy_posy: f32,
    player_posx: f32, player_posy: f32,
    enemies_shot_images: &Res<EnemiesShotImages>,) -> bool
{
    if *enemy_cooldown > 0 {
        *enemy_cooldown -= 1;
        return false;
    }

    let mut rng = rand::thread_rng();

    let mut velocity = Vec2::new(
        player_posx - enemy_posx, 
        player_posy - enemy_posy).normalize();
    velocity *= 200.0;

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: enemies_shot_images.enemy_07_shot.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: rng.gen_range(0..=3),
            custom_size: Some(SHOT_07_SIZE),
            ..Default::default()
        },
        transform: Transform { 
            translation: Vec3::new(enemy_posx, enemy_posy + 10.0, 100.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(EnemyShotComponent {
        enemy_type: 7,
        size: SHOT_07_SIZE,
        velocity,
    })
    .insert(AnimationTimer(Timer::from_seconds(0.2, true)));

    *enemy_cooldown = enemy_cooldown_max;
    true
}

fn enemy_07_movement(
    mut commands: Commands,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&Transform, &EnemyComponent), With<EnemyType07>>,
    mut maze: ResMut<Maze>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sounds: Res<Sounds>,) 
{
    let player_transform = player_query.single();

    for (enemy_transfrom, component) in enemy_query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        if enemy_07_shooting(&mut commands, &mut enemy.shooting_cooldown, enemy.shooting_cooldown_max, 
            enemy_transfrom.translation.x, enemy_transfrom.translation.y, 
            player_transform.translation.x, player_transform.translation.y, 
            &enemies_shot_images)
        {
            sfx.play(sounds.enemy_07_shot.clone());
        }
    }
}

fn enemy_08_shooting(
    commands: &mut Commands, 
    enemy_entity: &Entity,
    enemy_direction: GameDirection,
    enemy_health: &mut i16,
    enemy_posx: f32, enemy_posy: f32,
    player_posx: f32, player_posy: f32,
    enemies_shot_images: &Res<EnemiesShotImages>,) -> bool
{
    let dx = (player_posx - enemy_posx).abs();
    let dy = (player_posy - enemy_posy).abs();
    let d = (dx - dy).abs();

    if d < 25.0 / 2.0 && enemy_direction == GameDirection::Left && player_posx < enemy_posx 
    {
        // missile up left
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: enemies_shot_images.enemy_08_shot_left.clone(),
            sprite: TextureAtlasSprite {
                color: Color::WHITE,
                index: 0,
                custom_size: Some(SHOT_08_SIZE),
                ..Default::default()
            },
            transform: Transform { 
                translation: Vec3::new(enemy_posx, enemy_posy, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EnemyShotComponent {
            enemy_type: 8,
            size: SHOT_09_SIZE,
            velocity: Vec2::new(-140.0, 140.0),
        });

        // despawn enemy
        commands.entity(*enemy_entity).despawn_recursive();
        *enemy_health = 0;

        return true;
    }
    else if d < 25.0 / 2.0 && enemy_direction == GameDirection::Right && player_posx > enemy_posx 
    {
        // missile up right
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: enemies_shot_images.enemy_08_shot_right.clone(),
            sprite: TextureAtlasSprite {
                color: Color::WHITE,
                index: 0,
                custom_size: Some(SHOT_08_SIZE),
                ..Default::default()
            },
            transform: Transform { 
                translation: Vec3::new(enemy_posx, enemy_posy, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EnemyShotComponent {
            enemy_type: 8,
            size: SHOT_08_SIZE,
            velocity: Vec2::new(140.0, 140.0),
        });

        // despawn enemy
        commands.entity(*enemy_entity).despawn_recursive();
        *enemy_health = 0;

        return true;
    }

    false
}

fn enemy_08_movement(
    mut commands: Commands,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(Entity, &Transform, &EnemyComponent), With<EnemyType08>>,
    mut maze: ResMut<Maze>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx: Res<AudioChannel<Shooting08Channel>>,
    mut sounds: ResMut<Sounds>,) 
{
    let player_transform = player_query.single();

    for (enemy_entity, enemy_transfrom, component) in enemy_query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        if enemy_08_shooting(&mut commands, &enemy_entity, enemy.direction, &mut enemy.health, 
            enemy_transfrom.translation.x, enemy_transfrom.translation.y, 
            player_transform.translation.x, player_transform.translation.y, 
            &enemies_shot_images)
        {
            sfx.stop();
            sfx.play(sounds.enemy_08_shot.clone()).looped();
            sounds.enemy_08_shot_counter += 1;
        }
    }
}

fn enemy_09_shooting(
    commands: &mut Commands, 
    enemy_entity: &Entity,
    enemy_health: &mut i16,
    enemy_posx: f32, enemy_posy: f32,
    player_posx: f32, player_posy: f32,
    enemies_shot_images: &Res<EnemiesShotImages>,) -> bool
{
    if player_posy > enemy_posy - 25.0 && player_posy < enemy_posy + 25.0
    {
        if player_posx < enemy_posx {
            // missile left
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: enemies_shot_images.enemy_09_shot_left.clone(),
                sprite: TextureAtlasSprite {
                    color: Color::WHITE,
                    index: 0,
                    custom_size: Some(SHOT_09_SIZE),
                    ..Default::default()
                },
                transform: Transform { 
                    translation: Vec3::new(enemy_posx, enemy_posy, 100.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(EnemyShotComponent {
                enemy_type: 9,
                size: SHOT_09_SIZE,
                velocity: Vec2::new(-200.0, 0.0),
            })
            .insert(AnimationTimer(Timer::from_seconds(0.05, true)));
        }
        else {
            // missile right
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: enemies_shot_images.enemy_09_shot_right.clone(),
                sprite: TextureAtlasSprite {
                    color: Color::WHITE,
                    index: 0,
                    custom_size: Some(SHOT_09_SIZE),
                    ..Default::default()
                },
                transform: Transform { 
                    translation: Vec3::new(enemy_posx, enemy_posy, 100.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(EnemyShotComponent {
                enemy_type: 9,
                size: SHOT_09_SIZE,
                velocity: Vec2::new(200.0, 0.0),
            })
            .insert(AnimationTimer(Timer::from_seconds(0.05, true)));
        }

        // despawn enemy
        commands.entity(*enemy_entity).despawn_recursive();
        *enemy_health = 0;
        return true;
    }

    false
}

fn enemy_09_movement(
    mut commands: Commands,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(Entity, &Transform, &EnemyComponent), With<EnemyType09>>,
    mut maze: ResMut<Maze>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sfx09: Res<AudioChannel<Shooting09Channel>>,
    mut sounds: ResMut<Sounds>,) 
{
    let player_transform = player_query.single();

    for (enemy_entity, enemy_transfrom, component) in enemy_query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        if enemy_09_shooting(&mut commands, &enemy_entity, &mut enemy.health, 
            enemy_transfrom.translation.x, enemy_transfrom.translation.y, 
            player_transform.translation.x, player_transform.translation.y, 
            &enemies_shot_images)
        {
            sfx.play(sounds.enemy_09_launch.clone());

            sfx09.stop();
            sfx09.play(sounds.enemy_09_shot.clone()).looped();
            sounds.enemy_09_shot_counter += 1;
        }
    }
}

fn enemy_10_spawning(
    commands: &mut Commands, 
    rng: &mut ThreadRng,
    maze: &mut Maze,
    enemy_posx: f32, enemy_posy: f32,
    room_seq: usize,
    enemies_images: &Res<EnemiesImages>) -> bool
{
    let room = &mut maze.rooms[room_seq];

    if room.enemies_from_10.len() < 10 {
        let id = rng.gen_range(11..=17); // random spawnable enemy
        let color_idx = rng.gen_range(0..ENEMY_COLORS.len());

        println!("Enemy 10: spawn enemy {}, seq={}", id, room.from_10_seq);

        let velocity: Vec2;
        if id == 15 || id == 16 {
            let mut x = 150.0;
            let mut y = 150.0;
            if rng.gen_bool(1.0/2.0) {
                x *= -1.0;
            }
            if rng.gen_bool(1.0/2.0) {
                y *= -1.0;
            }
            velocity = Vec2::new(x, y);
        }
        else {
            velocity = Vec2::splat(0.0);
        }

        let cooldown: u16;
        if id == 13 {
            cooldown = 500;
        }
        else {
            cooldown = u16::MAX;
        }

        let new_enemy = Enemy {
            health: 10,
            room_seq,
            enemy_seq: room.from_10_seq, 
            enemy_type: id, 
            enemy_subtype: 0, 
            first: false,
            posx: enemy_posx, 
            posy: enemy_posy, 
            color: ENEMY_COLORS[color_idx],
            velocity,
            shooting_cooldown: 0,
            shooting_cooldown_max: cooldown,
            direction: GameDirection::None,
            is_from_10: true,
            fellow_enemy: None,
            fellow_item: None,
        };

        match id {
            11 => {
                let e = crate::maze::spawn_enemy(commands, &new_enemy, enemies_images.enemy_11.clone(), 
                    ENEMY_NN_SIZE, Vec3::new(enemy_posx, enemy_posy, 100.0), 0);
                commands.entity(e).insert(EnemyType11);
            },
            12 => {
                let e = crate::maze::spawn_enemy(commands, &new_enemy, enemies_images.enemy_12.clone(), 
                    ENEMY_NN_SIZE, Vec3::new(enemy_posx, enemy_posy, 100.0), 0);
                commands.entity(e).insert(EnemyType12)
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
            },
            13 => {
                let e = crate::maze::spawn_enemy(commands, &new_enemy, enemies_images.enemy_13_left.clone(), 
                    ENEMY_NN_SIZE, Vec3::new(enemy_posx, enemy_posy, 100.0), 0);
                commands.entity(e).insert(EnemyType13)
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
            },
            14 => {
                let e = crate::maze::spawn_enemy(commands, &new_enemy, enemies_images.enemy_14_left.clone(), 
                    ENEMY_NN_SIZE, Vec3::new(enemy_posx, enemy_posy, 100.0), 0);
                commands.entity(e).insert(EnemyType14)
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
            },
            15 => {
                let e = crate::maze::spawn_enemy(commands, &new_enemy, enemies_images.enemy_15.clone(), 
                    ENEMY_NN_SIZE, Vec3::new(enemy_posx, enemy_posy, 100.0), 0);
                commands.entity(e).insert(EnemyType15)
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
            },
            16 => {
                let e = crate::maze::spawn_enemy(commands, &new_enemy, enemies_images.enemy_16.clone(), 
                    ENEMY_NN_SIZE, Vec3::new(enemy_posx, enemy_posy, 100.0), 0);
                commands.entity(e).insert(EnemyType16)
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
            },
            17 => {
                let e = crate::maze::spawn_enemy(commands, &new_enemy, enemies_images.enemy_17_left.clone(), 
                    ENEMY_NN_SIZE, Vec3::new(enemy_posx, enemy_posy, 100.0), 0);
                commands.entity(e).insert(EnemyType17)
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
            },
            _ => panic!("Unexpected enemy id!")
        }

        room.enemies_from_10.push(new_enemy);
        room.from_10_seq += 1;
    }

    true
}

fn enemy_10_movement(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &EnemyComponent, &mut TextureAtlasSprite), With<EnemyType10>>,
    mut maze: ResMut<Maze>,
    enemies_images: Res<EnemiesImages>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sounds: Res<Sounds>,) 
{
    let mut rng = rand::thread_rng();

    for (enemy_transfrom, component, mut sprite) in enemy_query.iter_mut() 
    {
        {
            let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];
            if enemy.shooting_cooldown > 0 {
                enemy.shooting_cooldown -= 1;
                continue;
            }
            enemy.shooting_cooldown = enemy.shooting_cooldown_max;
        }

        if enemy_10_spawning(&mut commands, &mut rng, &mut maze, enemy_transfrom.translation.x, enemy_transfrom.translation.y, 
            component.room_seq, &enemies_images) 
        {
            if sprite.index == 0 {
                sprite.index = 1;
            }
            else {
                sprite.index = 0;
            }
            sfx.play(sounds.enemy_10.clone());
        }
    }
}

fn enemy_11_movement(
    time: Res<Time>,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyComponent, &mut TextureAtlasSprite), (With<EnemyType11>, Without<PlayerComponent>)>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>,) 
{
    let mut rng = rand::thread_rng();
    let player_transform = player_query.single();

    for (mut enemy_transfrom, component, mut enemy_sprite) in enemy_query.iter_mut() {
        // println!("MOVEMENT: Enemy11");

        if enemy_transfrom.translation.x > player_transform.translation.x {
            enemy_sprite.index = 0;
        }
        else {
            enemy_sprite.index = 1;
        }

        let enemy: &mut Enemy;
        if component.is_from_10 {
            // enemy = &mut maze.rooms[component.room_seq].enemies_from_10[component.enemy_seq];
            let eopt = get_enemy_from_10(&mut maze.rooms[component.room_seq].enemies_from_10, component.enemy_seq);
            if eopt.is_some() {
                enemy = eopt.unwrap();
            }
            else {
                return;
            }
        }
        else {
            enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];
        }

        enemy.velocity += Vec2::new(rng.gen_range(-1000.0..1000.0), rng.gen_range(-1000.0..1000.0)) * time.delta_seconds();
        enemy.velocity += Vec2::new(0.0, enemy_transfrom.translation.y - player_transform.translation.y) * time.delta_seconds() * -1.0;

        process_movement(time.delta_seconds(), enemy, &mut enemy_transfrom, ENEMY_NN_SIZE, &walls_query);
    }
}

fn enemy_12_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut EnemyComponent), With<EnemyType12>>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>,) 
{
    let mut rng = rand::thread_rng();

    for (mut transfrom, component) in query.iter_mut() {
        let enemy: &mut Enemy;
        if component.is_from_10 {
            // enemy = &mut maze.rooms[component.room_seq].enemies_from_10[component.enemy_seq];
            let eopt = get_enemy_from_10(&mut maze.rooms[component.room_seq].enemies_from_10, component.enemy_seq);
            if eopt.is_some() {
                enemy = eopt.unwrap();
            }
            else {
                return;
            }
        }
        else {
            enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];
        }

        enemy.velocity += Vec2::new(rng.gen_range(-1000.0..1000.0), rng.gen_range(-1000.0..1000.0)) * time.delta_seconds();

        process_movement(time.delta_seconds(), enemy, &mut transfrom, ENEMY_NN_SIZE, &walls_query);
    }
}

fn enemy_13_movement(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyComponent, &mut Handle<TextureAtlas>), (With<EnemyType13>, Without<PlayerComponent>)>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>,
    enemies_images: Res<EnemiesImages>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sounds: Res<Sounds>,) 
{
    let mut rng = rand::thread_rng();
    let player_transform = player_query.single();

    for (mut enemy_transfrom, component, mut enemy_atlas) in enemy_query.iter_mut() {
        let enemy: &mut Enemy;
        if component.is_from_10 {
            // enemy = &mut maze.rooms[component.room_seq].enemies_from_10[component.enemy_seq];
            let eopt = get_enemy_from_10(&mut maze.rooms[component.room_seq].enemies_from_10, component.enemy_seq);
            if eopt.is_some() {
                enemy = eopt.unwrap();
            }
            else {
                return;
            }
        }
        else {
            enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];
        }

        if enemy_transfrom.translation.x > player_transform.translation.x {
            *enemy_atlas = enemies_images.enemy_13_left.clone();
            enemy.direction = GameDirection::Left;
        }
        else {
            *enemy_atlas = enemies_images.enemy_13_right.clone();
            enemy.direction = GameDirection::Right;
        }

        enemy.velocity += Vec2::new(rng.gen_range(-1000.0..1000.0), rng.gen_range(-1000.0..1000.0)) * time.delta_seconds();
        enemy.velocity += Vec2::new(0.0, enemy_transfrom.translation.y - player_transform.translation.y) * time.delta_seconds() * -1.0;

        process_movement(time.delta_seconds(), enemy, &mut enemy_transfrom, ENEMY_NN_SIZE, &walls_query);

        if player_transform.translation.y > enemy_transfrom.translation.y - 25.0 &&
           player_transform.translation.y < enemy_transfrom.translation.y + 25.0
        {
            if enemy.shooting_cooldown > 0 {
                enemy.shooting_cooldown -= 1;
                continue;
            }

            let velocity: Vec2;
            let start_x: f32;
            match enemy.direction {
                GameDirection::Left => {
                    velocity = Vec2::new(-300.0, 0.0);
                    start_x = enemy_transfrom.translation.x - 12.0;
                },
                GameDirection::Right => {
                    velocity = Vec2::new(300.0, 0.0);
                    start_x = enemy_transfrom.translation.x + 12.0;
                },
                GameDirection::None => panic!("Unexpected direction!")
            }

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: enemies_shot_images.enemy_13_shot.clone(),
                sprite: TextureAtlasSprite {
                    color: Color::WHITE,
                    index: 0,
                    custom_size: Some(SHOT_13_SIZE),
                    ..Default::default()
                },
                transform: Transform { 
                    translation: Vec3::new(start_x, enemy_transfrom.translation.y + 8.0, 100.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(EnemyShotComponent {
                enemy_type: 13,
                size: SHOT_13_SIZE,
                velocity,
            });

            sfx.play(sounds.enemy_03_13_shot.clone());

            enemy.shooting_cooldown = enemy.shooting_cooldown_max;
        }
    }
}

fn enemy_14_movement(
    time: Res<Time>,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyComponent, &mut Handle<TextureAtlas>), (With<EnemyType14>, Without<PlayerComponent>)>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>,
    enemies_images: Res<EnemiesImages>) 
{
    let mut rng = rand::thread_rng();
    let player_transform = player_query.single();

    for (mut enemy_transfrom, component, mut enemy_atlas) in enemy_query.iter_mut() {
        if enemy_transfrom.translation.x > player_transform.translation.x {
            *enemy_atlas = enemies_images.enemy_14_left.clone();
        }
        else {
            *enemy_atlas = enemies_images.enemy_14_right.clone();
        }

        let enemy: &mut Enemy;
        if component.is_from_10 {
            // enemy = &mut maze.rooms[component.room_seq].enemies_from_10[component.enemy_seq];
            let eopt = get_enemy_from_10(&mut maze.rooms[component.room_seq].enemies_from_10, component.enemy_seq);
            if eopt.is_some() {
                enemy = eopt.unwrap();
            }
            else {
                return;
            }
        }
        else {
            enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];
        }

        enemy.velocity += Vec2::new(rng.gen_range(-1000.0..1000.0), rng.gen_range(-1000.0..1000.0)) * time.delta_seconds();
        enemy.velocity += Vec2::new(0.0, enemy_transfrom.translation.y - player_transform.translation.y) * time.delta_seconds() * -1.0;

        process_movement(time.delta_seconds(), enemy, &mut enemy_transfrom, ENEMY_NN_SIZE, &walls_query);
    }
}

fn enemy_15_movement(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyComponent), With<EnemyType15>>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>) 
{
    for (mut transfrom, component) in enemy_query.iter_mut() {

        let enemy: &mut Enemy;
        if component.is_from_10 {
            // enemy = &mut maze.rooms[component.room_seq].enemies_from_10[component.enemy_seq];
            let eopt = get_enemy_from_10(&mut maze.rooms[component.room_seq].enemies_from_10, component.enemy_seq);
            if eopt.is_some() {
                enemy = eopt.unwrap();
            }
            else {
                return;
            }
        }
        else {
            enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];
        }

        process_movement(time.delta_seconds(), enemy, &mut transfrom, ENEMY_NN_SIZE, &walls_query);
    }
}

fn enemy_16_movement(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyComponent), With<EnemyType16>>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>) 
{
    for (mut transfrom, component) in enemy_query.iter_mut() {

        let enemy: &mut Enemy;
        if component.is_from_10 {
            // enemy = &mut maze.rooms[component.room_seq].enemies_from_10[component.enemy_seq];
            let eopt = get_enemy_from_10(&mut maze.rooms[component.room_seq].enemies_from_10, component.enemy_seq);
            if eopt.is_some() {
                enemy = eopt.unwrap();
            }
            else {
                return;
            }
        }
        else {
            enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];
        }

        process_movement(time.delta_seconds(), enemy, &mut transfrom, ENEMY_NN_SIZE, &walls_query);
    }
}

fn enemy_17_movement(
    time: Res<Time>,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyComponent, &mut Handle<TextureAtlas>), (With<EnemyType17>, Without<PlayerComponent>)>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>,
    enemies_images: Res<EnemiesImages>) 
{
    let mut rng = rand::thread_rng();
    let player_transform = player_query.single();

    for (mut enemy_transfrom, component, mut enemy_atlas) in enemy_query.iter_mut() {
        if enemy_transfrom.translation.x > player_transform.translation.x {
            *enemy_atlas = enemies_images.enemy_17_left.clone();
        }
        else {
            *enemy_atlas = enemies_images.enemy_17_right.clone();
        }

        let enemy: &mut Enemy;
        if component.is_from_10 {
            // enemy = &mut maze.rooms[component.room_seq].enemies_from_10[component.enemy_seq];
            let eopt = get_enemy_from_10(&mut maze.rooms[component.room_seq].enemies_from_10, component.enemy_seq);
            if eopt.is_some() {
                enemy = eopt.unwrap();
            }
            else {
                return;
            }
        }
        else {
            enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];
        }

        enemy.velocity += Vec2::new(rng.gen_range(-1000.0..1000.0), rng.gen_range(-1000.0..1000.0)) * time.delta_seconds();
        enemy.velocity += Vec2::new(0.0, enemy_transfrom.translation.y - player_transform.translation.y) * time.delta_seconds() * -1.0;

        process_movement(time.delta_seconds(), enemy, &mut enemy_transfrom, ENEMY_NN_SIZE, &walls_query);
    }
}

fn enemy_18_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut EnemyComponent), With<EnemyType18>>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>,) 
{
    let mut rng = rand::thread_rng();

    for (mut transfrom, component) in query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        enemy.velocity += Vec2::new(rng.gen_range(-1000.0..1000.0), rng.gen_range(-1000.0..1000.0)) * time.delta_seconds();

        process_movement(time.delta_seconds(), enemy, &mut transfrom, ENEMY_18_SIZE, &walls_query);
    }
}

fn enemy_19_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut EnemyComponent), With<EnemyType19>>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>,) 
{
    let mut rng = rand::thread_rng();

    for (mut transfrom, component) in query.iter_mut() {
        let enemy = &mut maze.rooms[component.room_seq].enemies[component.enemy_seq];

        enemy.velocity += Vec2::new(rng.gen_range(-1000.0..1000.0), rng.gen_range(-1000.0..1000.0)) * time.delta_seconds();

        process_movement(time.delta_seconds(), enemy, &mut transfrom, ENEMY_19_SIZE, &walls_query);
    }
}

fn process_carrier_movement(dt: f32, enemy: &mut Enemy, transform: &mut Transform, walls_query: &Query<&WallComponent>) {
    enemy.velocity.x = enemy.velocity.x.clamp(-400.0, 400.0);
    enemy.velocity.y = enemy.velocity.y.clamp(-400.0, 400.0);

    let mut position = Vec2::splat(0.0);
    position += enemy.velocity;
    position *= dt;

    let mut target_size = ENEMY_NN_SIZE;
    if enemy.fellow_enemy.is_some() {
        let fellow = enemy.fellow_enemy.as_ref().unwrap();
        match fellow.enemy_type {
            7 => target_size.y += ENEMY_07_SIZE.y + 4.0,
            _ => target_size.y += ENEMY_NN_SIZE.y,
        }
    }
    if enemy.fellow_item.is_some() {
        target_size.y += ITEM_H;
    }
    // println!("Target size = {:?}", target_size);
    
    // check collisions in x axis
    let target = transform.translation + Vec3::new(position.x, 0.0, 0.0);
    let target_tmp = Vec3::new(
        transform.translation.x + position.x,
        transform.translation.y - (ENEMY_NN_SIZE.y / 2.0) + (target_size.y / 2.0),
        0.0);
    if !walls_query.iter().any(|&wall| collision_check(
        target_tmp, target_size, 
        wall.position, wall.size)) 
    {
        transform.translation = target;
    }
    else {
        enemy.velocity.x *= -1.0;
    }

    // check collisions in y axis
    let target = transform.translation + Vec3::new(0.0, position.y, 0.0);
    let target_tmp = Vec3::new(
        transform.translation.x,
        (transform.translation.y - (ENEMY_NN_SIZE.y / 2.0) + (target_size.y / 2.0)) + position.y,
        0.0);
    if !walls_query.iter().any(|&wall| collision_check(
        target_tmp, target_size, 
        wall.position, wall.size)) 
    {
        transform.translation = target;
    }
    else {
        enemy.velocity.y *= -1.0;
    }

    if transform.translation.x < (target_size.x / 2.0) {
        transform.translation.x = target_size.x / 2.0;
        enemy.velocity.x *= -1.0;
    }

    if transform.translation.x > WINDOW_W - (target_size.x / 2.0) {
        transform.translation.x = WINDOW_W - target_size.x / 2.0;
        enemy.velocity.x *= -1.0;
    }

    let y_tmp = transform.translation.y - (ENEMY_NN_SIZE.y / 2.0) + (target_size.y / 2.0);
    
    if y_tmp > WINDOW_H - INFO_BAR_H - (target_size.y / 2.0) {
        transform.translation.y = WINDOW_H - INFO_BAR_H - (target_size.y / 2.0) - (ENEMY_NN_SIZE.y / 2.0);
        enemy.velocity.y *= -1.0;
    }

    if y_tmp < (target_size.y / 2.0) {
        transform.translation.y = ENEMY_NN_SIZE.y / 2.0;
        enemy.velocity.y *= -1.0;
    }

    enemy.posx = transform.translation.x;
    enemy.posy = transform.translation.y;
    enemy.first = false;
}

fn enemy_20_movement(
    mut commands: Commands, 
    time: Res<Time>,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut query_e20: Query<(Entity, &mut Transform, &mut EnemyComponent), (With<EnemyType20>, Without<PlayerComponent>)>,
    mut query_e03: Query<&mut TextureAtlasSprite, (With<EnemyType03Fellow>, Without<EnemyType10Fellow>)>,
    mut query_e10: Query<&mut TextureAtlasSprite, (With<EnemyType10Fellow>, Without<EnemyType03Fellow>)>,
    walls_query: Query<&WallComponent>,
    mut maze: ResMut<Maze>,
    enemies_images: Res<EnemiesImages>,
    enemies_shot_images: Res<EnemiesShotImages>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sfx01: Res<AudioChannel<Shooting01Channel>>,
    sfx05: Res<AudioChannel<Shooting05Channel>>,
    sfx08: Res<AudioChannel<Shooting08Channel>>,
    sfx09: Res<AudioChannel<Shooting09Channel>>,
    mut sounds: ResMut<Sounds>,) 
{
    let mut rng = rand::thread_rng();

    for (e20_entity, mut e20_transfrom, e20_component) in query_e20.iter_mut() {
        let enemy = &mut maze.rooms[e20_component.room_seq].enemies[e20_component.enemy_seq];

        enemy.velocity += Vec2::new(rng.gen_range(-1000.0..1000.0), rng.gen_range(-1000.0..1000.0)) * time.delta_seconds();
        
        process_carrier_movement(time.delta_seconds(), enemy, &mut e20_transfrom, &walls_query);

        if let Some(fellow) = &mut enemy.fellow_enemy {
            let player_transform = player_query.single();

            // process (possible) shooting of 1, 2, 3, 5, 7, 8, 9
            if fellow.enemy_type == 1 {
                if enemy_01_shooting(&mut commands, &mut fellow.shooting_cooldown, fellow.shooting_cooldown_max, fellow.direction, 
                    e20_transfrom.translation.x, e20_transfrom.translation.y + ENEMY_NN_SIZE.y, 
                    player_transform.translation.y, 
                    &enemies_shot_images)
                {
                    if !sfx01.is_playing_sound() {
                        sfx01.play(sounds.enemy_01_shot.clone()).looped();
                    }
                    sounds.enemy_01_shot_counter += 1;
                }
            }
            else if fellow.enemy_type == 2 {
                if enemy_02_shooting(&mut commands, &mut fellow.shooting_cooldown, fellow.shooting_cooldown_max, 
                    e20_transfrom.translation.x, e20_transfrom.translation.y + ENEMY_NN_SIZE.y, 
                    player_transform.translation.x, player_transform.translation.y, 
                    &enemies_shot_images)
                {
                    sfx.play(sounds.enemy_02_shot.clone());
                }
            }
            else if fellow.enemy_type == 3 {
                for mut sprite_e03 in query_e03.iter_mut() {
                    if e20_transfrom.translation.x > player_transform.translation.x {
                        sprite_e03.index = 0;
                        fellow.direction = GameDirection::Left;
                    }
                    else {
                        sprite_e03.index = 1;
                        fellow.direction = GameDirection::Right;
                    }
                }

                if enemy_03_shooting(&mut commands, &mut fellow.shooting_cooldown, fellow.shooting_cooldown_max, fellow.direction, 
                    e20_transfrom.translation.x, e20_transfrom.translation.y + ENEMY_NN_SIZE.y, 
                    player_transform.translation.y, 
                    &enemies_shot_images)
                {
                    sfx.play(sounds.enemy_03_13_shot.clone());
                }
            }
            else if fellow.enemy_type == 5 {
                if enemy_05_shooting(&mut commands, &mut fellow.shooting_cooldown, fellow.shooting_cooldown_max, 
                    e20_transfrom.translation.x, e20_transfrom.translation.y + ENEMY_NN_SIZE.y, 
                    player_transform.translation.x, 
                    &enemies_shot_images)
                {
                    sfx05.stop();
                    sfx05.play(sounds.enemy_05_shot.clone()).looped();
                    sounds.enemy_05_shot_counter += 1;
                }
            }
            else if fellow.enemy_type == 7 {
                if enemy_07_shooting(&mut commands, &mut fellow.shooting_cooldown, fellow.shooting_cooldown_max, 
                    e20_transfrom.translation.x, e20_transfrom.translation.y + ENEMY_NN_SIZE.y, 
                    player_transform.translation.x, player_transform.translation.y, 
                    &enemies_shot_images)
                {
                    sfx.play(sounds.enemy_07_shot.clone());
                }
            }
            else if fellow.enemy_type == 8 {
                if enemy_08_shooting(&mut commands, &e20_entity, fellow.direction, &mut enemy.health, 
                    e20_transfrom.translation.x, e20_transfrom.translation.y + ENEMY_NN_SIZE.y, 
                    player_transform.translation.x, player_transform.translation.y, 
                    &enemies_shot_images)
                {
                    sfx08.stop();
                    sfx08.play(sounds.enemy_08_shot.clone()).looped();
                    sounds.enemy_08_shot_counter += 1;
                }
            }
            else if fellow.enemy_type == 9 {
                if enemy_09_shooting(&mut commands, &e20_entity, &mut enemy.health, 
                    e20_transfrom.translation.x, e20_transfrom.translation.y + ENEMY_NN_SIZE.y, 
                    player_transform.translation.x, player_transform.translation.y, 
                    &enemies_shot_images)
                {
                    sfx.play(sounds.enemy_09_launch.clone());

                    sfx09.stop();
                    sfx09.play(sounds.enemy_09_shot.clone()).looped();
                    sounds.enemy_09_shot_counter += 1;
                }
            }
            // process (possible) spawning of 10
            else if fellow.enemy_type == 10 {
                if fellow.shooting_cooldown > 0 {
                    fellow.shooting_cooldown -= 1;
                    continue;
                }
                fellow.shooting_cooldown = fellow.shooting_cooldown_max;

                if enemy_10_spawning(&mut commands, &mut rng, &mut maze, e20_transfrom.translation.x, e20_transfrom.translation.y + ENEMY_NN_SIZE.y, 
                    e20_component.room_seq, &enemies_images) 
                {
                    for mut sprite_e10 in query_e10.iter_mut() {
                        if sprite_e10.index == 0 {
                            sprite_e10.index = 1;
                        }
                        else {
                            sprite_e10.index = 0;
                        }
                        sfx.play(sounds.enemy_10.clone());
                    }
                }
            }
        }
    }
}

fn enemy_shot_movement(
    mut commands: Commands, 
    mut shot_query: Query<(Option<Entity>, Option<&EnemyShotComponent>, Option<&mut Transform>), Without<PlayerComponent>>,
    player_query: Query<&Transform, (With<PlayerComponent>, Without<EnemyShotComponent>)>,
    mut player: ResMut<Player>,
    walls_query: Query<&WallComponent>,
    time: Res<Time>,
    explosions_images: Res<ExplosionsImages>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sfx01: Res<AudioChannel<Shooting01Channel>>,
    sfx05: Res<AudioChannel<Shooting05Channel>>,
    sfx06: Res<AudioChannel<Shooting06Channel>>,
    sfx08: Res<AudioChannel<Shooting08Channel>>,
    sfx09: Res<AudioChannel<Shooting09Channel>>,
    mut sounds: ResMut<Sounds>,) 
{
    let player_transform = player_query.single();

    for (eopt, copt, topt) in shot_query.iter_mut() {
        if let Some(shot_component) = copt {
            if let Some(mut shot_transform) = topt {

                let target = shot_transform.translation + 
                    Vec3::new(shot_component.velocity.x * time.delta_seconds(), 
                              shot_component.velocity.y * time.delta_seconds(), 0.0);

                // check shot collision with player
                if collision_check(target, shot_component.size, 
                    player_transform.translation, Vec2::new(PLAYER_W, PLAYER_H)) 
                {
                    // enemy shot hit player

                    if shot_component.enemy_type == 1 && sounds.enemy_01_shot_counter > 0 {
                        // println!("Shot 01 counter = {}", sounds.enemy_01_shot_counter);
                        sounds.enemy_01_shot_counter -= 1;
                        if sounds.enemy_01_shot_counter == 0 {
                            sfx01.stop();
                        }
                    }

                    if shot_component.enemy_type == 5 && sounds.enemy_05_shot_counter > 0 {
                        sounds.enemy_05_shot_counter -= 1;
                        if sounds.enemy_05_shot_counter == 0 {
                            sfx05.stop();
                        }
                    }

                    if shot_component.enemy_type == 6 && sounds.enemy_06_shot_counter > 0 {
                        sounds.enemy_06_shot_counter -= 1;
                        if sounds.enemy_06_shot_counter == 0 {
                            sfx06.stop();
                        }
                    }

                    if shot_component.enemy_type == 8 && sounds.enemy_08_shot_counter > 0 {
                        sounds.enemy_08_shot_counter -= 1;
                        if sounds.enemy_08_shot_counter == 0 {
                            sfx08.stop();
                        }
                    }

                    // enemy 9 -> player's instant death
                    if shot_component.enemy_type == 9 {
                        if sounds.enemy_09_shot_counter > 0 {
                            sounds.enemy_09_shot_counter = 0;
                            sfx09.stop();
                        }
                        player.health = 0.0;
                    }

                    // spawn boom for 1, 5, 6, 8
                    if shot_component.enemy_type == 1 ||
                        shot_component.enemy_type == 5 ||
                        shot_component.enemy_type == 6 ||
                        shot_component.enemy_type == 8 ||
                        shot_component.enemy_type == 9
                    {
                        crate::explosions::spawn_boom(&mut commands, shot_transform.translation, &explosions_images);
                        sfx.play(sounds.boom.clone());
                    }
                    else {
                        sfx.play(sounds.ship_damage.clone());
                    }

                    if let Some(entity) = eopt {
                        commands.entity(entity).despawn_recursive();
                    }

                    // reduce player damage
                    player.health -= 10.0;
                    continue;
                }

                // check shot collision with walls
                if walls_query.iter().any(|&wall| collision_check(
                    target, shot_component.size, 
                    wall.position, wall.size)) 
                {
                    // enemy shot hits wall

                    // spawn boom for 1, 5, 6, 8, 9
                    if shot_component.enemy_type == 1 ||
                        shot_component.enemy_type == 5 ||
                        shot_component.enemy_type == 6 ||
                        shot_component.enemy_type == 8 ||
                        shot_component.enemy_type == 9
                    {
                        crate::explosions::spawn_boom(&mut commands, shot_transform.translation, &explosions_images);
                        sfx.play(sounds.boom.clone());
                    }
                    
                    if shot_component.enemy_type == 1 && sounds.enemy_01_shot_counter > 0 {
                        // println!("Shot 01 counter = {}", sounds.enemy_01_shot_counter);
                        sounds.enemy_01_shot_counter -= 1;
                        if sounds.enemy_01_shot_counter == 0 {
                            sfx01.stop();
                        }
                    }

                    if shot_component.enemy_type == 5 && sounds.enemy_05_shot_counter > 0 {
                        sounds.enemy_05_shot_counter -= 1;
                        if sounds.enemy_05_shot_counter == 0 {
                            sfx05.stop();
                        }
                    }

                    if shot_component.enemy_type == 6 && sounds.enemy_06_shot_counter > 0 {
                        sounds.enemy_06_shot_counter -= 1;
                        if sounds.enemy_06_shot_counter == 0 {
                            sfx06.stop();
                        }
                    }

                    if shot_component.enemy_type == 8 && sounds.enemy_08_shot_counter > 0 {
                        sounds.enemy_08_shot_counter -= 1;
                        if sounds.enemy_08_shot_counter == 0 {
                            sfx08.stop();
                        }
                    }

                    if shot_component.enemy_type == 9 && sounds.enemy_09_shot_counter > 0 {
                        sounds.enemy_09_shot_counter -= 1;
                        if sounds.enemy_09_shot_counter == 0 {
                            sfx09.stop();
                        }
                    }

                    if let Some(entity) = eopt {
                        commands.entity(entity).despawn_recursive();
                    }

                    continue;
                }

                // is shot out of bounds ?
                if shot_transform.translation.x < 0.0 || shot_transform.translation.x > WINDOW_W ||
                   shot_transform.translation.y < 0.0 || shot_transform.translation.y > WINDOW_H - INFO_BAR_H
                {
                    if shot_component.enemy_type == 1 && sounds.enemy_01_shot_counter > 0 {
                        // println!("Shot 01 counter = {}", sounds.enemy_01_shot_counter);
                        sounds.enemy_01_shot_counter -= 1;
                        if sounds.enemy_01_shot_counter == 0 {
                            sfx01.stop();
                        }
                    }

                    if shot_component.enemy_type == 5 && sounds.enemy_05_shot_counter > 0 {
                        sounds.enemy_05_shot_counter -= 1;
                        if sounds.enemy_05_shot_counter == 0 {
                            sfx05.stop();
                        }
                    }

                    if shot_component.enemy_type == 6 && sounds.enemy_06_shot_counter > 0 {
                        sounds.enemy_06_shot_counter -= 1;
                        if sounds.enemy_06_shot_counter == 0 {
                            sfx06.stop();
                        }
                    }

                    if shot_component.enemy_type == 8 && sounds.enemy_08_shot_counter > 0 {
                        sounds.enemy_08_shot_counter -= 1;
                        if sounds.enemy_08_shot_counter == 0 {
                            sfx08.stop();
                        }
                    }

                    if shot_component.enemy_type == 9 && sounds.enemy_09_shot_counter > 0 {
                        sounds.enemy_09_shot_counter -= 1;
                        if sounds.enemy_09_shot_counter == 0 {
                            sfx09.stop();
                        }
                    }

                    if let Some(entity) = eopt {
                        commands.entity(entity).despawn_recursive();
                    }

                    continue;
                }

                // no collison, move
                shot_transform.translation = target;
            }
        }
    }
}
