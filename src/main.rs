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
/// + add a way to despawn the bullets when they're out of view
/// + add a check in the asteroid-bullet collision that tests if the bullet is out of the map first before executing collision logic
/// + fix z-fighting when spawning asteroids
/// + add test collision

/// Game objectives:
/// > shoot aliens and asteroids
/// > defend the cargo ships (they will deliver powerups, health, etc.
use bevy::prelude::*;
use lib::{
    asteroid::{spawn_asteroid, update_asteroid},
    bullet::update_bullet,
    debugging::{c_bullet_asteroid, setup_debug_cordon},
    player::{control_player_mouse, setup_player, setup_shoot_timer},
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
        .add_system(control_player_mouse)
        .add_system(update_bullet)
        .add_system(spawn_asteroid)
        .add_system(update_asteroid)
        .add_system(c_bullet_asteroid)
        // For debugging only
        .add_system(player_camera_control)
        .run();
}
