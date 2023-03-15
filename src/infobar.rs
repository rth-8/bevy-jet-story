use bevy::prelude::*;
use rand::Rng;

use crate::WINDOW_H;
use crate::GameState;
use crate::enemies::ENEMY_COLORS;
use crate::mainmenu::UiAssets;
use crate::maze::Maze;
use crate::player::AMMO_MAX;
use crate::player::FUEL_MAX;
use crate::player::HEALTH_MAX;
use crate::player::Player;
use crate::special::BALL_H2;
use crate::special::BALL_SIZE;
use crate::special::BALL_W2;
use crate::special::SpecialImages;
use crate::special::SpecialType;

pub struct InfoBarPlugin;

#[derive(Component)]
pub struct InfoBar;

#[derive(Component)]
pub struct FuelBar;

#[derive(Component)]
pub struct AmmoBar;

#[derive(Component)]
pub struct ShieldBar;

#[derive(Component)]
pub struct InfoBarRect {
    cooldown: u16,
}

#[derive(Component)]
pub struct SpecialInfoRect;

#[derive(Component)]
pub struct SpecialInfoText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct BasesCountText;

#[derive(Component, Deref, DerefMut)]
pub struct InfoBarRectsTimer(pub Timer);

pub struct SpecialChange(pub SpecialType);
pub struct SpecialAmmoChange(pub u8);
pub struct ScoreChange(pub u16);
pub struct BaseCountChange(pub u8);

pub const BAR_SIZE: f32 = 22.0;
pub const BAR_SIZE2: f32 = BAR_SIZE / 2.0;
pub const BAR_GAP: f32 = 3.0;

pub const COLOR_RESET_COOLDOWN: u16 = 10;

pub const FUEL_BAR_MAX_W: f32 = 100.0;
pub const FUEL_BAR_X_POS: f32 = 125.0;
pub const FUEL_BAR_Y_POS: f32 = WINDOW_H - 25.0;

pub const AMMO_BAR_MAX_W: f32 = 100.0;
pub const AMMO_BAR_X_POS: f32 = 125.0;
pub const AMMO_BAR_Y_POS: f32 = WINDOW_H - 50.0;

pub const SHIELD_BAR_MAX_W: f32 = 100.0;
pub const SHIELD_BAR_X_POS: f32 = 375.0;
pub const SHIELD_BAR_Y_POS: f32 = WINDOW_H - 50.0;

pub const SPECIAL_INFO_1_X: f32 = 275.0;
pub const SPECIAL_INFO_2_X: f32 = 300.0;
pub const SPECIAL_INFO_Y: f32 = 25.0;

pub const SPECIAL_INFO_TEXT_X: f32 = 280.0;
pub const SPECIAL_INFO_TEXT_Y: f32 = 50.0;

pub const SCORE_TEXT_X: f32 = 555.0;
pub const SCORE_TEXT_Y: f32 = 25.0;

pub const BASES_COUNT_TEXT_X: f32 = 730.0;
pub const BASES_COUNT_TEXT_Y: f32 = 50.0;

impl Plugin for InfoBarPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SpecialChange>()
        .add_event::<SpecialAmmoChange>()
        .add_event::<ScoreChange>()
        .add_event::<BaseCountChange>()
        .add_system_set(SystemSet::on_enter(GameState::Game)
            .with_system(spawn_info_bar))
        .add_system_set(SystemSet::on_update(GameState::Game)
            .with_system(update_bars)
            .with_system(special_changed)
            .with_system(special_ammo_changed)
            .with_system(score_changed)
            .with_system(base_count_changed)
            .with_system(change_rects_color))
        .add_system_set(SystemSet::on_exit(GameState::Game)
            .with_system(despawn_info_bar));
    }
}

