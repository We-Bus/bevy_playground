use crate::prelude::*;  

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, (
                player_movement,
                player_attack,
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
            transform: Transform::from_translation(Vec3::new(0., 0., 2.0,)),
            ..default()
        },
        Player {
            movement_speed: 500.0,
            health: 100.,
            max_health: 100.,
        },
        Collider::capsule(
            Vec2::new(-8.,10.),
            Vec2::new(-8.,-20.), 
            32.,
        ),
        RigidBody::KinematicVelocityBased,
        Sensor,
    ));
}

pub fn player_movement(
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

    if player_input.length_squared() > 0.001 {
        player_input = player_input.normalize();
    }

    let movement_distance = player.movement_speed * TIME_STEP;
    let translation_delta = player_input * movement_distance;
    transform.translation += translation_delta;
}

pub fn player_attack(
    
){

}