use crate::prelude::*;  

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, (
                player_movement,
                player_attack
            )
        );
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_texture = asset_server.load("player_idle.png");

    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform::from_translation(Vec3::new(SCREEN_WIDTH as f32 * 0.5, SCREEN_HEIGHT as f32 * 0.5, 0.0,)),
            ..default()
        },
        Player {
            movement_speed: 500.0,// meters per second
        },
    ));
}

pub fn player_movement(

) {

}

pub fn player_attack(
    
){

}