#![allow(unused)] // remove this when releasing / optemising

use bevy::{math::Vec3Swizzles, prelude::*};

const TIME_STEP: f32 = 1.0 / 60.0;

const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 600.;

const TILE_SIZE: f32 = 32.0;

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
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_systems(Startup,(
                setup,
                intialize_tiles_and_tracker.after(setup),
            )
        )
        .add_systems(
            FixedUpdate,
            (
                player_movement_system,
                move_camera_system,
                tile_update_system,
            ),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

/// player component
#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
}

#[derive(Component)]
struct ScreenTileTracker {
    current_x: i32,
    current_y: i32
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    let player_texture = asset_server.load("player_idle.png");
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            ..default()
        },
        Player {
            movement_speed: 500.0,// meters per second
        },
    ));
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();

    let mut player_input : Vec3 = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        player_input.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        player_input.y += -1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        player_input.x += -1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        player_input.x += 1.0;
    }

    if (player_input.length_squared() > 0.001) {
        player_input = player_input.normalize();
    }

    let movement_distance = player.movement_speed * TIME_STEP;
    let translation_delta = player_input * movement_distance;
    transform.translation += translation_delta;
}

fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&Camera2d, &mut Transform),Without<Player>>,
    player_query: Query<(&Player,&Transform)>,
) {
    let (_,mut transform) = camera_query.single_mut();
    let (_,player_transform) = player_query.single();

    transform.translation = player_transform.translation; 
}

struct ScreenTile {
    x: i32,
    y: i32,
}

struct Chunk { // 16 x 16 

}

fn intialize_tiles_and_tracker(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    commands.spawn((
        ScreenTileTracker {
            current_x:0,
            current_y:0,
        }
    ));
}

fn tile_update_system(
    camera_query : Query<&Transform,With<Camera2d>>,
    mut query_tile_tracker: Query<&mut ScreenTileTracker>,
    mut commands : Commands,
) {
    let camera_transform = camera_query.single();
    let tile_tracker = query_tile_tracker.single();

    let camera_x = camera_transform.translation.x;
    let camera_y = camera_transform.translation.y;

    let x_tile_index = (camera_x / TILE_SIZE).floor() as i32;
    let y_tile_index = (camera_y / TILE_SIZE).floor() as i32;    
}

fn get_tiles_from_position(x_position:i32,y_position:i32) -> [ScreenTile;9]
{
    let screen_tiles: [ScreenTile;9] = [
        ScreenTile{x:x_position-1,  y:y_position+1}, // tile top left
        ScreenTile{x:x_position,    y:y_position+1}, // tile top middel
        ScreenTile{x:x_position+1,  y:y_position+1}, // tile top right

        ScreenTile{x:x_position-1,  y:y_position},   // tile side left
        ScreenTile{x:x_position,    y:y_position},   // tile currently standing on
        ScreenTile{x:x_position+1,  y:y_position},   // tile side right

        ScreenTile{x:x_position-1,  y:y_position-1}, // tile bottom left
        ScreenTile{x:x_position,    y:y_position-1}, // tile bottom middel
        ScreenTile{x:x_position+1,  y:y_position-1}, // tile bototm right
    ];

    return screen_tiles;
}
