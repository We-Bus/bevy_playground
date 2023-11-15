use crate::prelude::*;
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position.before(check_collision),
                                                    check_collision,
                                                    ),
        ).add_systems(PostUpdate, update_lifetime);
    }
}

pub fn update_position(
    mut projectile_query: Query<(&mut Transform,&mut Projectile)>,
    time: Res<Time>
) {
    for (mut transfrom,mut projectile) in projectile_query.iter_mut() {
        transfrom.translation = transfrom.translation + projectile.velocity * time.delta_seconds();
        transfrom.translation.z = 2.0;
        projectile.minimum_alive_frames = i32::clamp(projectile.minimum_alive_frames - 1,0,10);
    }
}

pub fn check_collision (
    rapier_context: Res<RapierContext>,
    mut projectile_col_query: Query<(Entity,&mut Projectile),Without<Enemy>>,
    mut enemy_col_query: Query<(Entity,&mut Enemy),Without<Projectile>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (projectile_entity,mut projectile) in projectile_col_query.iter_mut() {
        for (enemy_entity,mut enemy) in enemy_col_query.iter_mut() {

            if (projectile.enemies_hit_cooldown.contains_key(&enemy_entity.index())) {
                continue; // This has already been hit by this projectile within the cooldown
            }

            let result = rapier_context.intersection_pair(enemy_entity, projectile_entity);
        
            match result {
                None => { /* collision not found */ },
                Some(r) => {
                    if (r == false) { 
                        return; 
                    }
                    enemy.health -= projectile.damage;

                    projectile.enemies_hit_cooldown.insert(enemy_entity.index(), 1.);
                    projectile.hits_before_delete -= 1.;
                    commands.spawn(
                        AudioBundle {
                            source: asset_server.load("sounds/hit_sound.ogg"),
                            settings: PlaybackSettings {
                                mode: bevy::audio::PlaybackMode::Despawn,
                                paused: false,
                                volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.5)),
                                ..default()
                            },
                            ..default()
                        }
                    );
                    
                    if (projectile.hits_before_delete <= 0.01) {
                        projectile.lifetime = 0.;
                    }
                    
                }
            }
        }
    }
}

pub fn update_lifetime(
    mut projectile_query: Query<(Entity,&mut Projectile)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity,mut projectile) in projectile_query.iter_mut(){
        projectile.lifetime -= time.delta_seconds();

        if (projectile.lifetime > 0. || projectile.minimum_alive_frames > 0) {
            continue;
        }
        commands.entity(entity).despawn();
    }
}