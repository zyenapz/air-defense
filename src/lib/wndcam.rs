use bevy::prelude::*;

use super::defines::{WND_RES, WND_TITLE};

#[derive(Component)]
pub struct PlayerCamera;

pub fn setup_wndcam(mut windows: ResMut<Windows>, mut commands: Commands) {
    // Setup window
    let window = windows.get_primary_mut().unwrap();

    window.set_resolution(WND_RES.0, WND_RES.1);
    window.set_title(WND_TITLE.to_string());
    window.set_resizable(false);

    // Setup camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}

pub fn player_camera_control(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut OrthographicProjection, With<PlayerCamera>>,
) {
    let dist = 2. * time.delta().as_secs_f32();

    let mut projection = query.single_mut();

    let mut log_scale = projection.scale.ln();

    if kb.pressed(KeyCode::PageUp) {
        log_scale -= dist;
    }
    if kb.pressed(KeyCode::PageDown) {
        log_scale += dist;
    }
    if kb.pressed(KeyCode::F12) {
        // Reset projection
        log_scale = 0.;
        projection.scale = log_scale.exp();
    }

    projection.scale = log_scale.exp();

    // reference: https://www.reddit.com/r/bevy/comments/ojd06s/zoom_with_orthographic_camera/
}
