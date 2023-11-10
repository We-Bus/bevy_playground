use crate::prelude::*;  

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_enemy_spawner)
        .add_systems(Update, (
                update_enemy_spawner,
                enemy_movement_physics,
                get_collisions
            )
        )
        .add_systems(PostUpdate, check_health);
    }
}

fn init_enemy_spawner(
    mut commands: Commands,
) {
    commands.spawn((
        EnemySpawner {
            spawn_countdown: 2.0,
            spawn_after_time: 10.0,
            max_enemy_count: 10,
        },
    ));
}

fn update_enemy_spawner(
    mut enemy_spawner_query: Query<&mut EnemySpawner>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    enemies_query: Query<&Enemy>,
    player_query: Query<&Transform,(With<Player>,Without<Enemy>)>,
) {
    let enemy_count = enemies_query.iter().count();
    
    let mut enemy_spawner = enemy_spawner_query.single_mut();

    let player_transform = player_query.single();

    if enemy_count as i32 > enemy_spawner.max_enemy_count {
        return;
    }

    enemy_spawner.spawn_countdown -= time.delta_seconds();

    if enemy_spawner.spawn_countdown <= 0. {
        let enemy_texture = asset_server.load("zombie_idle.png");

        let mut spawn_position = Vec3::ZERO;
        spawn_position.z = 3.;

        let mut rng = rand::thread_rng();
        let x_side = if rng.gen::<bool>() {1.} else {-1.};
        let y_side = if rng.gen::<bool>() {1.} else {-1.};
        
        let spawn_on_side = rng.gen::<bool>();

        if spawn_on_side {
            spawn_position.x = player_transform.translation.x + (x_side * SCREEN_WIDTH_HALF) + (32. * x_side);
            spawn_position.y = player_transform.translation.y + (rng.gen::<f32>() * SCREEN_HEIGHT) - SCREEN_HEIGHT_HALF;
        }
        else {
            spawn_position.x = player_transform.translation.x + (rng.gen::<f32>() * SCREEN_WIDTH) - SCREEN_WIDTH_HALF;
            spawn_position.y = player_transform.translation.y + (y_side * SCREEN_HEIGHT_HALF) + (32. * y_side);
        }
        commands.spawn((
            SpriteBundle {
                texture: enemy_texture,
                transform: Transform::from_translation(spawn_position),
                sprite: Sprite { 
                    anchor: bevy::sprite::Anchor::Center,
                    ..default()
                },
                ..default()
            },
            Enemy {
                movement_speed: 400.0,
                health: 100.,
                max_health: 100.,
                attack_damage: 10.,
                level: 1,
            },
            RigidBody::Dynamic,
            Collider::capsule(
                Vec2::new(-8.,10.),
                Vec2::new(-8.,-20.), 
                32.,
            ),
            Restitution::coefficient(0.0),
            LockedAxes::ROTATION_LOCKED,
            Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.
            },
            ActiveEvents::COLLISION_EVENTS,
        ));

        enemy_spawner.spawn_countdown = enemy_spawner.spawn_after_time;
        println!("Enemy spawned!");
    } 
}

fn enemy_movement_physics(
    mut enemy_query: Query<(&mut Velocity,&Transform),(With<Enemy>,Without<Player>)>,    
    player_query: Query<&Transform,(With<Player>,Without<Enemy>)>,
    time:Res<Time>,
) {
    let player_transform = player_query.single();
    
    for enemy_velocity in enemy_query.iter_mut() {
        
        let (mut velocity,enemy_transform) = enemy_velocity;

        let mut enemy_movement : Vec2 = Vec2::ZERO;

        enemy_movement.x = player_transform.translation.x - enemy_transform.translation.x;
        enemy_movement.y = player_transform.translation.y - enemy_transform.translation.y;

        if enemy_movement.length_squared() > 0.001 {
            enemy_movement = enemy_movement.normalize();
        }
        
        enemy_movement = enemy_movement * (20000. * time.delta_seconds());
        
        velocity.linvel = enemy_movement;
    }
}


fn get_collisions(
    rapier_context: Res<RapierContext>,
    mut player_col_query: Query<(Entity,&mut Player),Without<Enemy>>,
    enemy_col_query: Query<(Entity,&Enemy),Without<Player>>,
    time: Res<Time>
) {
    let (player_entity,mut player) = player_col_query.single_mut();

    for (enemy_entity,enemy) in enemy_col_query.iter() {
        let result = rapier_context.intersection_pair(enemy_entity, player_entity);
        
        match result {
            None => { /* collision not found */ },
            Some(r) => {
                if (r == false) { 
                    return; 
                }
                
                // collision found
                player.health -= enemy.attack_damage * time.delta_seconds();
                println!("{}",player.health); // intersetction was there but no collision ?

            }
        }
    }
}

fn check_health () 
{

}