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
) {
    commands.spawn((
        BackgroundRenderer {
            loaded_sprites: Vec::new(),
            current_center_chunk: IVec2::new(0,0),
            has_tiles: false,
        },
    ));
}

pub fn update_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut background_query: Query<&mut BackgroundRenderer>,
    camera_query: Query<&Transform,With<Camera2d>>,
) {
    let mut background_renderer = background_query.single_mut();
    let camera_transform = camera_query.single();    

    let current_center_chunk = position_to_chunk(camera_transform.translation.x,camera_transform.translation.y);
    let latest_center_chunk = background_renderer.current_center_chunk;

    if (current_center_chunk.x != latest_center_chunk.x) {
        println!("Chunk X changed from {} to {}",latest_center_chunk.x,current_center_chunk.x);
        background_renderer.current_center_chunk.x = current_center_chunk.x;

        render_chunk(&mut commands,&asset_server,current_center_chunk.x, current_center_chunk.y,&mut background_renderer);
    }

    if (current_center_chunk.y != latest_center_chunk.y) {
        println!("Chunk Y changed from {} to {}",latest_center_chunk.y,current_center_chunk.y);
        background_renderer.current_center_chunk.y = current_center_chunk.y;
        
        render_chunk(&mut commands,&asset_server,current_center_chunk.x, current_center_chunk.y,&mut background_renderer);
    }
}

fn position_to_chunk(x_position:f32,y_position:f32) -> IVec2 {
    let chunk_x = (x_position / CHUNK_WIDTH as f32).floor() as i32;
    let chunk_y = (y_position / CHUNK_HEIGHT as f32).floor() as i32;
    IVec2::new(chunk_x, chunk_y)
}

fn chunk_to_position(chunk_x: i32, chunk_y: i32) -> Vec2 {
    let x_position = chunk_x as f32 * CHUNK_WIDTH as f32;
    let y_position = chunk_y as f32 * CHUNK_HEIGHT as f32;
    Vec2::new(x_position,y_position)
}

fn render_chunk(
    commands:&mut Commands,
    asset_server: &Res<AssetServer>,
    chunk_x: i32,
    chunk_y: i32,
    chunk_renderer: &mut BackgroundRenderer,
) {

    if (chunk_renderer.has_tiles) {
        for entity in chunk_renderer.loaded_sprites.iter() {
            commands.entity(*entity).despawn();
        }
    }

    let x_start = chunk_x as f32 * CHUNK_WIDTH as f32;
    let y_start = chunk_y as f32 * CHUNK_HEIGHT as f32;

    let tile_texture = asset_server.load("tiles/tile.png");

    let mut new_tiles: Vec<Entity> = Vec::new();

    for x in 0..CHUNK_TILE_COUNT_WIDTH {
        for y in 0..CHUNK_TILE_COUNT_HEIGHT {
            let entity = commands.spawn((
                SpriteBundle {
                    texture: tile_texture.clone(),
                    transform: Transform::from_translation(Vec3::new(x_start + (x as f32 * TILE_SIZE as f32),y_start + (y as f32 * TILE_SIZE as f32),0.0,)),
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::BottomLeft,
                        custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                        ..default()
                    },
                    ..default()
                },
            )).id();

            new_tiles.push(entity);
        }
    }

    chunk_renderer.loaded_sprites = new_tiles;
    chunk_renderer.has_tiles = true;
}