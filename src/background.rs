use crate::prelude::*;
use rand::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (initialize_background,intiialize_ordaments))
            .add_systems(Update, (update_background,update_ordaments)
        );
    }
}

pub fn initialize_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let background_image_handle: Handle<Image> = asset_server.load("default_background.png");

    commands.spawn(ChunkTracker {latest_chunk: IVec2{x:0,y:0}});

    for x in -1..2 {
        for y in -1..2 {
            let chunk_position = chunk_index_to_chunk_position(x,y);
            commands.spawn((
                SpriteBundle {
                    texture: background_image_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(chunk_position.x,chunk_position.y,0.)),
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::Center,
                        custom_size: Some(Vec2{x:BACKGROUND_SPRITE_WIDTH,y:BACKGROUND_SPRITE_HEIGHT}),
                        ..default()
                    },
                    ..default()
                },
                BackgroundChunk {},
            ));
        }
    }

}

pub fn update_background(
    mut background_query: Query<&mut Transform,With<BackgroundChunk>>,
    mut chunk_query: Query<&mut ChunkTracker,Without<BackgroundChunk>>,
    camera_query: Query<&Transform,(With<Camera2d>,Without<BackgroundChunk>)>,

) {
    let mut chunk_tracker = chunk_query.single_mut();
    let camera_transform = camera_query.single();
    let current_chunk = position_to_chunk(camera_transform.translation.x, camera_transform.translation.y);
    
    if (chunk_tracker.latest_chunk.x == current_chunk.x && chunk_tracker.latest_chunk.y == current_chunk.y) {
        return;
    }

    let mut foo = 0;

    for mut bg_transfrom in background_query.iter_mut() {
        let translation = bg_transfrom.translation;

        if (camera_transform.translation.x < (bg_transfrom.translation.x - BACKGROUND_SPRITE_WIDTH * 1.5)) { // *   1.5 for a threshold
            let new_x= translation.x - (BACKGROUND_SPRITE_WIDTH * 3.); // 3 here is the number of tiles in 1 row or colum
            bg_transfrom.translation.x = new_x;
            foo += 1;
        }
        else if (camera_transform.translation.x > (bg_transfrom.translation.x + BACKGROUND_SPRITE_WIDTH * 1.5)) {
            let new_x = translation.x + (BACKGROUND_SPRITE_WIDTH * 3.);
            bg_transfrom.translation.x = new_x;
            foo += 1;
        }

        if (camera_transform.translation.y < (bg_transfrom.translation.y - BACKGROUND_SPRITE_HEIGHT * 1.5)) {
            let new_y = translation.y - (BACKGROUND_SPRITE_HEIGHT * 3.);
            bg_transfrom.translation.y = new_y;
            foo += 1;
        }
        else if (camera_transform.translation.y > (bg_transfrom.translation.y + BACKGROUND_SPRITE_HEIGHT * 1.5)) {
            let new_y = translation.y + (BACKGROUND_SPRITE_HEIGHT * 3.);
            bg_transfrom.translation.y = new_y;
            foo += 1;
        }
    }

    chunk_tracker.latest_chunk.x = current_chunk.x;
    chunk_tracker.latest_chunk.y = current_chunk.y;
}

fn position_to_chunk(x_position: f32, y_position: f32) -> IVec2 {
    let chunk_x = (x_position / BACKGROUND_SPRITE_WIDTH as f32).round() as i32;
    let chunk_y = (y_position / BACKGROUND_SPRITE_HEIGHT as f32).round() as i32;
    IVec2 {x:chunk_x,y:chunk_y}
}

fn chunk_index_to_chunk_position(x_chunk: i32, y_chunk: i32) -> Vec2 {
    let chunk_pos_x = x_chunk as f32 * BACKGROUND_SPRITE_WIDTH;
    let chunk_pos_y = y_chunk as f32 * BACKGROUND_SPRITE_HEIGHT;
    Vec2 {x:chunk_pos_x,y:chunk_pos_y}
}

pub fn intiialize_ordaments(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ordament_count = 25;
    let ordament_sprit_handle: Handle<Image> = asset_server.load("flower.png");
    let mut rng = rand::thread_rng();

    for i in 0..ordament_count {
        let random_x:f32 = (rng.gen::<f32>() * (SCREEN_WIDTH + 32.) as f32) - SCREEN_WIDTH * 0.5;
        let random_y:f32 = (rng.gen::<f32>() * (SCREEN_HEIGHT + 32.) as f32) - SCREEN_HEIGHT * 0.5;

        commands.spawn((
            SpriteBundle {
                texture: ordament_sprit_handle.clone(),
                transform: Transform::from_translation(Vec3::new(random_x,random_y,1.)),
                sprite: Sprite {
                    custom_size: Some(Vec2{x:64.,y:64.}),
                    ..default()
                },
                ..default()
            },
            BackgroundOrdament {},
        ));
    }
}

pub fn update_ordaments (
    mut ordament_query: Query<&mut Transform,With<BackgroundOrdament>>,
    camera_query: Query<&Transform,(With<Camera2d>,Without<BackgroundOrdament>)>
) {
    let camera_pos = camera_query.single().translation;
    let camera_x = camera_pos.x;
    let camera_y = camera_pos.y;

    let mut rng = rand::thread_rng();

    let spawn_x_axis = SCREEN_WIDTH_HALF + 32.;
    let spawn_y_axis = SCREEN_HEIGHT_HALF + 32.;

    let max_x_distance:f32 = spawn_x_axis + 20.;
    let max_y_distance:f32 = spawn_y_axis + 20.;

    for mut ordament in ordament_query.iter_mut() {
        let ordament_pos = ordament.translation;
        let ordament_x = ordament_pos.x;
        let ordament_y = ordament_pos.y;

        if ordament_x < (camera_x - max_x_distance) {
            let new_x = camera_x + spawn_x_axis;
            let new_y = camera_y + (rng.gen::<f32>() * SCREEN_HEIGHT) - SCREEN_HEIGHT_HALF;

            ordament.translation.x = new_x;
            ordament.translation.y = new_y;
        }
        else if ordament_x > (camera_x + max_x_distance) {
            let new_x = camera_x - spawn_x_axis;
            let new_y = camera_y + (rng.gen::<f32>() * SCREEN_HEIGHT) - SCREEN_HEIGHT_HALF;

            ordament.translation.x = new_x;
            ordament.translation.y = new_y;
        }
        else if ordament_y < camera_y - max_y_distance {
            let new_x = camera_x + (rng.gen::<f32>() * SCREEN_WIDTH) - SCREEN_WIDTH_HALF;
            let new_y = camera_y + spawn_y_axis;

            ordament.translation.x = new_x;
            ordament.translation.y = new_y;
        }
        else if ordament_y > camera_y + max_y_distance {
            let new_x = camera_x + (rng.gen::<f32>() * SCREEN_WIDTH) - SCREEN_WIDTH_HALF;
            let new_y = camera_y - spawn_y_axis;

            ordament.translation.x = new_x;
            ordament.translation.y = new_y;
        }
    }
}
