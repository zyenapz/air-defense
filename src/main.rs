/// Station commander!
/// Defend your space station under attack!
/// TODO:
/// + add a way to shoot
/// + added crosshair
/// + add a way to aim
/// + add cooldown for shooting
/// - add asteroids
///     + add way to spawn them
///     + add update code
///     + randomize spawning off-camera (or perhaps add a spawning box)
///     + randomize speed and direction
///     - add sprites
/// + add zooming (for debugging)
/// + add cordon that encloses the 800x800 world (for debugging, again)
/// - add function for spawning allied fighters
/// - add a way to despawn the bullets when they're out of view
/// + fix z-fighting when spawning asteroids
/// + add test collision

/// Game objectives:
/// > shoot aliens and asteroids
/// > defend the cargo ships (they will deliver powerups, health, etc.
use bevy::{
    input::mouse::MouseMotion, prelude::*, render::camera::RenderTarget,
    sprite::collide_aabb::collide,
};
use lib::{
    asteroid::{spawn_asteroid, update_asteroid},
    debugging::{c_bullet_asteroid, setup_debug_cordon},
    player::{control_player, setup_player, setup_shoot_timer, update_bullet},
    wndcam::{player_camera_control, setup_wndcam},
};
mod lib;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_wndcam)
        .add_startup_system(setup_player)
        .add_startup_system(setup_shoot_timer)
        .add_startup_system(setup_debug_cordon)
        .add_system(control_player)
        .add_system(update_bullet)
        .add_system(spawn_asteroid)
        .add_system(update_asteroid)
        .add_system(c_bullet_asteroid)
        // For debugging only
        .add_system(player_camera_control)
        .run();
}

// fn move_crosshair(
//     // crosshair
//     mut crosshair: Query<(&Crosshair, &mut Transform)>,
//     // need to get window dimensions
//     wnds: Res<Windows>,
//     // query to get camera transform
//     q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
// ) {
//     // get the camera info and transform
//     // assuming there is exactly one main camera entity, so query::single() is OK
//     let (camera, camera_transform) = q_camera.single();

//     // get the window that the camera is displaying to (or the primary window)
//     let wnd = wnds.get_primary().unwrap();

//     // check if the cursor is inside the window and get its position
//     if let Some(screen_pos) = wnd.cursor_position() {
//         // get the size of the window
//         let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

//         // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
//         let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

//         // matrix for undoing the projection and camera transform
//         let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

//         // use it to convert ndc to world-space coordinates
//         let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

//         // reduce it to a 2D value
//         let world_pos: Vec2 = world_pos.truncate();

//         crosshair.single_mut().1.translation.x = world_pos.x;
//         crosshair.single_mut().1.translation.y = world_pos.y;

//         eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
//     }
// }
