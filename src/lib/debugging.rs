use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::{asteroid::Asteroid, bullet::PlayerBullet, defines::WND_RES, shared::Health};

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

pub fn c_bullet_asteroid(
    b_query: Query<(Entity, &Sprite, &Transform), With<PlayerBullet>>,
    mut a_query: Query<(Entity, &Sprite, &Transform, &mut Health), With<Asteroid>>,
    mut commands: Commands,
) {
    for (bul_ent, bul_spr, bul_trans) in b_query.iter() {
        for (ast_ent, ast_spr, ast_trans, mut ast_health) in a_query.iter_mut() {
            // Check first to see if the bullet is within the map's bounds
            let map_width = WND_RES.0 / 2.;
            let map_height = WND_RES.1 / 2.;
            let x_range = -map_width..map_width;
            let y_range = -map_height..map_height;

            if x_range.contains(&bul_trans.translation.x)
                || y_range.contains(&bul_trans.translation.y)
            {
                let collision = collide(
                    bul_trans.translation,
                    bul_spr.custom_size.unwrap(),
                    ast_trans.translation,
                    ast_spr.custom_size.unwrap(),
                );

                if let Some(_) = collision {
                    ast_health.0 -= 1.;
                    commands.entity(bul_ent).despawn();

                    if ast_health.0 <= 0. {
                        commands.entity(ast_ent).despawn();
                    }
                }
            }
        }
    }
}
