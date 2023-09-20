use crate::prelude::*;
use std::{thread, time};

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
    textures: Res<Assets<Image>>
) {
    commands.spawn((
        BackgroundRenderer {
            loaded_sprites: Vec::new(),
            current_center_chunk: IVec2::new(0,0),
            has_tiles: false,
        },
    ));

    let background_image_handle: Handle<Image> = asset_server.load("tiles/tile.png");


    commands.spawn((
        SpriteBundle {
            texture: background_image_handle,
            ..default()
        },
    ));
    
}

pub fn update_background(
) {

}