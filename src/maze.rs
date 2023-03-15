use bevy::prelude::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::usize;
use rand::Rng;
use bevy_kira_audio::{AudioControl, AudioChannel};

use crate::audio::{Sounds, DamageChannel, Shooting01Channel, Shooting05Channel, Shooting06Channel, Shooting08Channel, Shooting09Channel};
use crate::enemies::{EnemiesImages, Enemy, EnemyComponent, EnemyShotComponent, ENEMY_COLORS, ENEMY_NN_SIZE, ENEMY_07_SIZE, ENEMY_18_SIZE, ENEMY_19_SIZE, 
    EnemyType01, EnemyType02, EnemyType03, EnemyType05, EnemyType06, EnemyType07, EnemyType08, EnemyType09, EnemyType10,
    EnemyType11, EnemyType12, EnemyType13, EnemyType14, EnemyType15, EnemyType16, EnemyType17, EnemyType18, EnemyType19, EnemyType20, FellowEnemy, 
    EnemyType03Fellow, EnemyType10Fellow};
use crate::explosions::{Fragment, Boom, FlashEffect};
use crate::items::{Item, ITEM_W, ITEM_H, ITEM_W2, ITEM_H2, ItemsImages, ItemComponent, ItemAnimationTimer, FellowItem};
use crate::player::{Player, CannonShot, PlayerComponent};
use crate::{GameState, WINDOW_H, INFO_BAR_H, GameDirection};
use crate::special::{SpecialBall, SpecialStar};

pub const MAZE_ROWS: usize = 8;
pub const MAZE_COLS: usize = 16;

pub const START_ROOM_INDEX: usize = 0;
// pub const START_ROOM_INDEX: usize = (1 * MAZE_COLS) + 14; // carrier 5, 8, 9
// pub const START_ROOM_INDEX: usize = (2 * MAZE_COLS) + 11; // carrier 2
// pub const START_ROOM_INDEX: usize = (6 * MAZE_COLS) + 3; // fellow item test

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<RoomChangeEvent>()
        .add_startup_system_to_stage(StartupStage::PreStartup, load_wall_images)
        .add_startup_system_to_stage(StartupStage::PreStartup, create_maze_resource)
        .add_system_set(SystemSet::on_exit(GameState::Menu)
            .with_system(load_maze).label("LoadMazeSystem"))
        .add_system_set(SystemSet::on_enter(GameState::Game)
            .with_system(spawn_current_room).after("LoadMazeSystem"))
        .add_system_set(SystemSet::on_update(GameState::Game)
            .with_system(change_room).after("PlayerMovementSystem")
            .with_system(change_room_sounds).after("PlayerMovementSystem")
            .with_system(change_room_special_ball).after("PlayerMovementSystem")
            .with_system(change_room_special_star).after("PlayerMovementSystem")
            .with_system(check_bases_count)
        )
        .add_system_set(SystemSet::on_exit(GameState::Game)
            .with_system(despawn_room)
        );
    }
}

pub struct WallImages {
    wall_images: Vec<Handle<Image>>,
}

pub struct Wall {
    id: usize,
    posx: f32,
    posy: f32
}

#[derive(Component, Copy, Clone)]
pub struct WallComponent {
    pub position: Vec3,
    pub size: Vec2,
}

pub struct Room {
    pub walls: Vec<Wall>,
    pub enemies: Vec<Enemy>,
    pub enemies_from_10: Vec<Enemy>,
    pub from_10_seq: usize,
    pub items: Vec<Item>,
}

impl Room {
    pub fn clear(&mut self) {
        self.walls.clear();
        self.enemies.clear();
        self.enemies_from_10.clear();
        self.items.clear();
    }
}

pub struct Maze {
    pub loaded: bool,
    pub rooms: Vec<Room>,
    pub current_room: usize,
    pub score: u16,
    pub bases: u8,
    pub bases_total: u8,
}

impl Maze {
    pub fn clear(&mut self) {
        for room in self.rooms.iter_mut() {
            room.clear();
        }
        self.rooms.clear();
        self.loaded = false;
        self.current_room = START_ROOM_INDEX;
        self.score = 0;
        self.bases = 0;
    }
}

