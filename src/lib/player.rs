use std::time::Duration;

use bevy::prelude::*;

use crate::lib::defines::PB_INIT_SPEED;

use super::defines::{P_INIT_ARMOR, P_INIT_FIRE_COOLDOWN, P_INIT_HEALTH};

#[derive(Component)]
pub struct Station {
    health: f32,
    armor: f32,
}

#[derive(Component)]
pub struct Gun {
    rotation: f32,
}

#[derive(Component)]
pub struct Bullet {
    speed: f32,
    direction: Vec2,
}

#[derive(Component)]
pub struct FireCooldown {
    timer: Timer,
}

pub fn setup_player(mut commands: Commands) {
    let station = Station {
        health: P_INIT_HEALTH,
        armor: P_INIT_ARMOR,
    };
    let gun = Gun { rotation: 0. };

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
        .insert(station)
        .insert(gun);
}

pub fn setup_shoot_timer(mut commands: Commands) {
    commands.insert_resource(FireCooldown {
        timer: Timer::new(Duration::from_millis(P_INIT_FIRE_COOLDOWN), true),
    })
}

pub fn control_player(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut cooldown: ResMut<FireCooldown>,
    mut commands: Commands,
    mut query: Query<(&Station, &mut Gun)>,
) {
    // Update timer
    cooldown.timer.tick(time.delta());

    match query.get_single_mut() {
        Ok(mut player) => {
            // --- Rotate gun ---
            if keyboard.pressed(KeyCode::Q) {
                player.1.rotation -= 1.;
            } else if keyboard.pressed(KeyCode::E) {
                player.1.rotation += 1.;
            }

            // --- Fire bullet ---
            if keyboard.pressed(KeyCode::Z) && cooldown.timer.finished() {
                let angle = player.1.rotation.to_radians();
                let dx = angle.sin();
                let dy = angle.cos();

                let bullet = Bullet {
                    speed: PB_INIT_SPEED,
                    direction: Vec2::new(dx, dy),
                };

                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::YELLOW,
                            custom_size: Some(Vec2::new(10., 10.)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0., 0., 1.),
                        ..default()
                    })
                    .insert(bullet);
            }
        }
        Err(e) => panic!("{}", e),
    }
}

pub fn update_bullet(time: Res<Time>, mut query: Query<(&Bullet, &mut Transform)>) {
    for (b, mut t) in query.iter_mut() {
        t.translation.x += b.direction.x * b.speed * time.delta_seconds();
        t.translation.y += b.direction.y * b.speed * time.delta_seconds();
    }
}
