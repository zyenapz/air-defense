use bevy::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, UnitCircle};

use super::{
    defines::{AST_MAX_SPEED, AST_MIN_SPEED, WND_RES},
    player::Player,
    shared::{Health, Speed, ZnDirection},
};

#[derive(Bundle)]
pub struct AsteroidBundle {
    health: Health,
    direction: ZnDirection,
    speed: Speed,
}

#[derive(Component)]
pub struct Asteroid;

pub fn spawn_asteroid(
    mut commands: Commands,
    p_query: Query<&Transform, With<Player>>,
    // TODO: For debugging only, remove later.
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::F1) {
        // Randomize spawn position
        let rx = WND_RES.0 * 0.8;
        let ry = WND_RES.1 * 0.8;

        let v: [f64; 2] = UnitCircle.sample(&mut rand::thread_rng());

        let ax = v[0] as f32 * rx;
        let ay = v[1] as f32 * ry;

        // Determine direction
        let player_pos = p_query.single();

        let rel_x = player_pos.translation.x - ax;
        let rel_y = player_pos.translation.y - ay;
        let angle = rel_y.atan2(rel_x);

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(32., 32.)),
                    ..default()
                },
                transform: Transform::from_xyz(ax, ay, 1.),
                ..default()
            })
            .insert_bundle(AsteroidBundle {
                health: Health(2.),
                direction: ZnDirection(Vec2::new(angle.cos(), angle.sin())),
                speed: Speed(rand::thread_rng().gen_range(AST_MIN_SPEED..=AST_MAX_SPEED)),
            })
            .insert(Asteroid);
    }
}

pub fn update_asteroid(
    time: Res<Time>,
    mut query: Query<(&Speed, &ZnDirection, &mut Transform), With<Asteroid>>,
) {
    for (speed, direction, mut transform) in query.iter_mut() {
        transform.translation.x += direction.0.x * speed.0 * time.delta_seconds();
        transform.translation.y += direction.0.y * speed.0 * time.delta_seconds();
    }
}
