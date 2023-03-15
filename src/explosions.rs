use bevy::prelude::*;
use rand::Rng;

use crate::{WINDOW_W, WINDOW_H, H_PADDING, GRAVITY, GameState, load_atlas, collision_check};
use crate::maze::WallComponent;

pub const FRAGMENT_COOLDOWN: u16 = 200;

pub struct ExplosionsPlugin;

impl Plugin for ExplosionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_explosions_images)
        .add_system_set(SystemSet::on_update(GameState::Game)
            .with_system(animate_boom)
            .with_system(fragment_movement)
            .with_system(process_flash)
        )
        .add_system_set(SystemSet::on_update(GameState::Death)
            .with_system(animate_boom)
            .with_system(fragment_movement)
            .with_system(process_flash)
        );
    }
}

pub struct ExplosionsImages {
    pub boom_image: Handle<TextureAtlas>,
    pub fragment_image: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct Boom;

#[derive(Component)]
pub struct Fragment {
    pub velocity: Vec2,
    pub cooldown: u16,
}

#[derive(Component, Deref, DerefMut)]
pub struct BoomAnimationTimer(pub Timer);

#[derive(Component)]
pub struct FlashEffect {
    counter: u8,
}

#[derive(Component, Deref, DerefMut)]
pub struct FlashAnimationTimer(pub Timer);

pub fn load_explosions_images(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    commands.insert_resource(ExplosionsImages {
        boom_image: load_atlas(&assets, &mut texture_atlases, "images/boom.png", Vec2::splat(50.0), 1, 6, Some(H_PADDING)),
        fragment_image: load_atlas(&assets, &mut texture_atlases, "images/fragment.png", Vec2::splat(25.0), 1, 5, Some(H_PADDING)),
    });
}

pub fn spawn_boom(commands: &mut Commands, position: Vec3, explosions_images: &Res<ExplosionsImages>,) {
    // boom
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: explosions_images.boom_image.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(Vec2::splat(50.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(BoomAnimationTimer(Timer::from_seconds(0.08, true)))
    .insert(Boom);

    // 0 1 2
    // 3   4
    // 5 6 7

    let mut rng = rand::thread_rng();

    // fragment 0
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: explosions_images.fragment_image.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(Vec2::splat(25.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Fragment { velocity: Vec2::new(rng.gen_range(-200.0..-150.0), rng.gen_range(150.0..200.0)), cooldown: FRAGMENT_COOLDOWN });

    // fragment 1
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: explosions_images.fragment_image.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(Vec2::splat(25.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Fragment { velocity: Vec2::new(0.0, rng.gen_range(150.0..200.0)), cooldown: FRAGMENT_COOLDOWN });

    // fragment 2
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: explosions_images.fragment_image.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(Vec2::splat(25.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Fragment { velocity: Vec2::new(rng.gen_range(150.0..200.0), rng.gen_range(150.0..200.0)), cooldown: FRAGMENT_COOLDOWN });

    // fragment 3
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: explosions_images.fragment_image.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(Vec2::splat(25.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Fragment { velocity: Vec2::new(rng.gen_range(-200.0..-150.0), 0.0), cooldown: FRAGMENT_COOLDOWN });

    // fragment 4
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: explosions_images.fragment_image.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(Vec2::splat(25.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Fragment { velocity: Vec2::new(rng.gen_range(150.0..200.0), 0.0), cooldown: FRAGMENT_COOLDOWN });

    // fragment 5
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: explosions_images.fragment_image.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(Vec2::splat(25.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Fragment { velocity: Vec2::new(rng.gen_range(-200.0..-150.0), rng.gen_range(-200.0..-150.0)), cooldown: FRAGMENT_COOLDOWN });

    // fragment 6
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: explosions_images.fragment_image.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(Vec2::splat(25.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Fragment { velocity: Vec2::new(0.0, rng.gen_range(-200.0..-150.0)), cooldown: FRAGMENT_COOLDOWN });

    // fragment 7
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: explosions_images.fragment_image.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: 0,
            custom_size: Some(Vec2::splat(25.0)),
            ..Default::default()
        },
        transform: Transform { 
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Fragment { velocity: Vec2::new(rng.gen_range(150.0..200.0), rng.gen_range(-200.0..-150.0)), cooldown: FRAGMENT_COOLDOWN });

}

fn animate_boom(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut BoomAnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>,)>,) 
{
    for (entity, mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index += 1;
            if sprite.index >= texture_atlas.textures.len() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn fragment_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut fragment_query: Query<(Entity, &mut Fragment, &mut Transform, &mut TextureAtlasSprite)>,
    walls_query: Query<&WallComponent>) 
{
    let mut rng = rand::thread_rng();

    for (entity, mut fragment, mut transform, mut sprite) in fragment_query.iter_mut() {
        if fragment.cooldown > 0 {
            fragment.cooldown -= 1;
        }
        else {
            fragment.cooldown = FRAGMENT_COOLDOWN;
            sprite.index = rng.gen_range(0..=4);
        }

        // gravity
        fragment.velocity += GRAVITY * time.delta_seconds() * rng.gen_range(3.0..10.0);

        // calculate new position

        let mut position = Vec2::splat(0.0);
        position += fragment.velocity;
        position *= time.delta_seconds();

        let target = transform.translation + Vec3::new(position.x, position.y, 0.0);
        if !walls_query.iter().any(|&wall| collision_check(
            target, Vec2::splat(25.0), 
            wall.position, wall.size)) 
        {
            transform.translation = target;
        }
        else {
            // println!("Fragment hits wall -> despawn");
            commands.entity(entity).despawn_recursive();
            return;
        }

        if transform.translation.x < -25.0 || transform.translation.x > WINDOW_W + 25.0 ||
           transform.translation.y < -25.0 || transform.translation.y > WINDOW_H + 25.0
        {
            // println!("Fragment out of bounds -> despawn");
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_flash(commands: &mut Commands) {
    commands.spawn()
    .insert(FlashAnimationTimer(Timer::from_seconds(0.1, true)))
    .insert(FlashEffect { counter: 0 });
}

pub fn process_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FlashEffect, &mut FlashAnimationTimer)>,
    mut clear_color: ResMut<ClearColor>)
{
    for (entity, mut effect, mut timer) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if effect.counter == 0 {
                clear_color.0 = Color::WHITE;
            }
            else if effect.counter == 2 {
                clear_color.0 = Color::YELLOW;
            }
            else if effect.counter == 4 {
                clear_color.0 = Color::BLACK;
                commands.entity(entity).despawn_recursive();
            }
            effect.counter += 1;
        }
    }
}
