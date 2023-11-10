mod player;
mod background;
mod camera;
mod enemy;
mod projectile;

pub mod prelude {

    pub const SCREEN_WIDTH: f32 = 1600.;
    pub const SCREEN_HEIGHT: f32 = 720.;
    pub const SCREEN_WIDTH_HALF: f32 = SCREEN_WIDTH * 0.5;
    pub const SCREEN_HEIGHT_HALF: f32 = SCREEN_HEIGHT * 0.5;
    pub const BACKGROUND_SPRITE_WIDTH: f32 = 1600.; // 201 test value // 
    pub const BACKGROUND_SPRITE_HEIGHT: f32 = 800.; // 113 test value //

    pub use crate::player::PlayerPlugin;
    pub use crate::background::BackgroundPlugin;
    pub use crate::camera::CameraPlugin;
    pub use crate::enemy::EnemyPlugin;
    pub use crate::projectile::ProjectilePlugin;
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub use rand::prelude::*;
    pub use std::collections::HashMap;

    #[derive(Resource)]
    pub struct Random {
    }

    #[derive(Component)]
    pub struct Player {
        pub movement_speed: f32,
        pub health: f32,
        pub max_health: f32,
        pub attack_cooldown: f32,
        pub attack_timer: f32,
    }

    #[derive(Component)]
    pub struct ChunkTracker {
        pub latest_chunk: IVec2,
    }

    #[derive(Component)]
    pub struct BackgroundChunk {}

    #[derive(Component)]
    pub struct BackgroundOrdament {}
    
    #[derive(Component)]
    pub struct Enemy {
        pub movement_speed: f32,
        pub health: f32,
        pub max_health: f32,
        pub attack_damage: f32,
        pub level: i32,
    }

    #[derive(Component)]
    pub struct EnemySpawner {
        pub spawn_countdown: f32,
        pub spawn_after_time: f32,
        pub max_enemy_count: i32,
    }

    #[derive(Component)]
    pub struct Projectile {
        pub just_fired: bool,
        pub velocity: Vec3,
        pub enemies_hit_cooldown: HashMap<u32,f32>,
        pub hits_before_delete: i32,
        pub lifetime: f32,
        pub damage: f32,
    }
}  
