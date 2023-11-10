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
            transform: Transform::from_translation(Vec3::new(0., 0., 2.0,)),
            ..default()
        },
        Player {
            movement_speed: 500.0,
            health: 100.,
            max_health: 100.,
            attack_cooldown: 1.,
            attack_timer: 5.
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time:Res<Time>
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

    let movement_distance = player.movement_speed * time.delta_seconds();
    let translation_delta = player_input * movement_distance;
    transform.translation += translation_delta;
}

pub fn player_attack(
    time:Res<Time>,
    enemy_query: Query<&Transform,(With<Enemy>,Without<Player>)>,
    mut player_query: Query<(&mut Player,&Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let (mut player,player_transfrom) = player_query.single_mut();

    player.attack_timer -= time.delta_seconds();

    if (player.attack_timer > 0.) {
        // attack is on cooldown
        return;
    }
    player.attack_timer = 0.; // making sure it stays at 0

    // If we have no enemies, wait with shooting
    if (enemy_query.iter().count() < 1) {
        return;
    }
    
    let mut closestEnemyPosition: Vec3 = Vec3 { x: f32::MAX, y: f32::MAX, z:f32::MAX };
    let mut closestEnemyLength = f32::MAX;

    // search closed enemy transform
    for enemy_transfrom in enemy_query.iter() {
        let distance = enemy_transfrom.translation.distance(player_transfrom.translation);

        if (distance < closestEnemyLength) {
            closestEnemyPosition = enemy_transfrom.translation;
            closestEnemyLength = distance;
        }
    }

    let delta = (closestEnemyPosition - player_transfrom.translation).normalize();
    let projectile_velocity = delta * 100.;

    let projectile_texture = asset_server.load("projectile.png");

    let projectile_rotation = Quat::from_rotation_arc(Vec3::new(0.0, 1.0, 0.0), delta);
    let projectile_transfrom = Transform::from_translation(player_transfrom.translation).with_rotation(projectile_rotation);

    commands.spawn((
        SpriteBundle {
            texture: projectile_texture,
            transform: projectile_transfrom,
            ..default()
        },
        Projectile {
            just_fired : true,
            velocity : projectile_velocity,
            enemies_hit_cooldown : HashMap::new(),
            lifetime: 2.,
            hits_before_delete: 1,
            damage: 10.
        },
        RigidBody::KinematicPositionBased,
        Collider::capsule(
            Vec2::new(0.,5.),
            Vec2::new(0.,-5.), 
            12.,
        ),
        Restitution::coefficient(0.0),
        LockedAxes::ROTATION_LOCKED,
        Sensor,
    ));

    player.attack_timer = player.attack_cooldown;
}