use bevy::prelude::*;
use bevy_kira_audio::{AudioControl, AudioChannel};

use crate::infobar::{SpecialAmmoChange, SpecialChange, ScoreChange, BaseCountChange};
use crate::items::{ITEM_W, ITEM_H};
use crate::{WINDOW_H, WINDOW_W, INFO_BAR_H, GRAVITY, GameState, GameDirection, collision_check, GameKeys};
use crate::enemies::{ENEMY_NN_SIZE, ENEMY_07_SIZE, EnemyComponent};
use crate::maze::{Maze, WallComponent, RoomChangeEvent};
use crate::explosions::ExplosionsImages;
use crate::audio::{Sounds, SfxChannel, DamageChannel};
use crate::special::{SpecialType, SpecialImages};

pub const PLAYER_W: f32 = 99.0;
pub const PLAYER_H: f32 = 48.0;
pub const PLAYER_W2: f32 = PLAYER_W / 2.0;
pub const PLAYER_H2: f32 = PLAYER_H / 2.0;

pub const PLAYER_START_X: f32 = 210.0;
pub const PLAYER_START_Y: f32 = 300.0;
// pub const PLAYER_START_X: f32 = 650.0;  // room 1,14
// pub const PLAYER_START_Y: f32 = 200.0;  // room 1,14

pub const LEFT_EDGE: f32 = PLAYER_W2;
pub const RIGHT_EDGE: f32 = WINDOW_W - PLAYER_W2;
pub const TOP_EDGE: f32 = WINDOW_H - INFO_BAR_H - PLAYER_H2;
pub const BOTTOM_EDGE: f32 = PLAYER_H2;

pub const FLAME_BACK_W: f32 = 43.0;
pub const FLAME_BACK_H: f32 = 35.0;
pub const FLAME_BACK_W2: f32 = FLAME_BACK_W / 2.0;
// pub const FLAME_BACK_H2: f32 = FLAME_BACK_H / 2.0;
pub const FLAME_BACK_OFFSET_X: f32 = PLAYER_W2 + FLAME_BACK_W2;
pub const FLAME_BACK_OFFSET_Y: f32 = 6.0;

pub const FLAME_DOWN_BIG_W: f32 = 32.0;
pub const FLAME_DOWN_BIG_H: f32 = 44.0;
// pub const FLAME_DOWN_BIG_W2: f32 = FLAME_DOWN_BIG_W / 2.0;
pub const FLAME_DOWN_BIG_H2: f32 = FLAME_DOWN_BIG_H / 2.0;
pub const FLAME_DOWN_BIG_OFFSET_X: f32 = 19.0;
pub const FLAME_DOWN_BIG_OFFSET_Y: f32 = -(PLAYER_H2 + FLAME_DOWN_BIG_H2);

pub const FLAME_DOWN_SMALL_W: f32 = 19.0;
pub const FLAME_DOWN_SMALL_H: f32 = 38.0;
// pub const FLAME_DOWN_SMALL_W2: f32 = FLAME_DOWN_SMALL_W / 2.0;
pub const FLAME_DOWN_SMALL_H2: f32 = FLAME_DOWN_SMALL_H / 2.0;
pub const FLAME_DOWN_SMALL_OFFSET_X: f32 = -22.0;
pub const FLAME_DOWN_SMALL_OFFSET_Y: f32 = -(PLAYER_H2 + FLAME_DOWN_SMALL_H2);

pub const SHOT_CANNON_W: f32 = 22.0;
pub const SHOT_CANNON_H: f32 = 6.0;
// pub const SHOT_CANNON_W2: f32 = SHOT_CANNON_W / 2.0;
pub const SHOT_CANNON_H2: f32 = SHOT_CANNON_H / 2.0;

pub const HEALTH_MAX: f32 = 1000.0;
pub const FUEL_MAX: f32 = 1000.0;
pub const AMMO_MAX: u16 = 1000;

pub const FUEL_SUB: f32 = 0.005;

pub const CANNON_DAMAGE: i16 = 10;
pub const DAMAGE_DELAY: u8 = 50;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_player_resources)
        .add_startup_system_to_stage(StartupStage::PreStartup, create_player_resource)
        .add_system_set(SystemSet::on_enter(GameState::Game)
            .with_system(spawn_player)
        )
        .add_system_set(SystemSet::on_update(GameState::Game)
            .with_system(player_movement).label("PlayerMovementSystem")
            .with_system(cannon_shot_movement)
            .with_system(player_vs_enemy)
            .with_system(check_player_status)
        )
        .add_system_set(SystemSet::on_exit(GameState::Game)
            .with_system(despawn_player)
        );
    }
}

pub struct Player {
    pub posx: f32,
    pub posy: f32,
    pub health: f32,
    pub fuel: f32,
    pub ammo: u16,
    pub special_type: SpecialType,
    pub ammo_special: u8,
    direction: GameDirection,
    pub velocity: Vec2,
    pub shooting_cannon: bool,
    pub shooting_special: bool,
    current_room: (usize, usize),
    pub changing_room: bool,
    damage_delay: u8,
    pub is_dead: bool,
    pub color_index: usize,
}

