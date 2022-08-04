use std::time::Duration;

use bevy::prelude::*;

use crate::lib::{
    bullet::{BulletBundle, PlayerBullet},
    defines::PB_INIT_SPEED,
    shared::{Speed, ZnDirection},
};

use super::{
    defines::{P_INIT_ARMOR, P_INIT_FIRE_COOLDOWN, P_INIT_HEALTH},
    shared::{Armor, Health},
    wndcam::PlayerCamera,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    health: Health,
    armor: Armor,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FireTimer(Timer);

#[derive(Component)]
pub struct FireCooldown {
    timer: Timer,
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
        .insert_bundle(PlayerBundle {
            health: Health(P_INIT_HEALTH),
            armor: Armor(P_INIT_ARMOR),
        })
        .insert(Player);
}

pub fn setup_shoot_timer(mut commands: Commands) {
    commands.insert_resource(FireCooldown {
        timer: Timer::new(Duration::from_millis(P_INIT_FIRE_COOLDOWN), true),
    })
}

pub fn control_player_mouse(
    time: Res<Time>,
    mut cooldown: ResMut<FireCooldown>,
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    wnds: Res<Windows>,
    m_btn: Res<Input<MouseButton>>,
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

    // --- Fire bullet ---
    if m_btn.pressed(MouseButton::Left) && cooldown.timer.finished() {
        let rel_y = mouse_pos.y - player.translation.y;
        let rel_x = mouse_pos.x - player.translation.x;

        let angle = rel_y.atan2(rel_x);
        let dx = angle.cos();
        let dy = angle.sin();

        println!("{} {}", dx, dy);

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
            .insert_bundle(BulletBundle {
                speed: Speed(PB_INIT_SPEED),
                direction: ZnDirection(Vec2::new(dx, dy)),
            })
            .insert(PlayerBullet);
    }
}
