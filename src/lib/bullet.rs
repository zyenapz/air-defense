use bevy::prelude::*;

use super::{
    defines::WND_RES,
    shared::{Speed, ZnDirection},
};

#[derive(Bundle)]
pub struct BulletBundle {
    pub(crate) speed: Speed,
    pub(crate) direction: ZnDirection,
}

#[derive(Component)]
pub struct PlayerBullet;

pub fn update_bullet(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &ZnDirection, &Speed, &mut Transform)>,
) {
    for (entity, direction, speed, mut transform) in query.iter_mut() {
        transform.translation.x += direction.0.x * speed.0 * time.delta_seconds();
        transform.translation.y += direction.0.y * speed.0 * time.delta_seconds();

        if (transform.translation.x < -WND_RES.0 || transform.translation.x > WND_RES.1)
            || (transform.translation.y < -WND_RES.1 || transform.translation.y > WND_RES.1)
        {
            commands.entity(entity).despawn();
        }
    }
}
