/// Station commander!
/// Defend your space station under attack!
/// TODO:
/// + add a way to shoot
/// + added crosshair
/// + add a way to aim
/// + add cooldown for shooting
/// - add function for spawning allied fighters
/// - add simple enemies
/// - add a way to despawn the bullets when they're out of view

/// Game objectives:
/// - shoot aliens and asteroids
/// - defend the cargo ships (they will deliver powerups, health, etc.
use bevy::{input::mouse::MouseMotion, prelude::*, render::camera::RenderTarget};
use lib::{player::setup_player, wndcam::setup_wndcam};
mod lib;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_wndcam)
        .add_startup_system(setup_player)
        .run();
}

// #[derive(Component)]
// struct Player;

// #[derive(Component)]
// struct Bullet {
//     direction: Vec2,
// }

// #[derive(Component)]
// struct ShootCDTimer {
//     timer: Timer,
// }

// #[derive(Component)]
// struct Crosshair;

// #[derive(Component)]
// struct MainCamera;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup)
//         .add_startup_system(setup_shoot_timer)
//         .add_startup_system(setup_window)
//         .add_system(move_bullet)
//         .add_system(shoot)
//         .add_system(upgrade_weapon)
//         .add_system(move_crosshair)
//         .insert_resource(ClearColor(Color::rgb(0., 0.4, 0.4)))
//         .run();
// }

// fn setup_window(mut windows: ResMut<Windows>) {
//     let window = windows.get_primary_mut().unwrap();

//     window.set_resolution(500., 750.);
//     window.set_title("Station Commander".to_string());
//     window.set_resizable(false);
// }

// fn setup(windows: Res<Windows>, mut commands: Commands) {
//     // Camera
//     commands
//         .spawn()
//         .insert_bundle(OrthographicCameraBundle::new_2d())
//         .insert(MainCamera);

//     // Station (Player)

//     let win_h = -(windows.get_primary().unwrap().height() / 2.);
//     println!("{}", win_h);

//     commands
//         .spawn_bundle(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(1., 1., 1.),
//                 custom_size: Some(Vec2::new(48., 48.)),
//                 ..default()
//             },
//             transform: Transform::from_xyz(0., win_h + 100., 0.),
//             ..default()
//         })
//         .insert(Player);

//     // Crosshair
//     commands
//         .spawn_bundle(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::RED,
//                 custom_size: Some(Vec2::new(10., 10.)),
//                 ..default()
//             },
//             transform: Transform::from_xyz(0., 0., 0.),
//             ..default()
//         })
//         .insert(Crosshair);
// }

// fn shoot(
//     mut commands: Commands,
//     kb: Res<Input<KeyCode>>,
//     time: Res<Time>,
//     mut config: ResMut<ShootCDTimer>,
//     player: Query<(&Player, &Transform)>,
//     crosshair: Query<(&Crosshair, &Transform)>,
// ) {
//     config.timer.tick(time.delta());

//     if kb.pressed(KeyCode::Space) && config.timer.finished() {
//         let player_pos = player.single().1;
//         let ch_pos = crosshair.single().1;

//         // calculate dx and dy
//         let rel_y = ch_pos.translation.y - player_pos.translation.y;
//         let rel_x = ch_pos.translation.x - player_pos.translation.x;
//         let angle = rel_x.atan2(rel_y);

//         let dx = angle.sin();
//         let dy = angle.cos();

//         println!("x{:?} y{:?}", dx, dy);

//         commands
//             .spawn_bundle(SpriteBundle {
//                 sprite: Sprite {
//                     color: Color::rgb(1., 0., 0.),
//                     custom_size: Some(Vec2::new(10., 10.)),
//                     ..default()
//                 },
//                 transform: Transform::from_xyz(
//                     player_pos.translation.x,
//                     player_pos.translation.y,
//                     0.,
//                 ),
//                 ..default()
//             })
//             .insert(Bullet {
//                 direction: Vec2::new(dx, dy),
//             });

//         config.timer.reset();
//     }
// }

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

// fn upgrade_weapon(kb: Res<Input<KeyCode>>, mut commands: Commands) {
//     if kb.pressed(KeyCode::F1) {
//         commands.insert_resource(ShootCDTimer {
//             timer: Timer::new(Duration::from_millis(500), true),
//         })
//     } else if kb.pressed(KeyCode::F2) {
//         commands.insert_resource(ShootCDTimer {
//             timer: Timer::new(Duration::from_millis(100), true),
//         })
//     }
// }

// fn setup_shoot_timer(mut commands: Commands) {
//     commands.insert_resource(ShootCDTimer {
//         timer: Timer::new(Duration::from_millis(800), true),
//     })
// }

// const BULLET_SPEED: f32 = 550.;
// fn move_bullet(time: Res<Time>, mut query: Query<(&Bullet, &mut Transform)>) {
//     for (b, mut tr) in query.iter_mut() {
//         let direction = b.direction;

//         let dx = direction.x * BULLET_SPEED * time.delta_seconds();
//         let dy = direction.y * BULLET_SPEED * time.delta_seconds();

//         tr.translation.x += dx;
//         tr.translation.y += dy;
//     }
// }
