use std::time::Duration;

use bevy::prelude::*;

use crate::lib::defines::PB_INIT_SPEED;

use super::{
    defines::{P_INIT_ARMOR, P_INIT_FIRE_COOLDOWN, P_INIT_HEALTH},
    wndcam::PlayerCamera,
};

#[derive(Component)]
pub struct Player {
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
    let station = Player {
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
    mut query: Query<(&Player, &mut Gun)>,
) {
    // Update timer
    cooldown.timer.tick(time.delta());

    match query.get_single_mut() {
        Ok(mut player) => {
            // --- Rotate gun ---
            if keyboard.pressed(KeyCode::Q) {
                player.1.rotation -= 2.;
            } else if keyboard.pressed(KeyCode::E) {
                player.1.rotation += 2.;
            }

            // --- Fire bullet ---
            if (keyboard.pressed(KeyCode::Z) || keyboard.pressed(KeyCode::Space))
                && cooldown.timer.finished()
            {
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

pub fn control_player_mouse(
    time: Res<Time>,
    mut cooldown: ResMut<FireCooldown>,
    mut commands: Commands,
    q_player: Query<(&Player, &mut Gun, &Transform)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    wnds: Res<Windows>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = wnds.get_primary().unwrap();
    let mut mouse_pos = Vec2::new(0., 0.);

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        mouse_pos = world_pos.truncate();
    }

    let player = q_player.single();
    cooldown.timer.tick(time.delta());

    // eprintln!(
    //     "{} {} {} {}",
    //     mouse_pos.x, mouse_pos.y, player.2.translation.x, player.2.translation.y
    // );

    // --- Fire bullet ---
    if cooldown.timer.finished() {
        let rel_y = mouse_pos.y - player.2.translation.y;
        let rel_x = mouse_pos.x - player.2.translation.x;

        let angle = rel_y.atan2(rel_x);
        let dx = angle.cos();
        let dy = angle.sin();

        println!("{} {}", dx, dy);

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