fn spawn_rects(commands: &mut Commands, start_x: f32, y: f32, amount: u8, assets: &Res<AssetServer>) {
    for i in 0..amount {
        commands.spawn_bundle(SpriteBundle {
            texture: assets.load("images/bar.png").clone(),
            sprite: Sprite { 
                color: Color::CYAN,
                custom_size: Some(Vec2::splat(BAR_SIZE)),
                flip_x: false,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(start_x + (i as f32) * (BAR_SIZE + BAR_GAP) + BAR_SIZE2, WINDOW_H - y - BAR_SIZE2, 300.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InfoBarRect { cooldown: COLOR_RESET_COOLDOWN });
    }
}

fn spawn_info_bar(mut commands: Commands, assets: Res<AssetServer>, ui_assets: Res<UiAssets>, special_images: Res<SpecialImages>, maze: Res<Maze>) {
    // text style
    let text_style = TextStyle {
        font: ui_assets.font.clone(),
        font_size: 25.0,
        color: Color::WHITE,
    };

    // info bar
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("images/infobar.png").clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(Vec2::new(800.0, 100.0)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(400.0, WINDOW_H - 50.0, 200.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(InfoBar)
    .insert(InfoBarRectsTimer(Timer::from_seconds(0.2, true)));

    // fuel
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("images/bar.png").clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(Vec2::new(FUEL_BAR_MAX_W, BAR_SIZE)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(FUEL_BAR_X_POS + FUEL_BAR_MAX_W / 2.0, FUEL_BAR_Y_POS - BAR_SIZE2, 300.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(FuelBar);

    // ammo
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("images/bar.png").clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(Vec2::new(AMMO_BAR_MAX_W, BAR_SIZE)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(AMMO_BAR_X_POS + AMMO_BAR_MAX_W / 2.0, AMMO_BAR_Y_POS - BAR_SIZE2, 300.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(AmmoBar);

    // shield (health)
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("images/bar.png").clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(Vec2::new(SHIELD_BAR_MAX_W, BAR_SIZE)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(SHIELD_BAR_X_POS + SHIELD_BAR_MAX_W / 2.0, SHIELD_BAR_Y_POS - BAR_SIZE2, 300.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ShieldBar);

    // special info 1
    commands.spawn_bundle(SpriteBundle {
        texture: special_images.ball.clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(BALL_SIZE),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(SPECIAL_INFO_1_X + BALL_W2, WINDOW_H - SPECIAL_INFO_Y - BALL_H2, 300.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpecialInfoRect);

    // special info 2
    commands.spawn_bundle(SpriteBundle {
        texture: special_images.ball.clone(),
        sprite: Sprite { 
            color: Color::WHITE,
            custom_size: Some(BALL_SIZE),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(SPECIAL_INFO_2_X + BALL_W2, WINDOW_H - SPECIAL_INFO_Y - BALL_H2, 300.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpecialInfoRect);

    // special info text
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("04", text_style.clone()),
        transform: Transform { 
            translation: Vec3::new(SPECIAL_INFO_TEXT_X, WINDOW_H - SPECIAL_INFO_TEXT_Y, 300.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpecialInfoText);

    // score info text
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("0000000", text_style.clone()),
        transform: Transform { 
            translation: Vec3::new(SCORE_TEXT_X, WINDOW_H - SCORE_TEXT_Y, 300.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ScoreText);

    // bases count info text
    let bases = format!("{:0>2}", maze.bases);
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(bases, text_style.clone()),
        transform: Transform { 
            translation: Vec3::new(BASES_COUNT_TEXT_X, WINDOW_H - BASES_COUNT_TEXT_Y, 300.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(BasesCountText);

    // rects - row 1
    spawn_rects(&mut commands, 0.0, 0.0, 32, &assets);
    // rects - row 2
    spawn_rects(&mut commands, 0.0, 25.0, 1, &assets);
    spawn_rects(&mut commands, 250.0, 25.0, 1, &assets);
    spawn_rects(&mut commands, 325.0, 25.0, 1, &assets);
    spawn_rects(&mut commands, 500.0, 25.0, 2, &assets);
    spawn_rects(&mut commands, 700.0, 25.0, 1, &assets);
    spawn_rects(&mut commands, 775.0, 25.0, 1, &assets);
    // rects - row 3
    spawn_rects(&mut commands, 0.0, 50.0, 1, &assets);
    spawn_rects(&mut commands, 250.0, 50.0, 1, &assets);
    spawn_rects(&mut commands, 325.0, 50.0, 1, &assets);
    spawn_rects(&mut commands, 500.0, 50.0, 9, &assets);
    spawn_rects(&mut commands, 775.0, 50.0, 1, &assets);
    // rects - row 4
    spawn_rects(&mut commands, 0.0, 75.0, 32, &assets);
}

fn despawn_info_bar(
    mut commands: Commands, 
    info_bar_query: Query<Entity, With<InfoBar>>,
    fuel_bar_query: Query<Entity, With<FuelBar>>,
    ammo_bar_query: Query<Entity, With<AmmoBar>>,
    shield_bar_query: Query<Entity, With<ShieldBar>>,
    special_info_query: Query<Entity, With<SpecialInfoRect>>,
    special_text_query: Query<Entity, With<SpecialInfoText>>,
    score_text_query: Query<Entity, With<ScoreText>>,
    base_count_text_query: Query<Entity, With<BasesCountText>>,
    info_bar_rect_query: Query<Entity, With<InfoBarRect>>,
) 
{
    for entity in info_bar_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in fuel_bar_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in ammo_bar_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in shield_bar_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in special_info_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in special_text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in score_text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in base_count_text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in info_bar_rect_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_bars(
    player: Res<Player>, 
    mut fuel_bar_query: Query<(&mut Sprite, &mut Transform), (With<FuelBar>, Without<AmmoBar>, Without<ShieldBar>)>,
    mut ammo_bar_query: Query<(&mut Sprite, &mut Transform), (With<AmmoBar>, Without<FuelBar>, Without<ShieldBar>)>,
    mut shield_bar_query: Query<(&mut Sprite, &mut Transform), (With<ShieldBar>, Without<AmmoBar>, Without<FuelBar>)>) 
{
    let (mut shield_bar_sprite, mut shield_bar_transform) = shield_bar_query.single_mut();
    let (mut fuel_bar_sprite, mut fuel_bar_transform) = fuel_bar_query.single_mut();
    let (mut ammo_bar_sprite, mut ammo_bar_transform) = ammo_bar_query.single_mut();

    let w = (player.health / HEALTH_MAX) * SHIELD_BAR_MAX_W;
    shield_bar_sprite.custom_size = Some(Vec2::new(w, BAR_SIZE));
    shield_bar_transform.translation.x = SHIELD_BAR_X_POS + w / 2.0;

    let w = (player.fuel / FUEL_MAX) * FUEL_BAR_MAX_W;
    fuel_bar_sprite.custom_size = Some(Vec2::new(w, BAR_SIZE));
    fuel_bar_transform.translation.x = FUEL_BAR_X_POS + w / 2.0;

    let w = (player.ammo as f32 / AMMO_MAX as f32) * AMMO_BAR_MAX_W;
    ammo_bar_sprite.custom_size = Some(Vec2::new(w, BAR_SIZE));
    ammo_bar_transform.translation.x = AMMO_BAR_X_POS + w / 2.0;
}

pub fn special_changed(
    mut change_event: EventReader<SpecialChange>,
    mut sprite_query: Query<&mut Handle<Image>, With<SpecialInfoRect>>,
    mut rects_query: Query<(&mut Sprite, &mut InfoBarRect)>,
    special_images: Res<SpecialImages>) 
{
    for change in change_event.iter() {
        for mut texture in sprite_query.iter_mut() {
            match change.0 {
                SpecialType::Ball => *texture = special_images.ball.clone(),
                SpecialType::MissileDown => *texture = special_images.missile_down.clone(),
                SpecialType::MissileSide => *texture = special_images.missile_side.clone(),
                SpecialType::Star => *texture = special_images.star.clone(),
            }
        }

        for (mut sprite, mut rect) in rects_query.iter_mut() {
            sprite.color = Color::CYAN;
            rect.cooldown = COLOR_RESET_COOLDOWN;
        }
    }
}

pub fn special_ammo_changed(mut change_event: EventReader<SpecialAmmoChange>, mut text_query: Query<&mut Text, With<SpecialInfoText>>) {
    for change in change_event.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{:0>2}", change.0);
        }
    }
}

pub fn score_changed(mut change_event: EventReader<ScoreChange>, mut text_query: Query<&mut Text, With<ScoreText>>) {
    for change in change_event.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{:0>7}", change.0);
        }
    }
}

pub fn base_count_changed(mut change_event: EventReader<BaseCountChange>, mut text_query: Query<&mut Text, With<BasesCountText>>) {
    for change in change_event.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{:0>2}", change.0);
        }
    }
}

pub fn change_rects_color(
    time: Res<Time>,
    mut timer_query: Query<&mut InfoBarRectsTimer>,
    mut rects_query: Query<(&mut Sprite, &mut InfoBarRect)>)
{
    for mut timer in &mut timer_query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let mut rng = rand::thread_rng();

            for (mut sprite, mut rect) in rects_query.iter_mut() {
                if rng.gen_bool(1.0 / 10.0) {
                    let color_idx = rng.gen_range(0..ENEMY_COLORS.len());
                    sprite.color = ENEMY_COLORS[color_idx];
                    rect.cooldown = COLOR_RESET_COOLDOWN;
                }
                else {
                    rect.cooldown -= 1;
                    if rect.cooldown == 0 {
                        sprite.color = Color::CYAN;
                        rect.cooldown = COLOR_RESET_COOLDOWN;
                    }
                }
            }
        }
    }
}
