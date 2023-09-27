use crate::prelude::*;  

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_enemy_spawner)
        .add_systems(Update, (
                update_enemy_spawner,
                enemy_movement,
            )
        );
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
    enemies_query: Query<&Enemy>,
    player_query: Query<&Transform,(With<Player>,Without<Enemy>)>,
) {
    let enemy_count = enemies_query.iter().count();
    
    let mut enemy_spawner = enemy_spawner_query.single_mut();

    let player_transform = player_query.single();

    if enemy_count as i32 > enemy_spawner.max_enemy_count {
        return;
    }

    enemy_spawner.spawn_countdown -= TIME_STEP;

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
                ..default()
            },
            Enemy {
                movement_speed: 400.0,
                health: 100,
                max_health: 100,
                attack_damage: 10,
                level: 1,
            },
            RigidBody::Dynamic,
        ))
            .insert(Collider::capsule(
                Vec2::new(-8.,10.),
                Vec2::new(-8.,-20.), 
                32.,
            ))
            .insert(Restitution::coefficient(0.0))
            .insert(LockedAxes::ROTATION_LOCKED);

        enemy_spawner.spawn_countdown = enemy_spawner.spawn_after_time;
        println!("Enemy spawned!");
    } 
}

// base it on velocity of enemy and not transform
fn enemy_movement(
    mut enemy_query: Query<(&mut Transform,&mut Enemy),Without<Player>>,
    player_query: Query<&Transform,(With<Player>,Without<Enemy>)>,
) {
    let player_transform = player_query.single();

    for enemy in enemy_query.iter_mut(){
        let (mut enemy_transform, mut enemy) = enemy;

        // Let the enemy move towards the player with the movement speed of the enemy

        let mut enemy_movement : Vec3 = Vec3::ZERO;

        enemy_movement.x = player_transform.translation.x - enemy_transform.translation.x;
        enemy_movement.y = player_transform.translation.y - enemy_transform.translation.y;

        if enemy_movement.length_squared() > 0.001 {
            enemy_movement = enemy_movement.normalize();
        }

        let movement_distance = enemy.movement_speed * TIME_STEP;
        let translation_delta = enemy_movement * movement_distance;
        enemy_transform.translation += translation_delta;
        
    }
}