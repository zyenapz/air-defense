use bevy::prelude::*;

use super::player::Station;

#[derive(Component)]
pub struct Asteroid {
    health: f32,
    direction: Vec2,
    speed: f32,
}

pub fn spawn_asteroid(
    mut commands: Commands,
    // TODO: For debugging only, remove later.
    keyboard: Res<Input<KeyCode>>,
) {
    if (keyboard.pressed(KeyCode::F1)) {
        let asteroid = Asteroid {
            health: 1.,
            direction: Vec2::new(-1., 0.5),
            speed: 100.,
        };

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(20., 20.)),
                    ..default()
                },
                transform: Transform::from_xyz(50., 30., 0.),
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