pub struct RoomChangeEvent {
    pub row: usize,
    pub col: usize
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn load_wall_images(mut commands: Commands, assets: Res<AssetServer>) {
    let mut wall_images = WallImages {
        wall_images: Vec::new(),
    };

    wall_images.wall_images.push(assets.load("images/walls/wall_00.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_01.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_02.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_03.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_04.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_05.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_06.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_07.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_08.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_09.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_10.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_11.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_12.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_13.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_14.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_15.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_16.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_17.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_18.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_19.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_20.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_21.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_22.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_23.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_24.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_25.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_26.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_27.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_28.png"));
    wall_images.wall_images.push(assets.load("images/walls/wall_29.png"));

    commands.insert_resource(wall_images);
}

fn create_maze_resource(mut commands: Commands) {
    let maze = Maze { 
        loaded: false, 
        rooms: Vec::new(), 
        current_room: START_ROOM_INDEX, 
        score: 0, 
        bases: 0,
        bases_total:0,
    };
    commands.insert_resource(maze);
}

fn load_walls(file_name: &str, room: &mut Room) {
    // println!("- Load walls: {}", file_name);

    let file = File::open(file_name).expect("Failed to open file!");
    let mut line = String::new();
    let mut reader = BufReader::new(file);

    // count
    reader.read_line(&mut line).expect("Error reading file!");
    let count = line.trim().parse::<i16>().unwrap();
    line.clear();

    // ;
    reader.read_line(&mut line).expect("Error reading file!");
    line.clear();

    for _ in 0..count {
        // type
        reader.read_line(&mut line).expect("Error reading file!");
        let id = line.trim().parse::<usize>().unwrap();
        line.clear();

        // x
        reader.read_line(&mut line).expect("Error reading file!");
        let posx = line.trim().parse::<f32>().unwrap();
        line.clear();

        // y
        reader.read_line(&mut line).expect("Error reading file!");
        let posy = line.trim().parse::<f32>().unwrap();
        line.clear();

        // ;
        reader.read_line(&mut line).expect("Error reading file!");
        line.clear();

        // println!("Wall: {}, {}, {}", id, posx, posy);
        room.walls.push(Wall { id, posx, posy });
    }
}

fn get_enemy_health(enemy_type: usize) -> i16 {
    match enemy_type {
        0 | 4 => 200,
        1 => 60,
        2 | 5 | 6 | 7 | 9 | 10 => 90,
        3 => 30,
        8 => 20,
        11 | 12 | 13 | 14 | 15 | 16 | 17 | 20 => 10,
        18 | 19 => 50,
        _ => panic!("Unexpected enemy id!")
    }
}

fn get_enemy_shooting_cooldown(enemy_type: usize) -> u16 {
    match enemy_type {
        1 | 3 | 5 | 6 | 13 => 500,
        2 => 2000,
        7 => 1000,
        10 => 10000,
        _ => u16::MAX
    }
}

fn get_enemy_direction(enemy_type: usize, enemy_subtype: usize) -> GameDirection {
    if enemy_type == 1 || enemy_type == 8 { 
        if enemy_subtype == 0 {
            return GameDirection::Left;
        }
        else {
            return GameDirection::Right;
        }
    }
    GameDirection::None
}

fn load_enemies(file_name: &str, room: &mut Room, room_seq: usize, base_count: &mut u8) {
    // println!("- Load enemies: {}", file_name);

    let mut rng = rand::thread_rng();

    let file = File::open(file_name).expect("Failed to open file!");
    let mut line = String::new();
    let mut reader = BufReader::new(file);

    // count
    reader.read_line(&mut line).expect("Error reading file!");
    let count = line.trim().parse::<i16>().unwrap();
    line.clear();

    // ;
    reader.read_line(&mut line).expect("Error reading file!");
    line.clear();

    for enemy_seq in 0..count {
        let color_idx = rng.gen_range(0..ENEMY_COLORS.len());

        // load enemy

        // x
        reader.read_line(&mut line).expect("Error reading file!");
        let posx = line.trim().parse::<f32>().unwrap();
        line.clear();

        // y
        reader.read_line(&mut line).expect("Error reading file!");
        let posy = line.trim().parse::<f32>().unwrap();
        line.clear();

        // type
        reader.read_line(&mut line).expect("Error reading file!");
        let id = line.trim().parse::<usize>().unwrap();
        line.clear();

        // subtype
        reader.read_line(&mut line).expect("Error reading file!");
        let subid = line.trim().parse::<usize>().unwrap();
        line.clear();

        // ;
        reader.read_line(&mut line).expect("Error reading file!");
        line.clear();

        if id == 0 {
            *base_count += 1;
        }

        // if enemy 20, load fellow
        let fellow_enemy: Option<FellowEnemy>;
        let fellow_item: Option<FellowItem>;
        if id == 20 {
            // fellow type
            reader.read_line(&mut line).expect("Error reading file!");
            let fellow_type = line.trim().parse::<u8>().unwrap();
            line.clear();

            // x
            reader.read_line(&mut line).expect("Error reading file!");
            let posx = line.trim().parse::<f32>().unwrap();
            line.clear();

            // y
            reader.read_line(&mut line).expect("Error reading file!");
            let posy = line.trim().parse::<f32>().unwrap();
            line.clear();

            // type
            reader.read_line(&mut line).expect("Error reading file!");
            let id = line.trim().parse::<usize>().unwrap();
            line.clear();

            // subtype
            reader.read_line(&mut line).expect("Error reading file!");
            let subid = line.trim().parse::<usize>().unwrap();
            line.clear();

            // ;
            reader.read_line(&mut line).expect("Error reading file!");
            line.clear();

            // in case base is carried (original maze does not have any of these)
            if id == 0 {
                *base_count += 1;
            }

            if fellow_type == 0 {
                // println!("Carrier ---> enemy {}, {}", id, subid);
                // fellow enemy
                fellow_enemy = Some(FellowEnemy {
                    health: get_enemy_health(id),
                    enemy_type: id,
                    enemy_subtype: subid,
                    first: true,
                    posx,
                    posy, 
                    color: ENEMY_COLORS[color_idx],
                    shooting_cooldown: 0,
                    shooting_cooldown_max: get_enemy_shooting_cooldown(id),
                    direction: get_enemy_direction(id, subid),
                });

                // no item
                fellow_item = None;
            }
            else {
                // fellow item
                fellow_item = Some(FellowItem { 
                    posx, posy, item_type: id, collected: false
                });
                // no enemy
                fellow_enemy = None;
            }
        }
        else {
            // not 20
            fellow_enemy = None;
            fellow_item = None;
        }

        // enemy 15 and 16 have constant velocity 
        // (other movable enemies calculates their velocity in corresponding enemy_NN_movement system)
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

        room.enemies.push(Enemy { 
            health: get_enemy_health(id),
            room_seq, 
            enemy_seq: enemy_seq as usize, 
            enemy_type: id, 
            enemy_subtype: subid, 
            first: true,
            posx, 
            posy, 
            color: ENEMY_COLORS[color_idx],
            velocity,
            shooting_cooldown: 0,
            shooting_cooldown_max: get_enemy_shooting_cooldown(id),
            direction: get_enemy_direction(id, subid),
            is_from_10: false,
            fellow_enemy,
            fellow_item,
        });
    } // end for
}

fn load_items(file_name: &str, room: &mut Room, room_seq: usize) {
    // println!("- Load items: {}", file_name);

    let file = File::open(file_name).expect("Failed to open file!");
    let mut line = String::new();
    let mut reader = BufReader::new(file);

    // count
    reader.read_line(&mut line).expect("Error reading file!");
    let count = line.trim().parse::<i16>().unwrap();
    line.clear();

    // ;
    reader.read_line(&mut line).expect("Error reading file!");
    line.clear();

    for item_seq in 0..count {
        // x
        reader.read_line(&mut line).expect("Error reading file!");
        let posx = line.trim().parse::<f32>().unwrap();
        line.clear();

        // y
        reader.read_line(&mut line).expect("Error reading file!");
        let posy = line.trim().parse::<f32>().unwrap();
        line.clear();

        // type
        reader.read_line(&mut line).expect("Error reading file!");
        let id = line.trim().parse::<usize>().unwrap();
        line.clear();

        // ;
        reader.read_line(&mut line).expect("Error reading file!");
        line.clear();

        room.items.push(Item {
            posx,
            posy,
            item_type: id,
            collected: false,
            room_seq,
            item_seq: item_seq as usize,
        });
    }
}

fn load_room(row: usize, col: usize, room_seq: usize, base_count: &mut u8) -> Room {
    // println!("Load room: {},{}", row, col);

    let mut room = Room { 
        walls: Vec::new(),
        enemies: Vec::new(),
        enemies_from_10: Vec::new(),
        from_10_seq: 0,
        items: Vec::new(),
    };

    load_walls(format!("assets/data/rooms/room{}{}.txt", row, col).as_str(), &mut room);
    load_enemies(format!("assets/data/enemies/enemy{}{}.txt", row, col).as_str(), &mut room, room_seq, base_count);
    load_items(format!("assets/data/items/item{}{}.txt", row, col).as_str(), &mut room, room_seq);

    room
}

fn load_maze(mut maze: ResMut<Maze>) {
    if !maze.loaded {
        println!("Load maze");
        
        let mut room_seq: usize = 0;
        let mut base_count: u8 = 0;

        for row in 0..MAZE_ROWS {
            for col in 0..MAZE_COLS {
                maze.rooms.push(load_room(row, col, room_seq, &mut base_count));
                room_seq += 1;
            }
        }
        println!("Base count = {}", base_count);
        // maze.bases = 1; // for debug
        maze.bases = base_count;
        maze.bases_total = base_count;

        maze.loaded = true;
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    enemy: &Enemy,
    texture_atlas: Handle<TextureAtlas>,
    size: Vec2,
    position: Vec3,
    index: usize
    ) -> Entity
{
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas,
        sprite: TextureAtlasSprite {
            color: enemy.color,
            index,
            custom_size: Some(size),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(EnemyComponent { room_seq: enemy.room_seq, enemy_seq: enemy.enemy_seq, is_from_10: enemy.is_from_10 })
    .id()
}

pub fn spawn_fellow_enemy(
    commands: &mut Commands,
    fellow_enemy: &FellowEnemy,
    texture_atlas: Handle<TextureAtlas>,
    size: Vec2,
    position: Vec3,
    index: usize
    ) -> Entity
{
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas,
        sprite: TextureAtlasSprite {
            color: fellow_enemy.color,
            index,
            custom_size: Some(size),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .id()
}

pub fn spawn_fellow_item(
    commands: &mut Commands,
    texture: Handle<Image>,
    position: Vec3,) -> Entity
{
    commands.spawn_bundle(SpriteBundle {
        texture,
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(Vec2::new(ITEM_W, ITEM_H)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ItemAnimationTimer(Timer::from_seconds(0.08, true)))
    .insert(ItemComponent { item_seq: usize::MAX, room_seq: usize::MAX, color_index: 0 })
    .id()
}

fn spawn_room(
    commands: &mut Commands, 
    walls: &Vec<Wall>, 
    wall_images: &Res<WallImages>, 
    enemies: &Vec<Enemy>,
    enemies_from_10: &Vec<Enemy>,
    enemies_images: &Res<EnemiesImages>,
    items: &Vec<Item>,
    items_images: &Res<ItemsImages>,
    assets: &Res<Assets<Image>>) 
{
    // spawn walls

    for wall in walls.iter() {
        let imgopt = assets.get(&wall_images.wall_images[wall.id]);
        if let Some(img) = imgopt {
            let imgw = img.texture_descriptor.size.width as f32;
            let imgh = img.texture_descriptor.size.height as f32;

            commands.spawn_bundle(SpriteBundle {
                texture: wall_images.wall_images[wall.id].clone(),
                sprite: Sprite { 
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(imgw, imgh)),
                    flip_x: false,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(wall.posx + imgw/2.0, WINDOW_H - wall.posy - INFO_BAR_H - imgh/2.0, 50.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(WallComponent {
                position: Vec3::new(wall.posx + imgw/2.0, WINDOW_H - wall.posy - INFO_BAR_H - imgh/2.0, 50.0),
                size: Vec2::new(imgw - 1.0, imgh - 1.0),
            });
        }
    }

    // spawn enemies

    for enemy in enemies.iter() {
        if enemy.health == 0 {
            continue;
        }

        let spawn_x: f32;
        let spawn_y: f32;
        if enemy.first {
            spawn_x = enemy.posx + 25.0;
            spawn_y = WINDOW_H - INFO_BAR_H - enemy.posy - 25.0;
        }
        else {
            spawn_x = enemy.posx;
            spawn_y = enemy.posy;
        }

        // animated enemy gets AnimationTimer - general handler = enemies::animate_sprite
        // enemy which is moving or/and shooting gets EnemyTypeN component - special handler = enemies::enemy_N_movement

        if enemy.enemy_type == 0 {
            spawn_enemy(commands, &enemy, enemies_images.enemy_00.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
        }
        else if enemy.enemy_type == 1 {
            if enemy.enemy_subtype == 0 {
                let e = spawn_enemy(commands, &enemy, enemies_images.enemy_01.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
                commands.entity(e).insert(EnemyType01);
            }
            else {
                let e = spawn_enemy(commands, &enemy, enemies_images.enemy_01.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 1);
                commands.entity(e).insert(EnemyType01);
            }
        }
        else if enemy.enemy_type == 2 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_02.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e).insert(EnemyType02);
        }
        else if enemy.enemy_type == 3 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_03.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e).insert(EnemyType03);
        }
        else if enemy.enemy_type == 4 {
            spawn_enemy(commands, &enemy, enemies_images.enemy_04.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
        }
        else if enemy.enemy_type == 5 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_05.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e).insert(EnemyType05);
        }
        else if enemy.enemy_type == 6 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_06.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e).insert(EnemyType06);
        }
        else if enemy.enemy_type == 7 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_07.clone(), ENEMY_07_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType07);
        }
        else if enemy.enemy_type == 8 {
            if enemy.enemy_subtype == 0 {
                let e = spawn_enemy(commands, &enemy, enemies_images.enemy_08.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
                commands.entity(e).insert(EnemyType08);
            }
            else {
                let e = spawn_enemy(commands, &enemy, enemies_images.enemy_08.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 1);
                commands.entity(e).insert(EnemyType08);
            }
        }
        else if enemy.enemy_type == 9 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_09.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType09);
        }
        else if enemy.enemy_type == 10 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_10.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e).insert(EnemyType10);
        }
        else if enemy.enemy_type == 11 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_11.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e).insert(EnemyType11);
        }
        else if enemy.enemy_type == 12 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_12.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType12);
        }
        else if enemy.enemy_type == 13 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_13_left.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType13);
        }
        else if enemy.enemy_type == 14 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_14_left.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType14);
        }
        else if enemy.enemy_type == 15 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_15.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType15);
        }
        else if enemy.enemy_type == 16 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_16.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType16);
        }
        else if enemy.enemy_type == 17 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_17_left.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType17);
        }
        else if enemy.enemy_type == 18 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_18.clone(), ENEMY_18_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e).insert(EnemyType18);
        }
        else if enemy.enemy_type == 19 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_19.clone(), ENEMY_19_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType19);
        }
        else if enemy.enemy_type == 20 {
            let e20: Entity;
            if enemy.enemy_subtype == 0 {
                e20 = spawn_enemy(commands, &enemy, enemies_images.enemy_20_v1.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
                commands.entity(e20)
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
                .insert(EnemyType20);
            }
            else {
                e20 = spawn_enemy(commands, &enemy, enemies_images.enemy_20_v2.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
                commands.entity(e20)
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
                .insert(EnemyType20);
            }
            // spawn fellow enemy
            if let Some(fellow_enemy) = &enemy.fellow_enemy {
                if fellow_enemy.enemy_type == 0 {
                    let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_00.clone(), ENEMY_NN_SIZE, 
                        Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 0);
                    commands.entity(e20).add_child(e);
                }
                else if fellow_enemy.enemy_type == 1 {
                    // println!("Spawn fellow 1, {}", fellow_enemy.enemy_subtype);
                    if fellow_enemy.enemy_subtype == 0 {
                        let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_01.clone(), ENEMY_NN_SIZE, 
                            Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 0);
                        commands.entity(e20).add_child(e);
                    }
                    else {
                        let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_01.clone(), ENEMY_NN_SIZE, 
                            Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 1);
                        commands.entity(e20).add_child(e);
                    }
                }
                else if fellow_enemy.enemy_type == 2 {
                    let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_02.clone(), ENEMY_NN_SIZE, 
                        Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 0);
                    commands.entity(e20).add_child(e);
                }
                else if fellow_enemy.enemy_type == 3 {
                    let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_03.clone(), ENEMY_NN_SIZE, 
                        Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 0);
                    commands.entity(e).insert(EnemyType03Fellow);
                    commands.entity(e20).add_child(e);
                }
                else if fellow_enemy.enemy_type == 4 {
                    let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_04.clone(), ENEMY_NN_SIZE, 
                        Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 0);
                    commands.entity(e20).add_child(e);
                }
                else if fellow_enemy.enemy_type == 5 {
                    let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_05.clone(), ENEMY_NN_SIZE, 
                        Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 0);
                    commands.entity(e20).add_child(e);
                }
                else if fellow_enemy.enemy_type == 7 {
                    let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_07.clone(), ENEMY_07_SIZE, 
                        Vec3::new(0.0, ENEMY_07_SIZE.y + 4.0, 100.0), 0);
                    commands.entity(e).insert(AnimationTimer(Timer::from_seconds(0.1, true)));
                    commands.entity(e20).add_child(e);
                }
                else if fellow_enemy.enemy_type == 8 {
                    if fellow_enemy.enemy_subtype == 0 {
                        let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_08.clone(), ENEMY_NN_SIZE, 
                            Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 0);
                        commands.entity(e20).add_child(e);
                    }
                    else {
                        let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_08.clone(), ENEMY_NN_SIZE, 
                            Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 1);
                        commands.entity(e20).add_child(e);
                    }
                }
                else if fellow_enemy.enemy_type == 9 {
                    let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_09.clone(), ENEMY_NN_SIZE, 
                        Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 0);
                    commands.entity(e).insert(AnimationTimer(Timer::from_seconds(0.1, true)));
                    commands.entity(e20).add_child(e);
                }
                else if fellow_enemy.enemy_type == 10 {
                    let e = spawn_fellow_enemy(commands, fellow_enemy, enemies_images.enemy_10.clone(), ENEMY_NN_SIZE, 
                        Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0), 0);
                    commands.entity(e).insert(EnemyType10Fellow);
                    commands.entity(e20).add_child(e);
                }
            } // end spawn fellow enemy
            // spawn fellow item
            if let Some(fellow_item) = &enemy.fellow_item {
                let e: Entity;
                match fellow_item.item_type {
                    0 => e = spawn_fellow_item(commands, items_images.item_ammo.clone(), Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0)),
                    1 => e = spawn_fellow_item(commands, items_images.item_ball.clone(), Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0)),
                    2 => e = spawn_fellow_item(commands, items_images.item_fuel.clone(), Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0)),
                    3 => e = spawn_fellow_item(commands, items_images.item_missile_down.clone(), Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0)),
                    4 => e = spawn_fellow_item(commands, items_images.item_missile_side.clone(), Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0)),
                    5 => e = spawn_fellow_item(commands, items_images.item_shield.clone(), Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0)),
                    6 => e = spawn_fellow_item(commands, items_images.item_star.clone(), Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0)),
                    7 => e = spawn_fellow_item(commands, items_images.item_random.clone(), Vec3::new(0.0, ENEMY_NN_SIZE.y, 100.0)),
                    _ => panic!("Unexpected fellow item type!")
                }
                commands.entity(e20).add_child(e);
            }
        } // end 20
    } // end enemies

    for enemy in enemies_from_10.iter() {
        if enemy.health == 0 {
            continue;
        }

        let spawn_x: f32;
        let spawn_y: f32;
        if enemy.first {
            spawn_x = enemy.posx + 25.0;
            spawn_y = WINDOW_H - INFO_BAR_H - enemy.posy - 25.0;
        }
        else {
            spawn_x = enemy.posx;
            spawn_y = enemy.posy;
        }

        if enemy.enemy_type == 11 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_11.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e).insert(EnemyType11);
        }
        else if enemy.enemy_type == 12 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_12.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType12);
        }
        else if enemy.enemy_type == 13 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_13_left.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType13);
        }
        else if enemy.enemy_type == 14 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_14_left.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType14);
        }
        else if enemy.enemy_type == 15 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_15.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType15);
        }
        else if enemy.enemy_type == 16 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_16.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType16);
        }
        else if enemy.enemy_type == 17 {
            let e = spawn_enemy(commands, &enemy, enemies_images.enemy_17_left.clone(), ENEMY_NN_SIZE, Vec3::new(spawn_x, spawn_y, 100.0), 0);
            commands.entity(e)
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(EnemyType17);
        }
    } // end from 10

    // spawn items
    for item in items.iter() {
        if item.collected {
            continue;
        }

        commands.spawn_bundle(SpriteBundle {
            texture: match item.item_type {
                0 => items_images.item_ammo.clone(),
                1 => items_images.item_ball.clone(),
                2 => items_images.item_fuel.clone(),
                3 => items_images.item_missile_down.clone(),
                4 => items_images.item_missile_side.clone(),
                5 => items_images.item_shield.clone(),
                6 => items_images.item_star.clone(),
                7 => items_images.item_random.clone(),
                _ => panic!("Unexpected item type!")
            },
            sprite: Sprite { 
                color: Color::WHITE,
                custom_size: Some(Vec2::new(ITEM_W, ITEM_H)),
                flip_x: false,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(item.posx + ITEM_W2, WINDOW_H - INFO_BAR_H - item.posy - ITEM_H2 - 6.0, 50.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ItemAnimationTimer(Timer::from_seconds(0.08, true)))
        .insert(ItemComponent { item_seq: item.item_seq, room_seq: item.room_seq, color_index: 0 });
    }
}

fn spawn_current_room(
    mut commands: Commands, 
    maze: Res<Maze>, 
    wall_images: Res<WallImages>, 
    enemies_images: Res<EnemiesImages>, 
    items_images: Res<ItemsImages>,
    assets: Res<Assets<Image>>) 
{
    let walls = &maze.rooms[maze.current_room].walls;
    let enemies = &maze.rooms[maze.current_room].enemies;
    let enemies_from_10 = &maze.rooms[maze.current_room].enemies_from_10;
    let items = &maze.rooms[maze.current_room].items;
    spawn_room(&mut commands, walls, &wall_images, &enemies, &enemies_from_10, &enemies_images, items, &items_images, &assets);
}

fn change_room(
    mut commands: Commands, 
    mut change_room_event: EventReader<RoomChangeEvent>,
    walls_query: Query<Entity, With<WallComponent>>,
    enemies_query: Query<Entity, With<EnemyComponent>>,
    enemies_shots_query: Query<Entity, With<EnemyShotComponent>>,
    booms_query: Query<Entity, With<Boom>>,
    fragments_query: Query<Entity, With<Fragment>>,
    // flash_query: Query<Entity, With<FlashEffect>>,
    items_query: Query<Entity, With<ItemComponent>>,
    mut player: ResMut<Player>,
    cannon_query: Query<Entity, With<CannonShot>>,
    mut maze: ResMut<Maze>, 
    wall_images: Res<WallImages>, 
    enemies_images: Res<EnemiesImages>,
    items_images: Res<ItemsImages>,
    assets: Res<Assets<Image>>) 
{
    for event in change_room_event.iter() {
        println!("Change room -> {},{}", event.row, event.col);
        
        // despawn current room
        for entity in walls_query.iter() {
            commands.entity(entity).despawn_recursive(); // despawn walls
        }
        for entity in enemies_query.iter() {
            commands.entity(entity).despawn_recursive(); // despawn enemies
        }
        for entity in enemies_shots_query.iter() {
            commands.entity(entity).despawn_recursive(); // despawn enemy shots
        }
        for entity in booms_query.iter() {
            commands.entity(entity).despawn_recursive(); // despawn explosions
        }
        for entity in fragments_query.iter() {
            commands.entity(entity).despawn_recursive(); // despawn fragments
        }
        // for entity in flash_query.iter() {
        //     commands.entity(entity).despawn_recursive(); // despawn flash effect
        // }
        for entity in items_query.iter() {
            commands.entity(entity).despawn_recursive(); // despawn items
        }

        let index = (event.row * MAZE_COLS) + event.col;
        {
            maze.current_room = index;
            let walls = &maze.rooms[index].walls;
            let enemies = &maze.rooms[index].enemies;
            let enemies_from_10 = &maze.rooms[index].enemies_from_10;
            let items = &maze.rooms[index].items;
            spawn_room(&mut commands, walls, &wall_images, enemies, enemies_from_10, &enemies_images, items, &items_images, &assets);
        }

        let enemies = &mut maze.rooms[index].enemies;
        for enemy in enemies.iter_mut() {
            if enemy.enemy_type != 10 {
                enemy.shooting_cooldown = 0;
            }
        }

        if player.shooting_cannon {
            // despawn cannon shot
            for cannon_entity in cannon_query.iter() {
                commands.entity(cannon_entity).despawn_recursive();
            }
            player.shooting_cannon = false;
        }
        player.changing_room = false;

        // println!("Change room END");
    }
}

fn change_room_sounds(
    mut change_room_event: EventReader<RoomChangeEvent>,
    sfx_dmg: Res<AudioChannel<DamageChannel>>, 
    sfx01: Res<AudioChannel<Shooting01Channel>>, 
    sfx05: Res<AudioChannel<Shooting05Channel>>, 
    sfx06: Res<AudioChannel<Shooting06Channel>>, 
    sfx08: Res<AudioChannel<Shooting08Channel>>, 
    sfx09: Res<AudioChannel<Shooting09Channel>>, 
    mut sounds: ResMut<Sounds>,) 
{
    for _ in change_room_event.iter() {
        // println!("change_room_sounds");
        sfx_dmg.stop();
        sounds.enemy_01_shot_counter = 0;
        sounds.enemy_05_shot_counter = 0;
        sounds.enemy_06_shot_counter = 0;
        sounds.enemy_08_shot_counter = 0;
        sounds.enemy_09_shot_counter = 0;
        sfx01.stop();
        sfx05.stop();
        sfx06.stop();
        sfx08.stop();
        sfx09.stop();
    }
}

fn change_room_special_ball(
    mut change_room_event: EventReader<RoomChangeEvent>,
    mut special_ball_query: Query<&mut Transform, (With<SpecialBall>, Without<PlayerComponent>)>,
    player_query: Query<&Transform, With<PlayerComponent>>) 
{
    for _ in change_room_event.iter() {
        let player_transform = player_query.single();
        for mut ball_transform in special_ball_query.iter_mut() {
            // println!("Change room: ball -> player ({:?})", player_transform.translation);
            ball_transform.translation.x = player_transform.translation.x;
            ball_transform.translation.y = player_transform.translation.y;
            // println!("{:?}", ball_transform.translation);
        }
    }
}

fn change_room_special_star(
    mut change_room_event: EventReader<RoomChangeEvent>,
    mut special_star_query: Query<&mut Transform, (With<SpecialStar>, Without<PlayerComponent>)>,
    player_query: Query<&Transform, With<PlayerComponent>>) 
{
    for _ in change_room_event.iter() {
        let player_transform = player_query.single();
        for mut star_transform in special_star_query.iter_mut() {
            // println!("Change room: star -> player ({:?})", player_transform.translation);
            star_transform.translation.x = player_transform.translation.x;
            star_transform.translation.y = player_transform.translation.y;
            // println!("{:?}", star_transform.translation);
        }
    }
}

fn despawn_room(
    mut commands: Commands, 
    walls_query: Query<Entity, With<WallComponent>>,
    enemies_query: Query<Entity, With<EnemyComponent>>,
    enemies_shots_query: Query<Entity, With<EnemyShotComponent>>,
    booms_query: Query<Entity, With<Boom>>,
    fragments_query: Query<Entity, With<Fragment>>,
    flash_query: Query<Entity, With<FlashEffect>>,
    items_query: Query<Entity, With<ItemComponent>>,) 
{
    println!("Despawn room");
    // despawn current room
    for entity in walls_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in enemies_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in enemies_shots_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in booms_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in fragments_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in flash_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in items_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn check_bases_count(maze: Res<Maze>, mut state: ResMut<State<GameState>>) {
    if maze.bases == 0 {
        state.set(GameState::Victory).expect("MAZE: Failed to change state!");
    }
}
