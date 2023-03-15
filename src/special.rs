use bevy::prelude::*;
use bevy_kira_audio::{AudioControl, AudioChannel};
use rand::Rng;

use crate::explosions::ExplosionsImages;
use crate::infobar::{ScoreChange, BaseCountChange};
use crate::items::ITEM_COLORS;
use crate::player::{Player, PlayerComponent};
use crate::{WINDOW_W, WINDOW_H, INFO_BAR_H, GameState, collision_check, GameDirection};
use crate::enemies::{EnemyComponent, ENEMY_NN_SIZE, ENEMY_07_SIZE};
use crate::maze::{WallComponent, Maze};
use crate::audio::{Sounds, SfxChannel};

pub struct SpecialPlugin;

#[derive(Copy, Clone)]
pub enum SpecialType {
    Ball,
    MissileSide,
    MissileDown,
    Star,
}

impl Plugin for SpecialPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_special_resources)
        .add_system_set(SystemSet::on_update(GameState::Game)
            .with_system(special_ball_movement)
            .with_system(special_missile_side_movement)
            .with_system(special_missile_down_movement)
            .with_system(special_star_movement)
        )
        .add_system_set(SystemSet::on_exit(GameState::Game)
            .with_system(despawn_specials)
        );
    }
}

pub struct SpecialImages {
    pub ball: Handle<Image>,
    pub missile_side: Handle<Image>,
    pub missile_down: Handle<Image>,
    pub star: Handle<Image>,
}

pub fn load_special_resources(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(SpecialImages {
        ball: assets.load("images/ship/ball.png"),
        missile_side: assets.load("images/ship/missile_side.png"),
        missile_down: assets.load("images/ship/missile_down.png"),
        star: assets.load("images/ship/star.png"),
    });
}

#[derive(Component)]
pub struct SpecialBall {
    pub velocity: Vec2,
    pub duration: u16,
    pub color_index: usize,
}

