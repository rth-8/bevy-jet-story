use bevy_kira_audio::prelude::*;
use bevy::prelude::{App, Plugin, StartupStage, Handle, Commands, ResMut, AssetServer};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
        .add_plugin(AudioPlugin)
        .add_audio_channel::<SfxChannel>()
        .add_audio_channel::<DamageChannel>()
        .add_audio_channel::<Shooting01Channel>()
        .add_audio_channel::<Shooting05Channel>()
        .add_audio_channel::<Shooting06Channel>()
        .add_audio_channel::<Shooting08Channel>()
        .add_audio_channel::<Shooting09Channel>()
        .add_audio_channel::<DeathSoundChannel>();
    }
}

pub struct Sounds {
    pub cannon_shot: Handle<AudioSource>,
    pub boom: Handle<AudioSource>,
    pub boom_base: Handle<AudioSource>,
    pub damage: Handle<AudioSource>,
    pub enemy_damage: Handle<AudioSource>,
    pub enemy_01_shot: Handle<AudioSource>,
    pub enemy_01_shot_counter: u8,
    pub enemy_02_shot: Handle<AudioSource>,
    pub enemy_03_13_shot: Handle<AudioSource>,
    pub enemy_05_shot: Handle<AudioSource>,
    pub enemy_05_shot_counter: u8,
    pub enemy_06_shot: Handle<AudioSource>,
    pub enemy_06_shot_counter: u8,
    pub enemy_07_shot: Handle<AudioSource>,
    pub enemy_08_shot: Handle<AudioSource>,
    pub enemy_08_shot_counter: u8,
    pub enemy_09_launch: Handle<AudioSource>,
    pub enemy_09_shot: Handle<AudioSource>,
    pub enemy_09_shot_counter: u8,
    pub enemy_10: Handle<AudioSource>,
    pub special_launch: Handle<AudioSource>,
    pub ball_bounce: Handle<AudioSource>,
    pub get_item: Handle<AudioSource>,
    pub ship_damage: Handle<AudioSource>,
    pub death: Handle<AudioSource>,
}

pub struct SfxChannel;

pub struct DamageChannel;

pub struct Shooting01Channel;

pub struct Shooting05Channel;

pub struct Shooting06Channel;

pub struct Shooting08Channel;

pub struct Shooting09Channel;

pub struct DeathSoundChannel;

fn load_audio(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    println!("Load sounds");
    commands.insert_resource(Sounds {
        cannon_shot: asset_server.load("sounds/ship_cannon_shot.wav"),
        boom: asset_server.load("sounds/boom.wav"),
        boom_base: asset_server.load("sounds/boom_base.wav"),
        damage: asset_server.load("sounds/damage.wav"),
        enemy_damage: asset_server.load("sounds/enemy_damage.wav"),
        enemy_01_shot: asset_server.load("sounds/enemy_01_shot.wav"),
        enemy_02_shot: asset_server.load("sounds/enemy_02_shot.wav"),
        enemy_03_13_shot: asset_server.load("sounds/enemy_03_13_shot.wav"),
        enemy_05_shot: asset_server.load("sounds/enemy_05_shot.wav"),
        enemy_06_shot: asset_server.load("sounds/enemy_06_shot.wav"),
        enemy_07_shot: asset_server.load("sounds/enemy_07_shot.wav"),
        enemy_08_shot: asset_server.load("sounds/enemy_08_shot.wav"),
        enemy_09_launch: asset_server.load("sounds/enemy_09_launch.wav"),
        enemy_09_shot: asset_server.load("sounds/enemy_09_shot.wav"),
        enemy_10: asset_server.load("sounds/enemy_10.wav"),
        special_launch: asset_server.load("sounds/special_launch.wav"),
        ball_bounce: asset_server.load("sounds/ball_bounce.wav"),
        get_item: asset_server.load("sounds/get_item.wav"),
        ship_damage: asset_server.load("sounds/ship_damage.wav"),
        death: asset_server.load("sounds/death.wav"),
        
        enemy_01_shot_counter: 0,
        enemy_05_shot_counter: 0,
        enemy_06_shot_counter: 0,
        enemy_08_shot_counter: 0,
        enemy_09_shot_counter: 0,
    });
}
