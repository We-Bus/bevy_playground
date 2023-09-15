#![allow(unused)] // remove this when releasing / optemising

use bevy::{math::Vec3Swizzles, prelude::*};

const TIME_STEP: f32 = 1.0 / 60.0;

const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 600.;

const TILE_SIZE: i32 = 32;

const CHUNK_TILE_SCALE_WIDTH: i32 = 10;
const CHUNK_TILE_SCALE_HEIGHT: i32 = 10;

const CHUNK_WIDTH: i32 = TILE_SIZE * CHUNK_TILE_SCALE_WIDTH;
const CHUNK_HEIGHT: i32 = TILE_SIZE * CHUNK_TILE_SCALE_HEIGHT;

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
struct ChunkTracker {
    current_chunk_x: i32,
    current_chunk_y: i32
}

#[derive(Component)]
struct ChunkRenderer {
    loaded_sprites: Vec<Entity>,
    is_loaded: bool
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    let mut initial_tiles: Vec<Entity> = Vec::new();

    for i in 0..CHUNK_TILE_SCALE_WIDTH {
        for j in 0..CHUNK_TILE_SCALE_HEIGHT {
            let default_tile_texture = asset_server.load("tiles/plaintile.png");

            let tile_x = i * TILE_SIZE;
            let tile_y = j * TILE_SIZE;

            let tile_position = IVec3::new(tile_x, tile_y, 0);
            
            let tile_id = commands.spawn((
                SpriteBundle {
                    texture: default_tile_texture,
                    transform: Transform::from_translation(tile_position.as_vec3()),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(32.0, 32.0)),
                        ..default()
                    },
                    ..default()
                },
            )).id();

            initial_tiles.push(tile_id);
        }
    }

    // 2D orthographic camera
    commands.spawn((
        Camera2dBundle {
            ..default()
        },
        ChunkTracker {
            current_chunk_x : 0,
            current_chunk_y : 0,
        },
        ChunkRenderer {
            is_loaded: false,
            loaded_sprites: initial_tiles
        }
    ));

    let player_texture = asset_server.load("player_idle.png");

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

fn intialize_tiles_and_tracker(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

}

fn tile_update_system(
    mut chunk_query : Query<(&mut ChunkTracker, &mut ChunkRenderer, &Transform)>,
    mut commands : Commands,
) {
    let (mut chunk_tracker,chunk_renderer,transform) = chunk_query.single_mut();
    let translation = transform.translation;
    
    let check_chunk_position = position_to_chunk(translation.x, translation.y);

    if (check_chunk_position.x != chunk_tracker.current_chunk_x) {
        println!("In the X, we moved from {} , to {} ",chunk_tracker.current_chunk_x,check_chunk_position.x);
        chunk_tracker.current_chunk_x = check_chunk_position.x;

        for e in chunk_renderer.loaded_sprites.iter() {
            commands.entity(*e).despawn();
        }
    }

    if (check_chunk_position.y != chunk_tracker.current_chunk_y) {
        println!("In the Y, we moved from {} , to {} ",chunk_tracker.current_chunk_y,check_chunk_position.y);
        chunk_tracker.current_chunk_y = check_chunk_position.y;
    }
}

fn position_to_chunk(x_position:f32,y_position:f32) -> IVec2 {
    let chunk_x = (x_position as i32) / CHUNK_WIDTH;
    let chunk_y = (y_position as i32) / CHUNK_HEIGHT;
    IVec2::new(chunk_x, chunk_y)
}