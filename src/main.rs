mod utils;

use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

use bevy_rand::prelude::*;
use rand_core::RngCore;

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
        .insert_resource(EnemySpawner {
            counter: 0.0,
            num_enemies: 0,
            num_enemies_killed: 0,
        })
        .insert_resource(TreatSpawner {
            counter: 200.0,
            num_treats: 0,
        })
        .insert_resource(PlayerData {
            score: 0,
            health: 10,
        })
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player, move_scythe, hunt_player))
        .add_systems(
            Update,
            (
                add_scythe,
                enemy_spawn,
                treat_spawn,
                handle_scythe_collision,
            ),
        )
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

    // Add UI Display
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(10.0),
                height: Val::Percent(10.0),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "A TEST",
                    TextStyle {
                        font: asset_server.load("fonts/FFGhost-Regular.ttf"),
                        font_size: 30.0,
                        ..Default::default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..Default::default()
                }),
                Label,
            ));
        });
}

#[derive(Resource)]
struct PlayerData {
    score: usize,
    health: usize,
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
    query: Query<(&Transform, Entity), (With<Player>, Without<Treat>)>,
    treat_query: Query<(&Transform, Entity), (With<Treat>, Without<Player>)>,
    mut commands: Commands,
) {
    let (player_transform, player_entity) = query.single();

    // TODO : This is horribly optimized
    for (treat_transform, treat) in treat_query.iter() {
        if player_transform
            .translation
            .distance(treat_transform.translation)
            < 10.0
        {
            console_log!(
                "Treat distance to player: {:?}",
                player_transform
                    .translation
                    .distance(treat_transform.translation)
            );

            // Destroy the treat
            commands.entity(treat).despawn();

            // Add a scythe if q is pressed
            console_log!("Adding a scythe");

            // Spawn scythe
            let new_scythe = commands.spawn(ScytheBundle::new()).id();

            // Insert as child
            commands.entity(player_entity).push_children(&[new_scythe]);
        }
    }
}

#[derive(Component)]
struct Scythe(u8);

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
            scythe: Scythe(2),
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

        // Rotate Around Parent
        scythe_transform.translation = new_pos;
        // Rotate actual scythe
        scythe_transform.rotate(Quat::from_rotation_z(ROT_VEL * time.delta_seconds()));
    }
}

// TODO: This needs to be pivotted to be signals based
fn handle_scythe_collision(
    mut commands: Commands,
    scythe_query: Query<(&GlobalTransform, Entity), (With<Scythe>, Without<Enemy>)>,
    enemy_query: Query<(&GlobalTransform, Entity), (With<Enemy>, Without<Scythe>)>,
    mut enemy_spawner: ResMut<EnemySpawner>,
) {
    for (scythe_transform, scythe_entity) in scythe_query.iter() {
        for (enemy_transform, enemy) in enemy_query.iter() {
            if scythe_transform
                .translation()
                .distance(enemy_transform.translation())
                < 30.0
            {
                console_log!("Collision: {:?}", enemy);
                // Decrement Scythe Durability
                // scythe.0 -= 1;

                // Destroy enemy
                commands.entity(enemy).despawn();
                enemy_spawner.num_enemies -= 1;

                // Handle scythe
                // if scythe.0 <= 0 {
                commands.entity(scythe_entity).despawn();
                // }
            }
        }
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    collider: Collider,
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
}

impl EnemyBundle {
    fn new() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::ZERO,
                    scale: Vec2::new(1.0, 1.0).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::BISQUE,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            enemy: Enemy,
        }
    }

    fn new_at(position: Vec2) -> Self {
        console_log!("Spawning Enemy!");
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec2::new(30.0, 30.0).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::BISQUE,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            enemy: Enemy,
        }
    }
}

const ENEMY_SPEED: f32 = 2.0;

fn hunt_player(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, Entity), (With<Enemy>, Without<Player>)>,
    mut enemy_spawner: ResMut<EnemySpawner>,
) {
    let player_transform = player_query.single();

    for (mut enemy_transform, enemy) in enemy_query.iter_mut() {
        if enemy_transform
            .translation
            .distance(player_transform.translation)
            < 30.0
        {
            // Close enough to act
            commands.entity(enemy).despawn();
            enemy_spawner.num_enemies -= 1;
        } else {
            // Move Towards Player
            let diff_vec = player_transform.translation - enemy_transform.translation;

            let unit_vec = diff_vec.normalize();
            enemy_transform.translation += unit_vec * ENEMY_SPEED;
        }
    }
}

#[derive(Resource)]
struct EnemySpawner {
    num_enemies: usize,
    num_enemies_killed: usize,
    counter: f32,
}

const FIXED_ENEMY_SPAWN: f32 = 5.0;

fn enemy_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_spawner: ResMut<EnemySpawner>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    // Update Timer
    enemy_spawner.counter += time.delta_seconds();

    // Check timer
    if enemy_spawner.counter > FIXED_ENEMY_SPAWN && enemy_spawner.num_enemies < 10 {
        enemy_spawner.num_enemies += 1;
        enemy_spawner.counter = 0.0;

        // Construct random location
        let pos = Vec2::new(
            (rng.next_u32() as f32 % SCREEN_WIDTH) - SCREEN_WIDTH / 2.0,
            rng.next_u32() as f32 % SCREEN_HEIGHT - SCREEN_HEIGHT / 2.0,
        );

        // Spawn an enemy in a random place!
        commands.spawn(EnemyBundle::new_at(pos));
    }
}

#[derive(Resource)]
struct TreatSpawner {
    num_treats: usize,
    counter: f32,
}

const FIXED_TREAT_SPAWN: f32 = 7.0;

fn treat_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut treat_spawner: ResMut<TreatSpawner>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    // Update Timer
    treat_spawner.counter += time.delta_seconds();

    // Check timer
    if treat_spawner.counter > FIXED_TREAT_SPAWN && treat_spawner.num_treats < 5 {
        treat_spawner.num_treats += 1;
        treat_spawner.counter = 0.0;

        // Construct random location
        let pos = Vec2::new(
            (rng.next_u32() as f32 % SCREEN_WIDTH) - SCREEN_WIDTH / 2.0,
            rng.next_u32() as f32 % SCREEN_HEIGHT - SCREEN_HEIGHT / 2.0,
        );
        console_log!("Spawning Treat! {:?}", pos);

        // Spawn a Treat in a random place!
        commands.spawn(TreatBundle::new_at(pos.extend(0.0)));
    }
}

#[derive(Component)]
struct Treat;

#[derive(Bundle)]
struct TreatBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    treat: Treat,
}

impl TreatBundle {
    fn new_at(position: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position,
                    scale: Vec2::new(10.0, 10.0).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::FUCHSIA,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            treat: Treat,
        }
    }
}
