#![allow(unused)] // remove this when releasing / optemising

use bevy_playground::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Insert Game Name".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),            
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(GlobalVolume::new(0.25))
        .add_plugins((
            PlayerPlugin,
            BackgroundPlugin,
            CameraPlugin,
            EnemyPlugin,
            ProjectilePlugin,
            InGameUIPlugin,
        ))
        .add_systems(Startup,(setup,start_audio)
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
) {
}

fn start_audio(
    mut commands: Commands, 
    asset_server: Res<AssetServer>) 
    {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/background_music.ogg"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                paused: false,
                volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.15)),
                ..default()
            },
            ..default()
        },
        BackgroundMusic,
    ));
}