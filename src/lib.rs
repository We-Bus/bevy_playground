mod player;
mod background;
mod camera;

pub mod prelude {
    pub const TIME_STEP: f32 = 1.0 / 60.0;
    pub const SCREEN_WIDTH: f32 = 1800.;
    pub const SCREEN_HEIGHT: f32 = 900.;

    pub use crate::player::PlayerPlugin;
    pub use crate::background::BackgroundPlugin;
    pub use crate::camera::CameraPlugin;
    pub use bevy::prelude::*;

    #[derive(Component)]
    pub struct Player {
        pub movement_speed: f32,
    }

    #[derive(Component)]
    pub struct BackgroundChunk {
    }
}  
