use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};
use rand_distr::{Distribution, UnitCircle};

use super::{defines::WND_RES, player::Player};

#[derive(Component)]
pub struct Asteroid {
    health: f32,
    direction: Vec2,
    speed: f32,
}

pub fn spawn_asteroid(
    mut commands: Commands,
    mut p_query: Query<&Transform, With<Player>>,
    // TODO: For debugging only, remove later.
    keyboard: Res<Input<KeyCode>>,
) {
    if (keyboard.pressed(KeyCode::F1)) {
        // Randomize spawn position
        let wnd_width = WND_RES.0;
        let wnd_height = WND_RES.1;

        let v: [f64; 2] = UnitCircle.sample(&mut rand::thread_rng());
        println!("{:?} is from the unit circle.", v);

        let ax = v[0] as f32 * 800.;
        let ay = v[1] as f32 * 800.;

        // Randomize speed
        let speed = rand::thread_rng().gen_range(80_f32..=100_f32);

        // Determine direction
        let player_pos = p_query.single();

        let rel_x = player_pos.translation.x - ax;
        let rel_y = player_pos.translation.y - ay;
        let angle = rel_y.atan2(rel_x);

        let ast_direction = Vec2::new(angle.cos(), angle.sin());

        let asteroid = Asteroid {
            health: 1.,
            direction: ast_direction,
            speed: speed,
        };

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(20., 20.)),
                    ..default()
                },
                transform: Transform::from_xyz(ax, ay, 1.),
                ..default()
            })
            .insert(asteroid);
    }
}

pub fn update_asteroid(time: Res<Time>, mut query: Query<(&Asteroid, &mut Transform)>) {
    for (a, mut t) in query.iter_mut() {
        t.translation.x += a.direction.x * a.speed * time.delta_seconds();
        t.translation.y += a.direction.y * a.speed * time.delta_seconds();
    }
}
