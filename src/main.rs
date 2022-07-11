use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
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
            transform: Transform::from_xyz(100., 100., 0.),
            ..default()
        })
        .insert(Player);
}

const PLAYER_SPEED: f32 = 350.;
fn move_player(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let mut transform = query.single_mut().1;

    let mut dx = 0.;
    let mut dy = 0.;

    if kb.pressed(KeyCode::W) {
        dy = PLAYER_SPEED * time.delta_seconds();
    }
    if kb.pressed(KeyCode::S) {
        dy = PLAYER_SPEED * -time.delta_seconds();
    }
    if kb.pressed(KeyCode::A) {
        dx = PLAYER_SPEED * -time.delta_seconds();
    }
    if kb.pressed(KeyCode::D) {
        dx = PLAYER_SPEED * time.delta_seconds();
    }

    transform.translation.x += dx;
    transform.translation.y += dy;
}
