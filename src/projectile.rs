use crate::prelude::*;
use rand::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position.before(check_collision),
                                                    check_collision.before(check_collision_cooldown),
                                                    check_collision_cooldown,
                                                    ),
        ).add_systems(PostUpdate, update_lifetime);
    }
}

pub fn update_position(
    mut projectile_query: Query<(&mut Transform,&Projectile)>,
    time: Res<Time>
) {
    for (mut transfrom,projectile) in projectile_query.iter_mut() {
        transfrom.translation = transfrom.translation + projectile.velocity * time.delta_seconds();
    }
}

pub fn check_collision (
    rapier_context: Res<RapierContext>,
    mut projectile_col_query: Query<(Entity,&mut Projectile),Without<Enemy>>,
    mut enemy_col_query: Query<(Entity,&mut Enemy),Without<Projectile>>,
    time: Res<Time>
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

                    println!("enemy hp:{}",enemy.health);

                    projectile.enemies_hit_cooldown.insert(enemy_entity.index(), 1.);
                    projectile.hits_before_delete -= 1;
                    
                    if (projectile.hits_before_delete >= 0) {
                        projectile.lifetime = 0.;
                    }
                }
            }
        }
    }
}

pub fn check_collision_cooldown() {
    
}

pub fn update_lifetime(
    mut projectile_query: Query<(Entity,&mut Projectile)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity,mut projectile) in projectile_query.iter_mut(){
        projectile.lifetime -= time.delta_seconds();

        if (projectile.lifetime > 0.) {
            continue;
        }

        commands.entity(entity).despawn();
    }
}