use bevy::prelude::*;
use bevy_kira_audio::{AudioControl, AudioChannel};
use rand::Rng;

use crate::GameState;
use crate::audio::{Sounds, DamageChannel, Shooting01Channel, Shooting05Channel, Shooting06Channel, Shooting08Channel, Shooting09Channel, DeathSoundChannel};
use crate::explosions::ExplosionsImages;
use crate::items::ITEM_COLORS;
use crate::player::{Player, PlayerComponent};

pub struct DeathScreenPlugin;

impl Plugin for DeathScreenPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Death)
            .with_system(start_death).label("StartDeathSystem")
        )
        .add_system_set(SystemSet::on_update(GameState::Death)
            .with_system(check_death_timer).after("StartDeathSystem")
            .with_system(spawn_death_boom).after("StartDeathSystem")
        );
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DeathTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct DeathBoomsTimer(pub Timer);

pub fn start_death(
    mut commands: Commands,
    sfx_dmg: Res<AudioChannel<DamageChannel>>,
    sfx01: Res<AudioChannel<Shooting01Channel>>, 
    sfx05: Res<AudioChannel<Shooting05Channel>>, 
    sfx06: Res<AudioChannel<Shooting06Channel>>, 
    sfx08: Res<AudioChannel<Shooting08Channel>>, 
    sfx09: Res<AudioChannel<Shooting09Channel>>, 
    sfx_death: Res<AudioChannel<DeathSoundChannel>>, 
    mut sounds: ResMut<Sounds>) 
{
    println!("DEATH: start");
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
    sfx_death.play(sounds.death.clone());
    commands.spawn().insert(DeathTimer(Timer::from_seconds(4.0, false)));
    commands.spawn().insert(DeathBoomsTimer(Timer::from_seconds(0.3, true)));
}

pub fn check_death_timer(
    mut commands: Commands,
    mut player: ResMut<Player>,
    time: Res<Time>,
    mut death_timer_query: Query<&mut DeathTimer>,
    death_booms_timer_query: Query<Entity, With<DeathBoomsTimer>>,
    mut state: ResMut<State<GameState>>,)
{
    for mut timer in death_timer_query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            player.is_dead = true;
            for entity in death_booms_timer_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            state.pop().expect("DEATH: Failed to pop Death state!");
        }
    }
}

pub fn spawn_death_boom(
    mut commands: Commands,
    time: Res<Time>,
    mut death_booms_timer_query: Query<&mut DeathBoomsTimer>,
    mut player_query: Query<(&Transform, &mut Sprite), With<PlayerComponent>>,
    mut player: ResMut<Player>,
    explosions_images: Res<ExplosionsImages>)
{
    for mut timer in death_booms_timer_query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let mut rng = rand::thread_rng();
            let (player_transform, mut player_sprite) = player_query.single_mut();

            player_sprite.color = ITEM_COLORS[player.color_index];
            player.color_index += 1;
            if player.color_index >= ITEM_COLORS.len() {
                player.color_index = 0;
            }

            let boom_pos = Vec3::new(
                player_transform.translation.x + rng.gen_range(-60.0..=60.0),
                player_transform.translation.y + rng.gen_range(-30.0..=30.0),
                player_transform.translation.z
            );

            crate::explosions::spawn_boom(&mut commands, boom_pos, &explosions_images);
        }
    }
}
