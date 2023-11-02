use crate::prelude::*;
use rand::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (initialize_projectile))
            .add_systems(Update, (update)
        );
    }
}

pub fn initialize_projectile() {

}

pub fn update() {

}