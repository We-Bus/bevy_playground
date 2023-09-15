use crate::prelude::*;  

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_background)
        .add_systems(Update, (
                update_background,
            )
        );
    }
}

pub fn initialize_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let tile_texture = asset_server.load("tiles/tile.png");

    for x in 0..CHUNK_TILE_SCALE_WIDTH {
        for y in 0..CHUNK_TILE_SCALE_HEIGHT {
            commands.spawn((
                SpriteBundle {
                    texture: tile_texture.clone(),
                    transform: Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE as f32,y as f32 * TILE_SIZE as f32,0.0,)),
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::BottomLeft,
                        custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }

    commands.spawn((
        BackgroundRenderer {
            loaded_sprites: Vec::new(),
            current_center_chunk: IVec2::new(0,0),
        },
    ));
}

pub fn update_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut background_query: Query<(&mut BackgroundRenderer,(&Transform,With<Camera2d>))>,
) {
    let (mut background_renderer,(camera_transform,_)) = background_query.single_mut();
    
    let current_center_chunk = position_to_chunk(camera_transform.translation.x,camera_transform.translation.y);
    let latest_center_chunk = background_renderer.current_center_chunk;

    if (current_center_chunk.x != latest_center_chunk.x) {
        println!("Chunk X changed from {} to {}",latest_center_chunk.x,current_center_chunk.x);
        background_renderer.current_center_chunk.x = current_center_chunk.x;
    }

    if (current_center_chunk.y != latest_center_chunk.y) {
        println!("Chunk Y changed from {} to {}",latest_center_chunk.y,current_center_chunk.y);
        background_renderer.current_center_chunk.y = current_center_chunk.x;
    }
}

fn position_to_chunk(x_position:f32,y_position:f32) -> IVec2 {
    let chunk_x = (x_position as i32) / CHUNK_WIDTH;
    let chunk_y = (y_position as i32) / CHUNK_HEIGHT;
    IVec2::new(chunk_x, chunk_y)
}