use bevy::prelude::*;

use super::defines::{WND_RES, WND_TITLE};

pub fn setup_wndcam(mut windows: ResMut<Windows>, mut commands: Commands) {
    // Setup window
    let window = windows.get_primary_mut().unwrap();

    window.set_resolution(WND_RES.0, WND_RES.1);
    window.set_title(WND_TITLE.to_string());
    window.set_resizable(false);

    // Setup camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}
