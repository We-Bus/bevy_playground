mod player;
mod background;

pub mod prelude {
    pub const TIME_STEP: f32 = 1.0 / 60.0;
    pub const SCREEN_WIDTH: f32 = 800.;
    pub const SCREEN_HEIGHT: f32 = 600.;
    pub const TILE_SIZE: i32 = 32;
    pub const CHUNK_TILE_SCALE_WIDTH: i32 = 10;
    pub const CHUNK_TILE_SCALE_HEIGHT: i32 = 10;
    pub const CHUNK_WIDTH: i32 = TILE_SIZE * CHUNK_TILE_SCALE_WIDTH;
    pub const CHUNK_HEIGHT: i32 = TILE_SIZE * CHUNK_TILE_SCALE_HEIGHT;

    pub use crate::player::PlayerPlugin;
    pub use crate::background::BackgroundPlugin;
    pub use bevy::prelude::*;

    #[derive(Component)]
    pub struct Player {
        pub movement_speed: f32,
    }

    #[derive(Component)]
    pub struct BackgroundRenderer {
        pub loaded_sprites: Vec<Entity>,
        pub current_center_chunk: IVec2,
    }
}  
