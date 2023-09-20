use crate::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_background)
            .add_systems(Update, (update_background,)
        );
    }
}

pub fn initialize_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let background_image_handle: Handle<Image> = asset_server.load("default_background.png");

    for x in -1..2 {
        for y in -1..2 {
            let chunk_position = chunk_index_to_chunk_position(x,y); 
            commands.spawn((
                SpriteBundle {
                    texture: background_image_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(chunk_position.x,chunk_position.y,0.)),
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::BottomLeft,
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
    mut commands: Commands,
    camera_query: Query<&Transform,(With<Camera2d>,Without<BackgroundChunk>)>,
    mut background_query: Query<&Transform,With<BackgroundChunk>>
) {
    for bg_transfrom in background_query.iter() {
        
    }
}

fn position_to_chunk(x_position: f32, y_position: f32) -> IVec2 {
    let chunk_x = (x_position / SCREEN_WIDTH as f32).floor() as i32;
    let chunk_y = (y_position / SCREEN_HEIGHT as f32).floor() as i32;
    IVec2 {x:chunk_x,y:chunk_y}
}

fn chunk_index_to_chunk_position(x_chunk: i32, y_chunk: i32) -> Vec2 {
    let chunk_pos_x = x_chunk as f32 * SCREEN_WIDTH;
    let chunk_pos_y = y_chunk as f32 * SCREEN_HEIGHT;
    Vec2 {x:chunk_pos_x,y:chunk_pos_y}
}
