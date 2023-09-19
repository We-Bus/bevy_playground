use crate::prelude::*;  

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
        .add_systems(Update, (
                update_camera,
            )
        );
    }
}

fn setup(
    mut commands: Commands,
) {
    // 2D orthographic camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(SCREEN_WIDTH as f32 * 0.5, SCREEN_HEIGHT as f32 * 0.5, 0.0)),
            ..default()
        },
    ));
}

fn update_camera(
    mut commands: Commands,
    mut camera_query: Query<&mut Transform,(With<Camera2d>,Without<Player>)>,
    player_query: Query<&Transform,(With<Player>,Without<Camera2d>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}