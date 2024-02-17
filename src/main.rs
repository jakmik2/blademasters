mod utils;

use std::f32::consts::PI;

//Aabb2d
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};
use utils::logging::*;

const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1280.0;

fn main() {
    console_log!("It's working!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#combative-survivors-canvas".into()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player, move_scythe))
        .add_systems(Update, add_scythe)
        .run();
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    speed: Speed,
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl PlayerBundle {
    fn new() -> Self {
        console_log!("Adding Player!");
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec2::ZERO.extend(0.0),
                    scale: Vec2::new(30.0, 30.0).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            player: Player,
            speed: Speed(150.0),
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    // TODO

    commands.spawn(PlayerBundle::new());
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_transform, player_speed) = query.single_mut();
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }

    let new_player_position = player_transform.translation
        + direction.extend(0.0) * player_speed.0 * time.delta_seconds();

    player_transform.translation = new_player_position;
}

fn add_scythe(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    let player_entity = query.single();

    if keyboard_input.just_pressed(KeyCode::Q) {
        // Add a scythe if q is pressed
        console_log!("Adding a scythe");

        // Spawn scythe
        let new_scythe = commands.spawn(ScytheBundle::new()).id();

        // Insert as child
        commands.entity(player_entity).push_children(&[new_scythe]);
    }
}

#[derive(Component)]
struct Scythe;

#[derive(Bundle)]
struct ScytheBundle {
    collider: Collider,
    sprite_bundle: SpriteBundle,
    scythe: Scythe,
}

impl ScytheBundle {
    fn new() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::ONE * 3.0,
                    scale: Vec2::new(0.5, 0.5).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::ALICE_BLUE,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            scythe: Scythe,
        }
    }
}

const ROT_VEL: f32 = PI;

fn move_scythe(mut query: Query<&mut Transform, (With<Scythe>, Without<Player>)>, time: Res<Time>) {
    for mut scythe_transform in query.iter_mut() {
        // Rotate scythe around parent position ROT_VEL * time passed
        let rot_position = Vec2::from_angle(ROT_VEL * time.delta_seconds())
            .rotate(scythe_transform.translation.truncate());

        let new_pos = rot_position.extend(0.0);

        scythe_transform.translation = new_pos;
    }
}
