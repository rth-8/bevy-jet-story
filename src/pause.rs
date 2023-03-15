use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy_kira_audio::{AudioControl, AudioChannel};

use crate::{GameState, WINDOW_W, WINDOW_H, WINDOW_W2, WINDOW_H2, mainmenu::UiAssets};
use crate::audio::{DamageChannel, Shooting01Channel, Shooting05Channel, Shooting06Channel, Shooting08Channel, Shooting09Channel};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Pause)
            .with_system(spawn_pause)
            .with_system(pause_all_sounds)
        )
        .add_system_set(SystemSet::on_exit(GameState::Pause)
            .with_system(despawn_pause)
            .with_system(resume_all_sounds)
        );
    }
}

#[derive(Component)]
struct PauseOverlay;

#[derive(Component)]
struct PauseText;

fn spawn_pause(mut commands: Commands, ui_assets: Res<UiAssets>) {
    // text style
    let text_style = TextStyle {
        font: ui_assets.font.clone(),
        font_size: 40.0,
        color: Color::RED,
    };

    // overlay
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { 
            color: Color::rgba(0.0, 0.0, 0.0, 0.7),
            custom_size: Some(Vec2::new(WINDOW_W, WINDOW_H)),
            flip_x: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(WINDOW_W2, WINDOW_H2, 800.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(PauseOverlay);

    // text box
    let box_size = Vec2::new(200.0, 60.0);
    let box_position = Vec2::new(WINDOW_W2, WINDOW_H2);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..Default::default()
        },
        transform: Transform::from_translation(box_position.extend(900.0)),
        ..Default::default()
    })
    .insert(PauseText);

    // text
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("PAUSE", text_style.clone()).with_alignment(TextAlignment::CENTER),
        text_2d_bounds: Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(box_position.x, box_position.y, 999.0),
        ..Default::default()
    })
    .insert(PauseText);
}

fn despawn_pause(mut commands: Commands, overlay_query: Query<Entity, With<PauseOverlay>>, text_query: Query<Entity, With<PauseText>>) {
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn pause_all_sounds(
    sfx_dmg: Res<AudioChannel<DamageChannel>>, 
    sfx01: Res<AudioChannel<Shooting01Channel>>, 
    sfx05: Res<AudioChannel<Shooting05Channel>>, 
    sfx06: Res<AudioChannel<Shooting06Channel>>, 
    sfx08: Res<AudioChannel<Shooting08Channel>>, 
    sfx09: Res<AudioChannel<Shooting09Channel>>,) 
{
    sfx_dmg.pause();
    sfx01.pause();
    sfx05.pause();
    sfx06.pause();
    sfx08.pause();
    sfx09.pause();
}

fn resume_all_sounds(
    sfx_dmg: Res<AudioChannel<DamageChannel>>, 
    sfx01: Res<AudioChannel<Shooting01Channel>>, 
    sfx05: Res<AudioChannel<Shooting05Channel>>, 
    sfx06: Res<AudioChannel<Shooting06Channel>>, 
    sfx08: Res<AudioChannel<Shooting08Channel>>, 
    sfx09: Res<AudioChannel<Shooting09Channel>>,) 
{
    sfx_dmg.resume();
    sfx01.resume();
    sfx05.resume();
    sfx06.resume();
    sfx08.resume();
    sfx09.resume();
}
