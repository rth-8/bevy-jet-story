use bevy::prelude::*;
use bevy_kira_audio::{AudioControl, AudioChannel};
use rand::Rng;

use crate::GameState;
use crate::maze::Maze;
use crate::player::{Player, PLAYER_W, PLAYER_H, AMMO_MAX, FUEL_MAX, HEALTH_MAX, PlayerComponent};
use crate::collision_check;
use crate::audio::{SfxChannel, Sounds};
use crate::special::SpecialType;
use crate::infobar::{SpecialChange, SpecialAmmoChange};

pub struct ItemsPlugin;

pub const ITEM_W: f32 = 49.0;
pub const ITEM_H: f32 = 43.0;
pub const ITEM_W2: f32 = ITEM_W / 2.0;
pub const ITEM_H2: f32 = ITEM_H / 2.0;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_items_resources)
        .add_system_set(SystemSet::on_update(GameState::Game)
            .with_system(change_items_color)
            .with_system(item_vs_player));
    }
}

pub struct Item {
    pub posx: f32,
    pub posy: f32,
    pub item_type: usize,
    pub collected: bool,
    pub item_seq: usize,
    pub room_seq: usize,
}

pub struct FellowItem {
    pub posx: f32,
    pub posy: f32,
    pub item_type: usize,
    pub collected: bool,
}

#[derive(Component)]
pub struct ItemComponent {
    pub item_seq: usize,
    pub room_seq: usize,
    pub color_index: usize,
}

pub struct ItemsImages {
    pub item_ammo: Handle<Image>,
    pub item_ball: Handle<Image>,
    pub item_fuel: Handle<Image>,
    pub item_missile_down: Handle<Image>,
    pub item_missile_side: Handle<Image>,
    pub item_random: Handle<Image>,
    pub item_shield: Handle<Image>,
    pub item_star: Handle<Image>,
}

pub fn load_items_resources(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(ItemsImages {
        item_ammo: assets.load("images/items/item_ammo.png"),
        item_ball: assets.load("images/items/item_ball.png"),
        item_fuel: assets.load("images/items/item_fuel.png"),
        item_missile_down: assets.load("images/items/item_m_d.png"),
        item_missile_side: assets.load("images/items/item_m_lr.png"),
        item_random: assets.load("images/items/item_random.png"),
        item_shield: assets.load("images/items/item_shield.png"),
        item_star: assets.load("images/items/item_star.png"),
    });
}

pub const ITEM_COLORS: [Color; 7] = [ 
    Color::rgb(0.0, 0.0, 1.0), // blue
    // Color::rgb(1.0, 0.0, 0.0), // red
    // Color::rgb(1.0, 0.0, 1.0), // magenta
    Color::rgb(0.0, 1.0, 0.0), // green
    Color::rgb(0.0, 1.0, 1.0), // cyan
    Color::rgb(1.0, 1.0, 0.0), // yellow
    Color::rgb(1.0, 1.0, 1.0), // white
    Color::rgb(1.0, 1.0, 0.0), // yellow
    Color::rgb(0.0, 1.0, 1.0), // cyan
    // Color::rgb(0.0, 1.0, 0.0), // green
    // Color::rgb(1.0, 0.0, 1.0), // magenta
    // Color::rgb(1.0, 0.0, 0.0), // red
];

#[derive(Component, Deref, DerefMut)]
pub struct ItemAnimationTimer(pub Timer);

pub fn change_items_color(
    time: Res<Time>,
    mut query: Query<(&mut ItemAnimationTimer, &mut Sprite, &mut ItemComponent)>,) 
{
    for (mut timer, mut sprite, mut item_component) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.color = ITEM_COLORS[item_component.color_index];
            item_component.color_index += 1;
            if item_component.color_index >= ITEM_COLORS.len() {
                item_component.color_index = 0;
            }
        }
    }
}

pub fn item_vs_player(
    mut commands: Commands, 
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut player: ResMut<Player>,
    items_query: Query<(Entity, &ItemComponent, &Transform), Without<PlayerComponent>>,
    mut maze: ResMut<Maze>,
    sfx: Res<AudioChannel<SfxChannel>>, 
    sounds: Res<Sounds>,
    mut change_special_event: EventWriter<SpecialChange>,
    mut change_special_ammo_event: EventWriter<SpecialAmmoChange>) 
{
    let player_transform = player_query.single();

    for (item_entity, item_component, item_transform) in items_query.iter() 
    {
        if item_component.room_seq > 127 {
            continue;
        }
        
        if collision_check(player_transform.translation, Vec2::new(PLAYER_W, PLAYER_H),
            item_transform.translation, Vec2::new(ITEM_W, ITEM_H))
        {
            // pick item
            let item = &mut maze.rooms[item_component.room_seq].items[item_component.item_seq];

            sfx.play(sounds.get_item.clone());

            match item.item_type {
                0 => player.ammo = AMMO_MAX,
                1 => {
                    player.special_type = SpecialType::Ball;
                    player.ammo_special = 20;
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
                6 => { 
                    player.special_type = SpecialType::Star;
                    player.ammo_special = 20;
                }, 
                7 => { 
                    let mut rng = rand::thread_rng();
                    match rng.gen_range(0..=6) {
                        0 => player.ammo = AMMO_MAX,
                        1 => {
                            player.special_type = SpecialType::Ball;
                            player.ammo_special = 20;
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
                        6 => { 
                            player.special_type = SpecialType::Star;
                            player.ammo_special = 20;
                        }, 
                        _ => panic!("Unexpected item type for random pickup!")
                    }
                }, 
                _ => panic!("Unexpected special type!")
            }

            change_special_event.send(SpecialChange(player.special_type));
            change_special_ammo_event.send(SpecialAmmoChange(player.ammo_special));

            item.collected = true;
            commands.entity(item_entity).despawn_recursive();
        }
    }
}
