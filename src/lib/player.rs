use bevy::{prelude::*, render::color};

use super::defines::{P_INIT_ARMOR, P_INIT_HEALTH};

#[derive(Component)]
struct Player {
    health: f64,
    armor: f64,
}

pub fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(Player {
            health: P_INIT_HEALTH,
            armor: P_INIT_ARMOR,
        });
}
