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
            spawn_after_time: 1.0,
            max_enemy_count: 50,
            max_mobs_per_spawn: 5,
        },
    ));
}

fn update_enemy_spawner(
    mut enemy_spawner_query: Query<&mut EnemySpawner>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    enemies_query: Query<&Enemy>,
    player_query: Query<(&Transform,&Level),Without<Enemy>>,
) {
    let enemy_count = enemies_query.iter().count();
    let mut enemy_spawner = enemy_spawner_query.single_mut();
    let (player_transform,player_level) = player_query.single();

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
                movement_speed: 245.0 + (player_level.level as f32 * 10.5),
                health: 100. + (player_level.level as f32 * 5.),
                max_health: 100. + (player_level.level as f32 * 5.),
                attack_damage: 10. + (player_level.level as f32 * 5.),
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
            AudioBundle {
                source: asset_server.load("sounds/hit_sound.ogg"),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Once,
                    paused: true,
                    volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.5)),
                    ..default()
                },
                ..default()
            }
        ));

        enemy_spawner.spawn_countdown = enemy_spawner.spawn_after_time;
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
    time: Res<Time>,
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
            }
        }
    }
}

fn check_health (
    enemy_query: Query<(Entity,&Enemy),Without<Player>>,
    mut player_query: Query<(&mut Player,&mut Level),Without<Enemy>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) 
{
    let mut deathCount = 0;

    for (entity,enemy) in enemy_query.iter() 
    {
        if (enemy.health <= 0.) {
            commands.entity(entity).despawn();
            deathCount += 1;
            commands.spawn(
                AudioBundle {
                    source: asset_server.load("sounds/death_sound.ogg"),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        paused: false,
                        volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.5)),
                        ..default()
                    },
                    ..default()
                }
            );
        }
    }

    if (deathCount > 0) {
        let (mut player,mut level) = player_query.single_mut();
        
        level.experience += 2. * deathCount as f32;

        while (level.experience >= level.experience_to_next_level){
            level.level += 1;
            level.experience = level.experience - level.experience_to_next_level;
            level.experience_to_next_level += 2.;

            player.attack_cooldown = f32::clamp(player.base_attack_cooldown - (0.01 * level.level as f32), 0.005, 10.);
            player.movement_speed = 10. * level.level as f32 + player.base_movement_speed;
            println!("Level up!");
        }
    }
}