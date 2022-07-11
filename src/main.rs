use std::time::Duration;

/// Station commander!
/// Defend your space station under attack!
/// TODO:
/// + add a way to shoot
/// - add a way to aim
/// + add cooldown for shooting
/// - add function for spawning fighters
/// - add simple enemies
/// - add a way to despawn the bullets when they're out of view
use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bullet {
    direction: Vec2,
}

#[derive(Component)]
struct ShootCDTimer {
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_shoot_timer)
        .add_system(move_bullet)
        .add_system(shoot)
        .add_system(upgrade_timer)
        .insert_resource(ClearColor(Color::rgb(0., 0.4, 0.4)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 0., 1.),
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        transform: Transform::from_xyz(200., 100., 0.),
        ..default()
    });
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(Player);
}

fn shoot(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut config: ResMut<ShootCDTimer>,
) {
    config.timer.tick(time.delta());

    if kb.pressed(KeyCode::Space) && config.timer.finished() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 0., 0.),
                    custom_size: Some(Vec2::new(10., 10.)),
                    ..default()
                },
                transform: Transform::from_xyz(100., 0., 0.),
                ..default()
            })
            .insert(Bullet {
                direction: Vec2::new(0., 1.),
            });

        config.timer.reset();
    }
}

fn upgrade_timer(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut ShootCDTimer>,
    mut commands: Commands,
) {
    if kb.pressed(KeyCode::Q) {
        commands.insert_resource(ShootCDTimer {
            timer: Timer::new(Duration::from_millis(500), true),
        })
    } else if kb.pressed(KeyCode::W) {
        commands.insert_resource(ShootCDTimer {
            timer: Timer::new(Duration::from_millis(100), true),
        })
    }
}

fn setup_shoot_timer(mut commands: Commands) {
    commands.insert_resource(ShootCDTimer {
        timer: Timer::new(Duration::from_millis(800), true),
    })
}

const BULLET_SPEED: f32 = 550.;
fn move_bullet(time: Res<Time>, mut query: Query<(&Bullet, &mut Transform)>) {
    for (b, mut tr) in query.iter_mut() {
        let mut direction = b.direction;

        let mut dx = BULLET_SPEED * time.delta_seconds();
        let mut dy = BULLET_SPEED * time.delta_seconds();

        tr.translation.x += dx;
        tr.translation.y += dy;
    }
}