impl Player {
    pub fn clear(&mut self) {
        self.posx = PLAYER_START_X;
        self.posy = PLAYER_START_Y;
        self.health = HEALTH_MAX;
        self.fuel = FUEL_MAX;
        self.ammo = AMMO_MAX;
        self.special_type = SpecialType::Ball;
        self.ammo_special = 4;
        self.direction = GameDirection::Right;
        self.velocity = Vec2::splat(0.0);
        self.shooting_cannon = false;
        self.shooting_special = false;
        self.current_room = (0, 0);
        self.changing_room = false;
        self.damage_delay = DAMAGE_DELAY;
        self.is_dead = false;
        self.color_index = 0;
    }
}

#[derive(Component)]
pub struct PlayerComponent;

#[derive(Component)]
pub struct PlayerFlameBack {
    frame: u8,
    cooldown: u8,
}

#[derive(Component)]
pub struct PlayerFlameDownBig {
    frame: u8,
    cooldown: u8,
}

#[derive(Component)]
pub struct PlayerFlameDownSmall {
    frame: u8,
    cooldown: u8,
}

#[derive(Component)]
pub struct CannonShot {
    velocity: Vec2,
}

pub struct PlayerImages {
    pub ship: Handle<Image>,
    pub flame_back1: Handle<Image>,
    pub flame_back2: Handle<Image>,
    pub flame_big_down1: Handle<Image>,
    pub flame_big_down2: Handle<Image>,
    pub flame_small_down1: Handle<Image>,
    pub flame_small_down2: Handle<Image>,
    pub cannon_shot: Handle<Image>,
}

pub fn load_player_resources(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(PlayerImages {
        ship: assets.load("images/ship/ship.png"),
        flame_back1: assets.load("images/ship/flame_back1.png"),
        flame_back2: assets.load("images/ship/flame_back2.png"),
        flame_big_down1: assets.load("images/ship/flame_big_down1.png"),
        flame_big_down2: assets.load("images/ship/flame_big_down2.png"),
        flame_small_down1: assets.load("images/ship/flame_small_down1.png"),
        flame_small_down2: assets.load("images/ship/flame_small_down2.png"),
        cannon_shot: assets.load("images/ship/cannon.png"),
    });
}

fn create_player_resource(mut commands: Commands) {
    let player = Player {
        posx: PLAYER_START_X,
        posy: PLAYER_START_Y,
        health: HEALTH_MAX,
        fuel: FUEL_MAX,
        ammo: AMMO_MAX,
        special_type: SpecialType::Ball,
        ammo_special: 4,
        direction: GameDirection::Right,
        velocity: Vec2::splat(0.0),
        shooting_cannon: false,
        shooting_special: false,
        current_room: (0, 0),
        changing_room: false,
        damage_delay: DAMAGE_DELAY,
        is_dead: false,
        color_index: 0,
    };
    commands.insert_resource(player);
}