#[derive(Component)]
pub struct SpecialMissileSide {
    pub direction: GameDirection,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct SpecialMissileDown {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct SpecialStar {
    pub velocity: Vec2,
    pub duration: u16,
    pub color_index: usize,
    pub sound_delay: u8,
}

pub const BALL_SIZE: Vec2 = Vec2::new(22.0, 22.0);
pub const BALL_W2: f32 = BALL_SIZE.x / 2.0;
pub const BALL_H2: f32 = BALL_SIZE.y / 2.0;

pub const MISSILE_SIDE_SIZE: Vec2 = Vec2::new(25.0, 21.0);

pub const MISSILE_SIDE_DOWN: Vec2 = Vec2::new(21.0, 25.0);

pub const STAR_SIZE: Vec2 = Vec2::new(21.0, 21.0);
pub const STAR_W2: f32 = STAR_SIZE.x / 2.0;
pub const STAR_H2: f32 = STAR_SIZE.y / 2.0;

pub const STAR_VELOCITY: f32 = 100.0;
pub const STAR_FORCE: f32 = 400.0;

pub fn spawn_special_ball(commands: &mut Commands, special_images: &Res<SpecialImages>, posx: f32, posy: f32, direction: GameDirection) {
    commands.spawn_bundle(SpriteBundle {
        texture: special_images.ball.clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(BALL_SIZE),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(posx, posy, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpecialBall {
        velocity: match direction {
            GameDirection::Left => Vec2::new(-250.0, -300.0),
            GameDirection::Right => Vec2::new(250.0, -300.0),
            GameDirection::None => panic!("Unexpected special direction!")
        },
        duration: 15000,
        color_index: 0,
    });
}

pub fn spawn_special_missile_side(commands: &mut Commands, special_images: &Res<SpecialImages>, posx: f32, posy: f32, direction: GameDirection) {
    commands.spawn_bundle(SpriteBundle {
        texture: special_images.missile_side.clone(),
        sprite: Sprite { 
            color: Color::CYAN,
            custom_size: Some(MISSILE_SIDE_SIZE),
            flip_x: match direction {
                GameDirection::Left => true,
                GameDirection::Right => false,
                GameDirection::None => panic!("Unexpected special direction!")
            },
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(posx, posy, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpecialMissileSide {
        direction,
        velocity: match direction {
            GameDirection::Left => Vec2::new(-200.0, -200.0),
            GameDirection::Right => Vec2::new(200.0, -200.0),
            GameDirection::None => panic!("Unexpected special direction!")
        },
    });
}

pub fn spawn_special_missile_down(commands: &mut Commands, special_images: &Res<SpecialImages>, posx: f32, posy: f32) {
    commands.spawn_bundle(SpriteBundle {
        texture: special_images.missile_down.clone(),
        sprite: Sprite { 
            color: Color::CYAN,
            custom_size: Some(MISSILE_SIDE_DOWN),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(posx, posy, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpecialMissileDown {
        velocity: Vec2::new(0.0, -200.0),
    });
}

pub fn spawn_special_star(commands: &mut Commands, special_images: &Res<SpecialImages>, posx: f32, posy: f32) {
    let mut rng = rand::thread_rng();

    let dir = rng.gen_range(0..=3);

    commands.spawn_bundle(SpriteBundle {
        texture: special_images.star.clone(),
        sprite: Sprite { 
            color: Color::YELLOW,
            custom_size: Some(STAR_SIZE),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(posx, posy, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpecialStar {
        velocity: match dir {
            0 => Vec2::new(-STAR_VELOCITY, STAR_VELOCITY),
            1 => Vec2::new(STAR_VELOCITY, STAR_VELOCITY),
            2 => Vec2::new(-STAR_VELOCITY, -STAR_VELOCITY),
            3 => Vec2::new(STAR_VELOCITY, -STAR_VELOCITY),
            _ => panic!("Unexpected star direction!")
        },
        duration: 18000,
        color_index: 0,
        sound_delay: 0,
    });
}

pub fn despawn_specials(
    mut commands: Commands, 
    special_ball_query: Query<Entity, With<SpecialBall>>,
    special_md_query: Query<Entity, With<SpecialBall>>,
    special_ms_query: Query<Entity, With<SpecialBall>>,
    special_star_query: Query<Entity, With<SpecialBall>>,)
{
    for entity in special_ball_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in special_md_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in special_ms_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in special_star_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn special_ball_movement(
    mut commands: Commands, 
    time: Res<Time>, 
    mut special_query: Query<(Entity, &mut Sprite, &mut Transform, &mut SpecialBall)>,
    walls_query: Query<&WallComponent>,
    mut enemies_query: Query<(Entity, &mut Transform, &mut EnemyComponent), Without<SpecialBall>>,
    mut player: ResMut<Player>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sounds: Res<Sounds>,
    mut maze: ResMut<Maze>,
    explosions_images: Res<ExplosionsImages>,
    mut change_score_event: EventWriter<ScoreChange>,
    mut change_base_count_event: EventWriter<BaseCountChange>) 
{
    for (special_entity, mut special_sprite, mut special_transform, mut ball) in special_query.iter_mut() {
        // apply force
        let mut position = Vec2::splat(0.0);
        position += ball.velocity;
        position *= time.delta_seconds();

        // check collision with walls

        let target = special_transform.translation + Vec3::new(position.x, 0.0, 0.0);
        if !walls_query.iter().any(|&wall| collision_check(
            target, BALL_SIZE, 
            wall.position, wall.size)) 
        {
            special_transform.translation = target;
        }
        else {
            sfx.play(sounds.ball_bounce.clone());
            ball.velocity.x *= -1.0;
        }

        let target = special_transform.translation + Vec3::new(0.0, position.y, 0.0);
        if !walls_query.iter().any(|&wall| collision_check(
            target, BALL_SIZE, 
            wall.position, wall.size)) 
        {
            special_transform.translation = target;
        }
        else {
            sfx.play(sounds.ball_bounce.clone());
            ball.velocity.y *= -1.0;
        }

        // check screen edges

        if special_transform.translation.x < BALL_W2 {
            sfx.play(sounds.ball_bounce.clone());
            special_transform.translation.x = BALL_W2;
            ball.velocity.x *= -1.0;
        }

        if special_transform.translation.x > WINDOW_W - BALL_W2 {
            sfx.play(sounds.ball_bounce.clone());
            special_transform.translation.x = WINDOW_W - BALL_W2;
            ball.velocity.x *= -1.0;
        }

        if special_transform.translation.y > WINDOW_H - INFO_BAR_H - BALL_H2 {
            sfx.play(sounds.ball_bounce.clone());
            special_transform.translation.y = WINDOW_H - INFO_BAR_H - BALL_H2;
            ball.velocity.y *= -1.0;
        }

        if special_transform.translation.y < BALL_H2 {
            sfx.play(sounds.ball_bounce.clone());
            special_transform.translation.y = BALL_H2;
            ball.velocity.y *= -1.0;
        }

        // check collision with enemies

        let mut something_died = false;
        for (enemy_entity, enemy_transform, enemy_component) in enemies_query.iter_mut() 
        {
            // from 10
            if enemy_component.is_from_10 {
                let room = &mut maze.rooms[enemy_component.room_seq];
                            
                if collision_check(special_transform.translation, BALL_SIZE, 
                    enemy_transform.translation, ENEMY_NN_SIZE) 
                {
                    for (idx, mut enemy) in room.enemies_from_10.iter_mut().enumerate() {
                        if enemy.enemy_seq == enemy_component.enemy_seq {
                            enemy.health = 0;
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            room.enemies_from_10.remove(idx);
                            something_died = true;
                            maze.score += 100;
                            break;
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
                            if collision_check(special_transform.translation, BALL_SIZE, fellow_pos, fellow_size) 
                            {
                                fellow.health = 0;
                                crate::explosions::spawn_boom(&mut commands, 
                                        Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + ENEMY_NN_SIZE.y, 0.0), &explosions_images);
                                sfx.play(sounds.boom.clone());
                                commands.entity(enemy_entity).despawn_recursive();
                                fellow_died = true;
                                something_died = true;
                                tmp_score += 100;
                            }
                        };

                        // check carrier
                        if collision_check(special_transform.translation, BALL_SIZE, enemy_transform.translation, ENEMY_NN_SIZE) 
                        {
                            enemy.health = 0;
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            carrier_died = true;
                            something_died = true;
                            tmp_score += 100;
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
                            special_transform.translation, BALL_SIZE, enemy_transform.translation, ENEMY_NN_SIZE) 
                        {
                            enemy.health = 0;
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
                    }
                }
                maze.score += tmp_score;
            }
        }

        if something_died {
            change_score_event.send(ScoreChange(maze.score));
            change_base_count_event.send(BaseCountChange(maze.bases));
        }

        // move ball
        ball.duration -= 1;
        if ball.duration == 0 {
            commands.entity(special_entity).despawn_recursive();
            player.shooting_special = false;
        }

        if ball.duration <= 6000 {
            special_sprite.color = ITEM_COLORS[ball.color_index];
            ball.color_index += 1;
            if ball.color_index >= ITEM_COLORS.len() {
                ball.color_index = 0;
            }
            ball.velocity.x -= ball.velocity.x * 0.0001;
            ball.velocity.y -= ball.velocity.y * 0.0001;
        }
        // println!("Ball velocity = {:?}", ball.velocity);
    }
}

pub fn special_missile_side_movement(
    mut commands: Commands, 
    time: Res<Time>, 
    mut special_query: Query<(Entity, &mut Transform, &mut SpecialMissileSide)>,
    walls_query: Query<&WallComponent>,
    mut enemies_query: Query<(Entity, &mut Transform, &mut EnemyComponent), Without<SpecialMissileSide>>,
    mut player: ResMut<Player>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sounds: Res<Sounds>,
    mut maze: ResMut<Maze>,
    explosions_images: Res<ExplosionsImages>,
    mut change_score_event: EventWriter<ScoreChange>,
    mut change_base_count_event: EventWriter<BaseCountChange>) 
{
    for (special_entity, mut special_transform, mut missile_side) in special_query.iter_mut() {
        let mut force = Vec2::splat(0.0);

        if missile_side.velocity.y < -1.0 || missile_side.velocity.y > 1.0 {
            force += Vec2::new(0.0, 150.0);
        }

        match missile_side.direction {
            GameDirection::Left => force += Vec2::new(-100.0, 0.0),
            GameDirection::Right => force += Vec2::new(100.0, 0.0),
            GameDirection::None => {}
        }

        force *= time.delta_seconds();
        missile_side.velocity += force;
        // missile_side.velocity.x = missile_side.velocity.x.clamp(-400.0, 400.0);
        // missile_side.velocity.y = missile_side.velocity.y.clamp(-400.0, 400.0);

        // calculate new position
        let mut position = Vec2::splat(0.0);
        position += missile_side.velocity;
        position *= time.delta_seconds();

        // check collision with walls
        let target = special_transform.translation + Vec3::new(position.x, position.y, 0.0);
        if walls_query.iter().any(|&wall| collision_check(
            target, MISSILE_SIDE_SIZE, 
            wall.position, wall.size)) 
        {
            // collision with wall
            commands.entity(special_entity).despawn_recursive();
            player.shooting_special = false;
            continue;
        }

        // check screen edges

        if special_transform.translation.x < BALL_W2 || 
           special_transform.translation.x > WINDOW_W - BALL_W2 || 
           special_transform.translation.y < BALL_H2 ||
           special_transform.translation.y > WINDOW_H - INFO_BAR_H - BALL_H2
        {
            commands.entity(special_entity).despawn_recursive();
            player.shooting_special = false;
            continue;
        }

        // move missile
        special_transform.translation = target;

        // check collision with enemies

        let mut something_died = false;
        for (enemy_entity, enemy_transform, enemy_component) in enemies_query.iter_mut() 
        {
            let mut collided = false;

            // from 10
            if enemy_component.is_from_10 {
                let room = &mut maze.rooms[enemy_component.room_seq];
                            
                if collision_check(special_transform.translation, MISSILE_SIDE_SIZE, 
                    enemy_transform.translation, ENEMY_NN_SIZE) 
                {
                    for (idx, mut enemy) in room.enemies_from_10.iter_mut().enumerate() {
                        if enemy.enemy_seq == enemy_component.enemy_seq {
                            collided = true;
                            enemy.health = 0;
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            room.enemies_from_10.remove(idx);
                            something_died = true;
                            maze.score += 100;
                            break;
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
                            if collision_check(special_transform.translation, MISSILE_SIDE_SIZE, fellow_pos, fellow_size) 
                            {
                                collided = true;
                                fellow.health = 0;
                                crate::explosions::spawn_boom(&mut commands, 
                                        Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + ENEMY_NN_SIZE.y, 0.0), &explosions_images);
                                sfx.play(sounds.boom.clone());
                                commands.entity(enemy_entity).despawn_recursive();
                                fellow_died = true;
                                something_died = true;
                                tmp_score += 100;
                            }
                        };

                        // check carrier
                        if collision_check(special_transform.translation, MISSILE_SIDE_SIZE, enemy_transform.translation, ENEMY_NN_SIZE) 
                        {
                            collided = true;
                            enemy.health = 0;
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            carrier_died = true;
                            something_died = true;
                            tmp_score += 100;
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
                            special_transform.translation, MISSILE_SIDE_SIZE, enemy_transform.translation, ENEMY_NN_SIZE) 
                        {
                            collided = true;
                            enemy.health = 0;
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
                    }
                }
                maze.score += tmp_score;
            }

            if collided {
                commands.entity(special_entity).despawn_recursive();
                player.shooting_special = false;
            }
        } // end for enemies

        if something_died {
            change_score_event.send(ScoreChange(maze.score));
            change_base_count_event.send(BaseCountChange(maze.bases));
        }
    }
}

pub fn special_missile_down_movement(
    mut commands: Commands, 
    time: Res<Time>, 
    mut special_query: Query<(Entity, &mut Transform, &mut SpecialMissileDown)>,
    walls_query: Query<&WallComponent>,
    mut enemies_query: Query<(Entity, &mut Transform, &mut EnemyComponent), Without<SpecialMissileDown>>,
    mut player: ResMut<Player>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sounds: Res<Sounds>,
    mut maze: ResMut<Maze>,
    explosions_images: Res<ExplosionsImages>,
    mut change_score_event: EventWriter<ScoreChange>,
    mut change_base_count_event: EventWriter<BaseCountChange>) 
{
    for (special_entity, mut special_transform, mut missile_side) in special_query.iter_mut() {
        let mut force = Vec2::splat(0.0);
        force += Vec2::new(0.0, -100.0);

        force *= time.delta_seconds();
        missile_side.velocity += force;
        // missile_side.velocity.x = missile_side.velocity.x.clamp(-400.0, 400.0);
        // missile_side.velocity.y = missile_side.velocity.y.clamp(-400.0, 400.0);

        // calculate new position
        let mut position = Vec2::splat(0.0);
        position += missile_side.velocity;
        position *= time.delta_seconds();

        // check collision with walls
        let target = special_transform.translation + Vec3::new(position.x, position.y, 0.0);
        if walls_query.iter().any(|&wall| collision_check(
            target, MISSILE_SIDE_DOWN, 
            wall.position, wall.size)) 
        {
            // collision with wall
            commands.entity(special_entity).despawn_recursive();
            player.shooting_special = false;
            continue;
        }

        // check screen edges

        if special_transform.translation.x < BALL_W2 || 
           special_transform.translation.x > WINDOW_W - BALL_W2 || 
           special_transform.translation.y < BALL_H2 ||
           special_transform.translation.y > WINDOW_H - INFO_BAR_H - BALL_H2
        {
            commands.entity(special_entity).despawn_recursive();
            player.shooting_special = false;
            continue;
        }

        // move missile
        special_transform.translation = target;

        // check collision with enemies

        let mut something_died = false;
        for (enemy_entity, enemy_transform, enemy_component) in enemies_query.iter_mut() 
        {
            let mut collided = false;

            // from 10
            if enemy_component.is_from_10 {
                let room = &mut maze.rooms[enemy_component.room_seq];
                            
                if collision_check(special_transform.translation, MISSILE_SIDE_DOWN, 
                    enemy_transform.translation, ENEMY_NN_SIZE) 
                {
                    for (idx, mut enemy) in room.enemies_from_10.iter_mut().enumerate() {
                        if enemy.enemy_seq == enemy_component.enemy_seq {
                            collided = true;
                            enemy.health = 0;
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            room.enemies_from_10.remove(idx);
                            something_died = true;
                            maze.score += 100;
                            break;
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
                            if collision_check(special_transform.translation, MISSILE_SIDE_DOWN, fellow_pos, fellow_size) 
                            {
                                collided = true;
                                fellow.health = 0;
                                crate::explosions::spawn_boom(&mut commands, 
                                        Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + ENEMY_NN_SIZE.y, 0.0), &explosions_images);
                                sfx.play(sounds.boom.clone());
                                commands.entity(enemy_entity).despawn_recursive();
                                fellow_died = true;
                                something_died = true;
                                tmp_score += 100;
                            }
                        };

                        // check carrier
                        if collision_check(special_transform.translation, MISSILE_SIDE_DOWN, enemy_transform.translation, ENEMY_NN_SIZE) 
                        {
                            collided = true;
                            enemy.health = 0;
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            carrier_died = true;
                            something_died = true;
                            tmp_score += 100;
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
                            special_transform.translation, MISSILE_SIDE_DOWN, enemy_transform.translation, ENEMY_NN_SIZE) 
                        {
                            collided = true;
                            enemy.health = 0;
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
                            something_died = true;
                        }
                    }
                }
                maze.score += tmp_score;
            }

            if collided {
                commands.entity(special_entity).despawn_recursive();
                player.shooting_special = false;
            }
        } // end for enemies

        if something_died {
            change_score_event.send(ScoreChange(maze.score));
            change_base_count_event.send(BaseCountChange(maze.bases));
        }
    }
}

pub fn special_star_movement(
    mut commands: Commands, 
    time: Res<Time>, 
    mut special_query: Query<(Entity, &mut Sprite, &mut Transform, &mut SpecialStar)>,
    walls_query: Query<&WallComponent>,
    mut enemies_query: Query<(Entity, &mut Transform, &mut EnemyComponent), (Without<SpecialStar>, Without<PlayerComponent>)>,
    mut player: ResMut<Player>,
    player_query: Query<&Transform, (With<PlayerComponent>, Without<SpecialStar>, Without<EnemyComponent>)>,
    sfx: Res<AudioChannel<SfxChannel>>,
    sounds: Res<Sounds>,
    mut maze: ResMut<Maze>,
    explosions_images: Res<ExplosionsImages>,
    mut change_score_event: EventWriter<ScoreChange>,
    mut change_base_count_event: EventWriter<BaseCountChange>) 
{
    let player_transform = player_query.single();

    for (special_entity, mut special_sprite, mut special_transform, mut star) in special_query.iter_mut() 
    {
        if star.sound_delay > 0 {
            star.sound_delay -= 1;
        }

        let mut force = Vec2::new(
            -(special_transform.translation.x - player_transform.translation.x), 
            -(special_transform.translation.y - player_transform.translation.y));

        if force.length() > 0.0 {
            force = force.normalize() * STAR_FORCE;
            force *= time.delta_seconds();
        }
        // println!("Star force = {:?}", force);

        star.velocity += force;
        // star.velocity.x = star.velocity.x.clamp(-400.0, 400.0);
        // star.velocity.y = star.velocity.y.clamp(-400.0, 400.0);
        // println!("Star velocity = {:?}", star.velocity);

        // calculate new position
        let mut position = Vec2::splat(0.0);
        position += star.velocity;
        position *= time.delta_seconds();

        // check collision with walls

        let target = special_transform.translation + Vec3::new(position.x, 0.0, 0.0);
        if !walls_query.iter().any(|&wall| collision_check(
            target, STAR_SIZE, 
            wall.position, wall.size)) 
        {
            special_transform.translation = target;
        }
        else {
            if star.sound_delay == 0 {
                sfx.play(sounds.ball_bounce.clone());
                star.sound_delay = 100;
            }
            star.velocity.x *= -0.5;
        }

        let target = special_transform.translation + Vec3::new(0.0, position.y, 0.0);
        if !walls_query.iter().any(|&wall| collision_check(
            target, STAR_SIZE, 
            wall.position, wall.size)) 
        {
            special_transform.translation = target;
        }
        else {
            if star.sound_delay == 0 {
                sfx.play(sounds.ball_bounce.clone());
                star.sound_delay = 100;
            }
            star.velocity.y *= -0.5;
        }

        // check screen edges

        if special_transform.translation.x < STAR_W2 {
            if star.sound_delay == 0 {
                sfx.play(sounds.ball_bounce.clone());
                star.sound_delay = 100;
            }
            special_transform.translation.x = STAR_W2;
            star.velocity.x *= -0.5;
        }

        if special_transform.translation.x > WINDOW_W - STAR_W2 {
            if star.sound_delay == 0 {
                sfx.play(sounds.ball_bounce.clone());
                star.sound_delay = 100;
            }
            special_transform.translation.x = WINDOW_W - STAR_W2;
            star.velocity.x *= -0.5;
        }

        if special_transform.translation.y > WINDOW_H - INFO_BAR_H - STAR_H2 {
            if star.sound_delay == 0 {
                sfx.play(sounds.ball_bounce.clone());
                star.sound_delay = 100;
            }
            special_transform.translation.y = WINDOW_H - INFO_BAR_H - STAR_H2;
            star.velocity.y *= -0.5;
        }

        if special_transform.translation.y < STAR_H2 {
            if star.sound_delay == 0 {
                sfx.play(sounds.ball_bounce.clone());
                star.sound_delay = 100;
            }
            special_transform.translation.y = STAR_H2;
            star.velocity.y *= -0.5;
        }

        // check collision with enemies

        let mut something_died = false;
        for (enemy_entity, enemy_transform, enemy_component) in enemies_query.iter_mut() 
        {
            // from 10
            if enemy_component.is_from_10 {
                let room = &mut maze.rooms[enemy_component.room_seq];
                            
                if collision_check(special_transform.translation, STAR_SIZE, 
                    enemy_transform.translation, ENEMY_NN_SIZE) 
                {
                    for (idx, mut enemy) in room.enemies_from_10.iter_mut().enumerate() {
                        if enemy.enemy_seq == enemy_component.enemy_seq {
                            enemy.health = 0;
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            room.enemies_from_10.remove(idx);
                            something_died = true;
                            maze.score += 100;
                            break;
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
                            if collision_check(special_transform.translation, STAR_SIZE, fellow_pos, fellow_size) 
                            {
                                fellow.health = 0;
                                crate::explosions::spawn_boom(&mut commands, 
                                        Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + ENEMY_NN_SIZE.y, 0.0), &explosions_images);
                                sfx.play(sounds.boom.clone());
                                commands.entity(enemy_entity).despawn_recursive();
                                fellow_died = true;
                                something_died = true;
                                tmp_score += 100;
                            }
                        };

                        // check carrier
                        if collision_check(special_transform.translation, STAR_SIZE, enemy_transform.translation, ENEMY_NN_SIZE) 
                        {
                            enemy.health = 0;
                            crate::explosions::spawn_boom(&mut commands, enemy_transform.translation, &explosions_images);
                            sfx.play(sounds.boom.clone());
                            commands.entity(enemy_entity).despawn_recursive();
                            carrier_died = true;
                            something_died = true;
                            tmp_score += 100;
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
                            special_transform.translation, STAR_SIZE, enemy_transform.translation, ENEMY_NN_SIZE) 
                        {
                            enemy.health = 0;
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
                    }
                }
                maze.score += tmp_score;
            }
        } // end for enemies

        if something_died {
            change_score_event.send(ScoreChange(maze.score));
            change_base_count_event.send(BaseCountChange(maze.bases));
        }

        star.duration -= 1;
        if star.duration == 0 {
            commands.entity(special_entity).despawn_recursive();
            player.shooting_special = false;
        }
        else if star.duration <= 6000 {
            special_sprite.color = ITEM_COLORS[star.color_index];
            star.color_index += 1;
            if star.color_index >= ITEM_COLORS.len() {
                star.color_index = 0;
            }
        }

        // println!("Start POS = {:?}", special_transform.translation);
    }
}
