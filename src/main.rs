#![allow(unused)] // remove this when releasing / optemising

use bevy_playground::prelude::*;
use bevy_rapier2d::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;
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
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugins((
            PlayerPlugin,
            BackgroundPlugin,
            CameraPlugin,
            EnemyPlugin,
        ))
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_systems(Startup,(
                setup,
            )
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
) {

}