fn spawn_player(mut commands: Commands, player_images: Res<PlayerImages>, mut player: ResMut<Player>) {
    // ship sprite
    commands.spawn_bundle(SpriteBundle {
        texture: player_images.ship.clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
            flip_x: match player.direction {
                GameDirection::Left => true,
                GameDirection::Right => false,
                GameDirection::None => panic!("Unexpected player direction!")
            },
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(player.posx, player.posy, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Player"))
    .insert(PlayerComponent);

    // back flame sprite
    commands.spawn_bundle(SpriteBundle {
        texture: player_images.flame_back1.clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(Vec2::new(FLAME_BACK_W, FLAME_BACK_H)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(player.posx - FLAME_BACK_OFFSET_X, player.posy + FLAME_BACK_OFFSET_Y, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("PlayerFlameBack"))
    .insert(PlayerFlameBack {
        frame: 0,
        cooldown: 0,
    });

    // down big flame sprite
    commands.spawn_bundle(SpriteBundle {
        texture: player_images.flame_big_down1.clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(Vec2::new(FLAME_DOWN_BIG_W, FLAME_DOWN_BIG_H)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(player.posx - FLAME_DOWN_BIG_OFFSET_X, player.posy + FLAME_DOWN_BIG_OFFSET_Y, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("PlayerFlameDownBig"))
    .insert(PlayerFlameDownBig {
        frame: 0,
        cooldown: 0,
    });

    // down small flame sprite
    commands.spawn_bundle(SpriteBundle {
        texture: player_images.flame_small_down1.clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(Vec2::new(FLAME_DOWN_SMALL_W, FLAME_DOWN_SMALL_H)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(player.posx - FLAME_DOWN_SMALL_OFFSET_X, player.posy + FLAME_DOWN_SMALL_OFFSET_Y, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("PlayerFlameDownSmall"))
    .insert(PlayerFlameDownSmall {
        frame: 0,
        cooldown: 0,
    });

    // NOTE: 
    // In case of exit to main menu, cannon and special are despawn, so there is no chance to end the movement and reset the ability to shoot again.
    // So flags must be re-set manualy here.
    // TODO: make cannon and all specials persistent, so they continue movement after resume.
    player.shooting_cannon = false;
    player.shooting_special = false;
}

fn despawn_player(
    mut commands: Commands, 
    player_query: Query<Entity, With<PlayerComponent>>,
    flame_back_query: Query<Entity, With<PlayerFlameBack>>,
    flame_db_query: Query<Entity, With<PlayerFlameDownBig>>,
    flame_ds_query: Query<Entity, With<PlayerFlameDownSmall>>,
    cannon_shot_query: Query<Entity, With<CannonShot>>) 
{
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in flame_back_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in flame_db_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in flame_ds_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in cannon_shot_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_cannon_shot(commands: &mut Commands, player_images: &Res<PlayerImages>, x: f32, y: f32, direction: GameDirection) {
    let velocity = match direction {
        GameDirection::Left => Vec2::new(-500.0, 0.0),
        GameDirection::Right => Vec2::new(500.0, 0.0),
        GameDirection::None => panic!("Unexpected direction!")
    };

    commands.spawn_bundle(SpriteBundle {
        texture: player_images.cannon_shot.clone(),
        sprite: Sprite { 
            color: Color::CYAN,
            custom_size: Some(Vec2::new(SHOT_CANNON_W, SHOT_CANNON_H)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("CannonShot"))
    .insert(CannonShot {
        velocity: velocity,
    });
}

fn player_movement(
    mut commands: Commands,
    mut player: ResMut<Player>,
    mut player_query: Query<
        (&mut Sprite, &mut Transform), 
        (With<PlayerComponent>, Without<PlayerFlameBack>, Without<PlayerFlameDownBig>, Without<PlayerFlameDownSmall>)
        >,
    mut flame_back_query: Query<
        (&mut PlayerFlameBack, &mut Handle<Image>, &mut Sprite, &mut Transform, &mut Visibility), 
        (With<PlayerFlameBack>, Without<PlayerFlameDownBig>, Without<PlayerFlameDownSmall>)
        >,
    mut flame_down_big_query: Query<
        (&mut PlayerFlameDownBig, &mut Handle<Image>, &mut Sprite, &mut Transform, &mut Visibility), 
        (With<PlayerFlameDownBig>, Without<PlayerFlameBack>, Without<PlayerFlameDownSmall>)
        >,
    mut flame_down_small_query: Query<
        (&mut PlayerFlameDownSmall, &mut Handle<Image>, &mut Sprite, &mut Transform, &mut Visibility), 
        (With<PlayerFlameDownSmall>, Without<PlayerFlameBack>, Without<PlayerFlameDownBig>)
        >,
    walls_query: Query<&WallComponent>,
    mut change_room_event: EventWriter<RoomChangeEvent>,
    mut change_special_ammo_event: EventWriter<SpecialAmmoChange>,
    images: Res<PlayerImages>,
    keyboard: Res<Input<KeyCode>>,
    sfx: Res<AudioChannel<SfxChannel>>, sounds: Res<Sounds>,
    special_images: Res<SpecialImages>,
    time: Res<Time>,
    game_input: Res<GameKeys>) 
{
    if player.changing_room {
        // println!("player changing room...");
        return;
    }

    let (mut player_sprite, mut player_transform) = player_query.single_mut();

    let (mut flame_back, 
         mut flame_back_texture, 
         mut flame_back_sprite, 
         mut flame_back_transform,
         mut flame_back_visibility) = flame_back_query.single_mut();

    let (mut flame_down_big, 
         mut flame_down_big_texture, 
         mut flame_down_big_sprite, 
         mut flame_down_big_transform,
         mut flame_down_big_visibility) = flame_down_big_query.single_mut();

    let (mut flame_down_small, 
         mut flame_down_small_texture, 
         mut flame_down_small_sprite, 
         mut flame_down_small_transform,
         mut flame_down_small_visibility) = flame_down_small_query.single_mut();

    let mut force = Vec2::splat(0.0);

    // gravity
    force += GRAVITY * time.delta_seconds();

    // friction

    if player.velocity.x > 0.0 {
        force += Vec2::new(-100.0, 0.0) * time.delta_seconds();
    }
    if player.velocity.x < 0.0 {
        force += Vec2::new(100.0, 0.0) * time.delta_seconds();
    }

    let mut horiz_key = false;
    let mut up_key = false;

    // apply force from directional keys (left / right / up)

    if keyboard.pressed(game_input.left) && player.fuel > 0.0 {
        // println!("LEFT");
        player.direction = GameDirection::Left;
        player.fuel -= FUEL_SUB;
        player_sprite.flip_x = true;
        force += Vec2::new(-200.0, 0.0) * time.delta_seconds();
        horiz_key = true;
    }

    if keyboard.pressed(game_input.right) && player.fuel > 0.0 {
        // println!("RIGHT");
        player.direction = GameDirection::Right;
        player.fuel -= FUEL_SUB;
        player_sprite.flip_x = false;
        force += Vec2::new(200.0, 0.0) * time.delta_seconds();
        horiz_key = true;
    }

    if keyboard.pressed(game_input.up) && player.fuel > 0.0 {
        // println!("UP");
        player.fuel -= FUEL_SUB;
        force += Vec2::new(0.0, 200.0) * time.delta_seconds();
        up_key = true;
    }

    // special shooting
    if keyboard.just_pressed(game_input.down) && !player.shooting_special && player.ammo_special > 0 {
        sfx.play(sounds.special_launch.clone());
        player.ammo_special -= 1;
        player.shooting_special = true;
        match player.special_type {
            SpecialType::Ball => crate::special::spawn_special_ball(&mut commands, &special_images, 
                player_transform.translation.x, player_transform.translation.y, player.direction),
            SpecialType::MissileDown => crate::special::spawn_special_missile_down(&mut commands, &special_images, 
                player_transform.translation.x, player_transform.translation.y - 10.0),
            SpecialType::MissileSide => crate::special::spawn_special_missile_side(&mut commands, &special_images, 
                player_transform.translation.x, player_transform.translation.y - 10.0, player.direction),
            SpecialType::Star => crate::special::spawn_special_star(&mut commands, &special_images, 
                player_transform.translation.x, player_transform.translation.y),
        }
        change_special_ammo_event.send(SpecialAmmoChange(player.ammo_special));
    }

    // cannon shooting
    if keyboard.pressed(game_input.fire) && !player.shooting_cannon && player.ammo > 0 {
        sfx.play(sounds.cannon_shot.clone());
        player.shooting_cannon = true;
        player.ammo -= 1;
        match player.direction {
            GameDirection::Left => {
                spawn_cannon_shot(&mut commands, &images, 
                    player_transform.translation.x - 15.0, 
                    player_transform.translation.y - SHOT_CANNON_H2, 
                    player.direction);
            },
            GameDirection::Right => {
                spawn_cannon_shot(&mut commands, &images, 
                    player_transform.translation.x + 15.0, 
                    player_transform.translation.y - SHOT_CANNON_H2, 
                    player.direction);
            },
            GameDirection::None => panic!("Unexpected direction!")
        }
    }

    // calculate final velocity

    player.velocity += force;
    player.velocity.x = player.velocity.x.clamp(-400.0, 400.0);
    player.velocity.y = player.velocity.y.clamp(-400.0, 400.0);
    // println!("velocity: {:?}", player.velocity);

    // calculate new position

    let mut position = Vec2::splat(0.0);
    position += player.velocity;
    position *= time.delta_seconds();

    let target = player_transform.translation + Vec3::new(position.x, 0.0, 0.0);
    if !walls_query.iter().any(|&wall| collision_check(
        target, Vec2::new(PLAYER_W, PLAYER_H), 
        wall.position, wall.size)) 
    {
        player_transform.translation = target;
    }
    else {
        // println!("X collision");
        player.velocity.x *= -0.25;
    }

    let target = player_transform.translation + Vec3::new(0.0, position.y, 0.0);
    if !walls_query.iter().any(|&wall| collision_check(
        target, Vec2::new(PLAYER_W, PLAYER_H), 
        wall.position, wall.size)) 
    {
        player_transform.translation = target;
    }
    else {
        // println!("Y collision");
        player.velocity.y *= -0.25;
    }
    // println!("position: {:?}", transform.translation);

    // check movement LEFT
    if player.velocity.x < 0.0 && player_transform.translation.x < LEFT_EDGE && !player.changing_room {
        // println!("Exit room to LEFT");
        player_transform.translation.x = RIGHT_EDGE - 1.0;
        player.current_room.1 -= 1;
        player.changing_room = true;
        change_room_event.send(RoomChangeEvent { row: player.current_room.0, col: player.current_room.1});
    }
    else
    // check movement RIGHT
    if player.velocity.x > 0.0 && player_transform.translation.x > RIGHT_EDGE && !player.changing_room {
        // println!("Exit room to RIGHT");
        player_transform.translation.x = LEFT_EDGE + 1.0;
        player.current_room.1 += 1;
        player.changing_room = true;
        change_room_event.send(RoomChangeEvent { row: player.current_room.0, col: player.current_room.1});
    }
    else
    // check movemnt UP
    if player.velocity.y > 0.0 && player_transform.translation.y > TOP_EDGE && !player.changing_room {
        // println!("Exit room to UP");
        player_transform.translation.y = BOTTOM_EDGE - 1.0;
        player.current_room.0 -= 1;
        player.changing_room = true;
        change_room_event.send(RoomChangeEvent { row: player.current_room.0, col: player.current_room.1});
    }
    else
    // check movement DOWN
    if player.velocity.y < 0.0 && player_transform.translation.y < BOTTOM_EDGE && !player.changing_room {
        // println!("Exit room to DOWN");
        player_transform.translation.y = TOP_EDGE + 1.0;
        player.current_room.0 += 1;
        player.changing_room = true;
        change_room_event.send(RoomChangeEvent { row: player.current_room.0, col: player.current_room.1});
    }

    // println!("Player direction: {:?}", player.direction);

    player.posx = player_transform.translation.x;
    player.posy = player_transform.translation.y;

    // set flames x position
    match player.direction {
        GameDirection::Left => {
            flame_back_transform.translation.x = player_transform.translation.x + FLAME_BACK_OFFSET_X;
            flame_back_sprite.flip_x = true;

            flame_down_big_transform.translation.x = player_transform.translation.x + FLAME_DOWN_BIG_OFFSET_X;
            flame_down_big_sprite.flip_x = true;

            flame_down_small_transform.translation.x = player_transform.translation.x + FLAME_DOWN_SMALL_OFFSET_X;
            flame_down_small_sprite.flip_x = true;
        },
        GameDirection::Right => {
            flame_back_transform.translation.x = player_transform.translation.x - FLAME_BACK_OFFSET_X;
            flame_back_sprite.flip_x = false;

            flame_down_big_transform.translation.x = player_transform.translation.x - FLAME_DOWN_BIG_OFFSET_X;
            flame_down_big_sprite.flip_x = false;

            flame_down_small_transform.translation.x = player_transform.translation.x - FLAME_DOWN_SMALL_OFFSET_X;
            flame_down_small_sprite.flip_x = false;
        },
        GameDirection::None => panic!("Unexpected direction!")
    }

    // set flames y position
    flame_back_transform.translation.y = player_transform.translation.y + FLAME_BACK_OFFSET_Y;
    flame_down_big_transform.translation.y = player_transform.translation.y + FLAME_DOWN_BIG_OFFSET_Y;
    flame_down_small_transform.translation.y = player_transform.translation.y + FLAME_DOWN_SMALL_OFFSET_Y;

    // if left or right key pressed, set back flame visible
    // else, set back flame invisible
    if horiz_key {
        flame_back_visibility.is_visible = true;
        flame_back.cooldown += 1;
        if flame_back.cooldown == 50 {
            if flame_back.frame == 0 {
                flame_back.frame = 1;
                *flame_back_texture = images.flame_back1.clone();
            }
            else {
                flame_back.frame = 0;
                *flame_back_texture = images.flame_back2.clone();
            }
            flame_back.cooldown = 0;
        }
    }
    else {
        flame_back_visibility.is_visible = false;
    }

    // if up key is pressed, set down (big, small) flames visible
    // else, set them invisible
    if up_key {
        flame_down_big_visibility.is_visible = true;
        flame_down_small_visibility.is_visible = true;

        flame_down_big.cooldown += 1;
        flame_down_small.cooldown += 1;
        
        if flame_down_big.cooldown == 50 {
            if flame_down_big.frame == 0 {
                flame_down_big.frame = 1;
                *flame_down_big_texture = images.flame_big_down1.clone();
            }
            else {
                flame_down_big.frame = 0;
                *flame_down_big_texture = images.flame_big_down2.clone();
            }
            flame_down_big.cooldown = 0;
        }

        if flame_down_small.cooldown == 50 {
            if flame_down_small.frame == 0 {
                flame_down_small.frame = 1;
                *flame_down_small_texture = images.flame_small_down1.clone();
            }
            else {
                flame_down_small.frame = 0;
                *flame_down_small_texture = images.flame_small_down2.clone();
            }
            flame_down_small.cooldown = 0;
        }
    }
    else {
        flame_down_big_visibility.is_visible = false;
        flame_down_small_visibility.is_visible = false;
    }

}

fn cannon_shot_movement(
    mut commands: Commands, 
    mut cannon_query: Query<(Option<Entity>, Option<&CannonShot>, Option<&mut Transform>), Without<EnemyComponent>>,
    mut player: ResMut<Player>,
    walls_query: Query<&WallComponent>,
    enemies_query: Query<(Entity, &EnemyComponent, &Transform), With<EnemyComponent>>,
    mut maze: ResMut<Maze>,
    time: Res<Time>,
    explosions_images: Res<ExplosionsImages>,
    sfx: Res<AudioChannel<SfxChannel>>, sounds: Res<Sounds>,
    mut change_score_event: EventWriter<ScoreChange>,
    mut change_base_count_event: EventWriter<BaseCountChange>) 
{
    for (eopt, copt, topt) in cannon_query.iter_mut() {
        if let Some(cannon) = copt {
            if let Some(mut cannon_transform) = topt {

                let target_pos = cannon_transform.translation + Vec3::new(cannon.velocity.x * time.delta_seconds(), 0.0, 0.0);
                let target_size = Vec2::new(SHOT_CANNON_W, SHOT_CANNON_H);
                let mut collided = false;

                // check collision with enemies
                for (enemy_entity, enemy_component, enemy_transform) in enemies_query.iter() {
                    // from 10
                    if enemy_component.is_from_10 {
                        let room = &mut maze.rooms[enemy_component.room_seq];
                        
                        if collision_check(target_pos, target_size, enemy_transform.translation, ENEMY_NN_SIZE) {
                            collided = true;
                            for (idx, mut enemy) in room.enemies_from_10.iter_mut().enumerate() {
                                if enemy.enemy_seq == enemy_component.enemy_seq {
                                    enemy.health -= CANNON_DAMAGE;
                                    if enemy.health <= 0 {
                                        crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                                        sfx.play(sounds.boom.clone());
                                        commands.entity(enemy_entity).despawn_recursive();
                                        room.enemies_from_10.remove(idx);
                                        maze.score += 100;
                                        break;
                                    }
                                    else {
                                        sfx.play(sounds.enemy_damage.clone());
                                    }
                                }
                            }
                        }
                    }
                    else {
                        let mut tmp_score: u16 = 0;
                        {
                            let enemy = &mut maze.rooms[enemy_component.room_seq].enemies[enemy_component.enemy_seq];
                            // 20 + fellow
                            if enemy.enemy_type == 20 {
                                let mut fellow_died = false;
                                let mut carrier_died = false;

                                if enemy.fellow_enemy.is_some() {
                                    // println!("Check cannon collision with fellow");
                                    let fellow = enemy.fellow_enemy.as_mut().unwrap();

                                    // prepare fellow size and position
                                    let mut fellow_size = ENEMY_NN_SIZE;
                                    if fellow.enemy_type == 7 {
                                        fellow_size = ENEMY_07_SIZE;
                                    }
                                    let fellow_pos = Vec3::new(
                                        enemy_transform.translation.x, 
                                        enemy_transform.translation.y + (ENEMY_NN_SIZE.y / 2.0) + (fellow_size.y / 2.0),
                                        0.0);
                                    
                                    // check collision with fellow
                                    if collision_check(target_pos, target_size, fellow_pos, fellow_size) {
                                        collided = true;
                                        fellow.health -= CANNON_DAMAGE;
                                        if fellow.health <= 0 {
                                            crate::explosions::spawn_boom(&mut commands, 
                                                Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + ENEMY_NN_SIZE.y, 0.0), &explosions_images);
                                            sfx.play(sounds.boom.clone());
                                            commands.entity(enemy_entity).despawn_recursive();
                                            tmp_score += 100;
                                            fellow_died = true;
                                        }
                                        else {
                                            sfx.play(sounds.enemy_damage.clone());
                                        }
                                    }
                                };
        
                                if !collided {
                                    // check also carrier
                                    if collision_check(target_pos, target_size, enemy_transform.translation, ENEMY_NN_SIZE) {
                                        collided = true;
                                        enemy.health -= CANNON_DAMAGE;
                                        if enemy.health <= 0 {
                                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                                            sfx.play(sounds.boom.clone());
                                            commands.entity(enemy_entity).despawn_recursive();
                                            tmp_score += 100;
                                            carrier_died = true;
                                        }
                                        else {
                                            sfx.play(sounds.enemy_damage.clone());
                                        }
                                    }
                                }

                                if fellow_died {
                                    // destroy also carrier
                                    enemy.health = 0;
                                    crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                                    sfx.play(sounds.boom.clone());
                                    commands.entity(enemy_entity).despawn_recursive();
                                }

                                if carrier_died {
                                    // destroy also fellow
                                    if enemy.fellow_enemy.is_some() {
                                        let fellow = enemy.fellow_enemy.as_mut().unwrap();
                                        fellow.health = 0;
                                        crate::explosions::spawn_boom(&mut commands, 
                                            Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + ENEMY_NN_SIZE.y, 0.0), &explosions_images);
                                        sfx.play(sounds.boom.clone());
                                        commands.entity(enemy_entity).despawn_recursive();
                                    }
                                }
                            }
                            else {
                                // others
                                if collision_check(target_pos, target_size, enemy_transform.translation, ENEMY_NN_SIZE) {
                                    collided = true;
                                    enemy.health -= CANNON_DAMAGE;
                                    if enemy.health <= 0 {
                                        crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                                        if enemy.enemy_type == 0 {
                                            crate::explosions::spawn_flash(&mut commands);
                                            sfx.play(sounds.boom_base.clone());
                                            maze.score += 1000;
                                            maze.bases -= 1;
                                        }
                                        else {
                                            sfx.play(sounds.boom.clone());
                                            maze.score += 100;
                                        }
                                        commands.entity(enemy_entity).despawn_recursive();
                                    }
                                    else {
                                        sfx.play(sounds.enemy_damage.clone());
                                    }
                                }
                            }
                        }
                        maze.score += tmp_score;
                    }
                } // end for

                if collided {
                    // cannon collided with some enemy
                    // despawn cannon shot and allow cannon shooting
                    if let Some(entity) = eopt {
                        commands.entity(entity).despawn_recursive();
                    }
                    player.shooting_cannon = false;
                    change_score_event.send(ScoreChange(maze.score));
                    change_base_count_event.send(BaseCountChange(maze.bases));
                }
                else {
                    // check collision with walls
                    if !walls_query.iter().any(|&wall| collision_check(
                        target_pos, target_size, 
                        wall.position, wall.size)) 
                    {
                        // no collison, move
                        cannon_transform.translation.x = target_pos.x;

                        // is out of bounds ?
                        if cannon_transform.translation.x < -SHOT_CANNON_W || cannon_transform.translation.x > WINDOW_W+SHOT_CANNON_W {
                            // despawn cannon shot and allow cannon shooting
                            if let Some(entity) = eopt {
                                commands.entity(entity).despawn_recursive();
                            }
                            player.shooting_cannon = false;
                        }
                    }
                    else {
                        // cannon shot hits wall
                        // despawn cannon shot and allow cannon shooting
                        if let Some(entity) = eopt {
                            commands.entity(entity).despawn_recursive();
                        }
                        player.shooting_cannon = false;
                    }
                }
            }
        }
    }
}

fn player_vs_enemy(
    mut commands: Commands, 
    mut player: ResMut<Player>,
    mut player_query: Query<&Transform, With<PlayerComponent>>,
    mut enemies_query: Query<(Entity, &EnemyComponent, &Transform), Without<PlayerComponent>>,
    mut maze: ResMut<Maze>,
    explosions_images: Res<ExplosionsImages>,
    sfx: Res<AudioChannel<SfxChannel>>, 
    sfx_dmg: Res<AudioChannel<DamageChannel>>, 
    sounds: Res<Sounds>,
    mut change_special_event: EventWriter<SpecialChange>,
    mut change_special_ammo_event: EventWriter<SpecialAmmoChange>,
    mut change_score_event: EventWriter<ScoreChange>,
    mut change_base_count_event: EventWriter<BaseCountChange>)
{
    if player.damage_delay > 0 {
        player.damage_delay -= 1;
        return;
    }

    let player_transform = player_query.single_mut();
    let mut taking_damage = false;
    let mut dmg_sound_started = false;
    let mut something_died = false;

    for (enemy_entity, enemy_component, enemy_transform) in enemies_query.iter_mut() 
    {
        // from 10
        if enemy_component.is_from_10 {
            let room = &mut maze.rooms[enemy_component.room_seq];
                        
            if collision_check(player_transform.translation, Vec2::new(PLAYER_W, PLAYER_H), 
                enemy_transform.translation, ENEMY_NN_SIZE) 
            {
                for (idx, mut enemy) in room.enemies_from_10.iter_mut().enumerate() {
                    if enemy.enemy_seq == enemy_component.enemy_seq {
                        taking_damage = true;
                        player.health -= 10.0;
                        enemy.health -= 10;
                        if enemy.health <= 0 {
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            room.enemies_from_10.remove(idx);
                            something_died = true;
                            maze.score += 100;
                            break;
                        }
                        else if !dmg_sound_started && !sfx_dmg.is_playing_sound() {
                            println!("DAMAGE: from 10: {}", enemy.enemy_type);
                            sfx_dmg.play(sounds.damage.clone()).looped();
                            dmg_sound_started = true;
                        }
                    }
                }
            }
        }
        else {
            let mut tmp_score: u16 = 0;
            {
                let enemy = &mut maze.rooms[enemy_component.room_seq].enemies[enemy_component.enemy_seq];

                // 20 + fellow
                if enemy.enemy_type == 20 {
                    let mut fellow_died = false;
                    let mut carrier_died = false;

                    if enemy.fellow_enemy.is_some() {
                        let fellow = enemy.fellow_enemy.as_mut().unwrap();

                        // prepare fellow size and position
                        let mut fellow_size = ENEMY_NN_SIZE;
                        if fellow.enemy_type == 7 {
                            fellow_size = ENEMY_07_SIZE;
                        }
                        let fellow_pos = Vec3::new(
                            enemy_transform.translation.x, 
                            enemy_transform.translation.y + (ENEMY_NN_SIZE.y / 2.0) + (fellow_size.y / 2.0),
                            0.0);
                        
                        // check collision with fellow
                        if collision_check(player_transform.translation, Vec2::new(PLAYER_W, PLAYER_H), fellow_pos, fellow_size) 
                        {
                            taking_damage = true;
                            player.health -= 10.0;
                            fellow.health -= 10;
                            if fellow.health <= 0 {
                                crate::explosions::spawn_boom(&mut commands, 
                                    Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + ENEMY_NN_SIZE.y, 0.0), &explosions_images);
                                sfx.play(sounds.boom.clone());
                                commands.entity(enemy_entity).despawn_recursive();
                                fellow_died = true;
                                something_died = true;
                                tmp_score += 100;
                            }
                            else if !dmg_sound_started && !sfx_dmg.is_playing_sound() {
                                println!("DAMAGE: fellow: {}", enemy.enemy_type);
                                sfx_dmg.play(sounds.damage.clone()).looped();
                                dmg_sound_started = true;
                            }
                        }
                    }
                    else if enemy.fellow_item.is_some() {
                        let fellow = enemy.fellow_item.as_mut().unwrap();

                        // check collision with fellow item
                        if collision_check(player_transform.translation, Vec2::new(PLAYER_W, PLAYER_H), 
                            Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + ENEMY_NN_SIZE.y, 0.0), 
                            Vec2::new(ITEM_W, ITEM_H)) 
                        {
                            sfx.play(sounds.get_item.clone());

                            match fellow.item_type {
                                0 => player.ammo = AMMO_MAX,
                                1 => {
                                    player.special_type = SpecialType::Ball;
                                    player.ammo_special = 10;
                                },
                                2 => player.fuel = FUEL_MAX,
                                3 => {
                                    player.special_type = SpecialType::MissileDown;
                                    player.ammo_special = 50;
                                },
                                4 => {
                                    player.special_type = SpecialType::MissileSide;
                                    player.ammo_special = 50;
                                },
                                5 => player.health = HEALTH_MAX,
                                6 => { }, // star
                                7 => { }, // random
                                _ => panic!("Unexpected special type!")
                            }

                            change_special_event.send(SpecialChange(player.special_type));
                            change_special_ammo_event.send(SpecialAmmoChange(player.ammo_special));

                            fellow.collected = true;
                            fellow_died = true;
                        }
                    }

                    // check carrier
                    if collision_check(player_transform.translation, Vec2::new(PLAYER_W, PLAYER_H), 
                        enemy_transform.translation, ENEMY_NN_SIZE) 
                    {
                        taking_damage = true;
                        player.health -= 10.0;
                        enemy.health -= 10;
                        if enemy.health <= 0 {
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            carrier_died = true;
                            something_died = true;
                            tmp_score += 100;
                        }
                        else if !dmg_sound_started && !sfx_dmg.is_playing_sound() {
                            println!("DAMAGE: carrier");
                            sfx_dmg.play(sounds.damage.clone()).looped();
                            dmg_sound_started = true;
                        }
                    }

                    if fellow_died {
                        // destroy also carrier
                        enemy.health = 0;
                        crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                        sfx.play(sounds.boom.clone());
                        commands.entity(enemy_entity).despawn_recursive();
                    }

                    if carrier_died {
                        // destroy also fellow
                        if enemy.fellow_enemy.is_some() {
                            let fellow = enemy.fellow_enemy.as_mut().unwrap();
                            fellow.health = 0;
                            crate::explosions::spawn_boom(&mut commands, 
                                Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + ENEMY_NN_SIZE.y, 0.0), &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                        }
                    }
                }
                // others
                else {
                    if collision_check(
                        player_transform.translation, Vec2::new(PLAYER_W, PLAYER_H), 
                        enemy_transform.translation, ENEMY_NN_SIZE) 
                    {
                        taking_damage = true;
                        player.health -= 10.0;
                        enemy.health -= 10;
                        if enemy.health <= 0 {
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            if enemy.enemy_type == 0 {
                                crate::explosions::spawn_flash(&mut commands);
                                sfx.play(sounds.boom_base.clone());
                                maze.score += 1000;
                                maze.bases -= 1;
                            }
                            else {
                                sfx.play(sounds.boom.clone());
                                maze.score += 100;
                            }
                            something_died = true;
                            commands.entity(enemy_entity).despawn_recursive();
                        }
                        else if !dmg_sound_started && !sfx_dmg.is_playing_sound() {
                            println!("DAMAGE: {}", enemy.enemy_type);
                            sfx_dmg.play(sounds.damage.clone()).looped();
                            dmg_sound_started = true;
                        }
                    }
                }
            }
            maze.score += tmp_score;
        }
    }

    if something_died {
        change_score_event.send(ScoreChange(maze.score));
        change_base_count_event.send(BaseCountChange(maze.bases));
    }

    if !taking_damage {
        sfx_dmg.stop();
    }

    player.damage_delay = DAMAGE_DELAY;

}

pub fn check_player_status(mut player: ResMut<Player>, mut state: ResMut<State<GameState>>) {
    if !player.is_dead {
        if player.fuel < 0.0 {
            player.health -= 0.5;
        }
        if player.health < 0.0 {
            state.push(GameState::Death).expect("PLAYER: Failed to push Death state!");
            return;
        }
    }
    else {
        // state.set(GameState::Menu).expect("PLAYER: Failed to change state!");
        state.set(GameState::GameOver).expect("PLAYER: Failed to change state!");
    }
}
