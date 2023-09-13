use bevy::{math::Vec3Swizzles, prelude::*};

const TIME_STEP: f32 = 1.0 / 60.0;
const TILE_SIZE: f32 = 32.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Insert Game Name".into(),
                    resolution: (800., 600.).into(),
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
        .add_systems(Startup,setup)
        .add_systems(
            FixedUpdate,
            (
                player_movement_system,
                move_camera_system,
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

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    let player_texture = asset_server.load("player_idle.png");
    
    let default_tile_texture = asset_server.load("tiles/plaintile.png");

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

    let player_x: f32 = 0.;
    let player_y: f32 = 0.;
    
    // now loop over a 9 by 9 grid to place tile
    for x in -1..=1 {
        for y in -1..=1 {
            let tile_x = x as f32 * TILE_SIZE;
            let tile_y = y as f32 * TILE_SIZE;

            let tile_position = Vec3::new(tile_x + ((x * 5) as f32), tile_y + ((y * 5) as f32), 0.0);
            commands.spawn((
                SpriteBundle {
                    texture: default_tile_texture.clone(),
                    transform: Transform::from_translation(tile_position),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(32.0, 32.0)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
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
    let (_, mut transform) = camera_query.single_mut();
    let (_,player_transform) = player_query.single();

    let player_x = player_transform.translation.x;
    let player_y = player_transform.translation.y;

    let camera_x = transform.translation.x;
    let camera_y = transform.translation.y;

    let max_camera_bounds = camera_x + (800./2.) - 50.;
    let min_camera_bounds = camera_x - (800./2.) + 50.;
    
    let translation = &mut transform.translation;

    let mut direction = Vec3::ZERO;
    
    if (player_x > max_camera_bounds) {
        let delta = player_x - max_camera_bounds;
        direction.x = delta;
        *translation += direction * 100.0 * TIME_STEP; // Adjust the speed by multiplying with a time step
    }

    if (player_x < min_camera_bounds) {
        let delta = player_x - min_camera_bounds;
        direction.x = delta;
        *translation += direction * 100.0 * TIME_STEP; // Adjust the speed by multiplying with a time step
    }
}