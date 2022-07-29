use bevy::prelude::*;

use super::defines::WND_RES;

pub fn setup_debug_cordon(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(WND_RES.0, WND_RES.1)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}
