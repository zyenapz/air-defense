use bevy::prelude::*;

use super::shared::{Speed, ZnDirection};

#[derive(Bundle)]
pub struct BulletBundle {
    pub(crate) speed: Speed,
    pub(crate) direction: ZnDirection,
}

#[derive(Component)]
pub struct PlayerBullet;

pub fn update_bullet(time: Res<Time>, mut query: Query<(&ZnDirection, &Speed, &mut Transform)>) {
    for (direction, speed, mut transform) in query.iter_mut() {
        transform.translation.x += direction.0.x * speed.0 * time.delta_seconds();
        transform.translation.y += direction.0.y * speed.0 * time.delta_seconds();
    }
}
