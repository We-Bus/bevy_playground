use bevy::ecs::query;

use crate::prelude::*;  

pub struct InGameUIPlugin;

impl Plugin for InGameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player_ui))
        .add_systems(Update, (update_player_healthbar));
    }
}


pub fn spawn_player_ui(
    mut commands: Commands
) {
    let parent_node = (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                width: Val::Percent(5.0),
                height: Val::Percent(2.0),
                //Player is always centered
                left: Val::Percent(47.5),
                right: Val::Auto,
                top: Val::Percent(60.0),
                bottom: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        PlayerUI{},
        Name::new("Player UI"),
    );

    let health_node = (
        NodeBundle {
            style: Style {
                width: Val::Percent(0.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::Rgba { red: (214./225.), green: (40./225.), blue: (40./225.), alpha: (1.0) }),
            ..default()
        },
        PlayerHealthBar{},
        Name::new("Health UI"),
    );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(health_node);
    });
}

fn update_player_healthbar(
    mut ui: Query<&mut Style, With<PlayerHealthBar>>, player: Query<&Player>
) {
    let mut style = ui.single_mut();
    let player = player.single();

    let percent = player.health / player.max_health;
    style.width = Val::Percent(percent * 100.0);
}