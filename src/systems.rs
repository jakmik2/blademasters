use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand_core::RngCore;

use crate::components::{prelude::*, *};
use crate::utils::logging::*;
use crate::{console_log, resources::*, SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    player_data: Res<PlayerData>,
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
                width: Val::Percent(20.0),
                height: Val::Percent(20.0),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!(
                        "Health: {:?}\nScore: {:?}",
                        player_data.health, player_data.score
                    ),
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
                PlayerDisplay,
            ));
        });
}

pub fn update_ui(mut query: Query<&mut Text, With<PlayerDisplay>>, player_data: Res<PlayerData>) {
    let mut display_text = query.single_mut();

    // Udpate displaye
    display_text.sections[0].value = format!(
        "Health: {:?}\nScore: {:?}",
        player_data.health, player_data.score
    );
}

pub fn move_player(
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
        + direction.try_normalize().unwrap_or(Vec2::ZERO).extend(0.0)
            * player_speed.0
            * time.delta_seconds();

    player_transform.translation = new_player_position;
}

const ENEMY_SPEED: f32 = 1.25;

pub fn hunt_player(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<
        (&mut Transform, Entity, Option<&Children>),
        (With<Enemy>, Without<Player>),
    >,
    mut enemy_spawner: ResMut<EnemySpawner>,
    mut player_data: ResMut<PlayerData>,
    scythe_query: Query<&Scythe>,
) {
    let player_transform = player_query.single();

    'enemies: for (mut enemy_transform, enemy, children) in enemy_query.iter_mut() {
        let distance_to_player = enemy_transform
            .translation
            .distance(player_transform.translation);

        if distance_to_player < 30.0 && player_data.health != 0 {
            // Close enough to act
            commands.entity(enemy).despawn_recursive();
            enemy_spawner.num_enemies -= 1;

            // Take Damage
            player_data.health -= 1;
        } else if distance_to_player > 120.0 {
            // Move Towards Player
            let diff_vec = player_transform.translation - enemy_transform.translation;

            let unit_vec = diff_vec.normalize();
            enemy_transform.translation += unit_vec * ENEMY_SPEED;
        } else if children.is_some() {
            for child in children.unwrap() {
                if scythe_query.get(*child).is_ok() {
                    continue 'enemies;
                }
            }
            // Move Towards Player : DON'T love duplicate code
            let diff_vec = player_transform.translation - enemy_transform.translation;

            let unit_vec = diff_vec.normalize();
            enemy_transform.translation += unit_vec * ENEMY_SPEED;
        }
    }
}

pub fn add_scythe(
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
            < 20.0
        {
            console_log!(
                "Treat distance to player: {:?}",
                player_transform
                    .translation
                    .distance(treat_transform.translation)
            );

            // Destroy the treat
            commands.entity(treat).despawn();

            // Insert as child
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn((ScytheBundle::new(), TargetsEnemies));
            });
        }
    }
}

const ROT_VEL: f32 = 3.0 * PI / 2.0;

pub fn move_scythe(
    mut query: Query<&mut Transform, (With<Scythe>, Without<Player>, Without<FlyingAway>)>,
    time: Res<Time>,
) {
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

pub fn handle_player_health(
    to_despawn: Query<Entity, Or<(With<Scythe>, With<Enemy>, With<Treat>)>>,
    mut player_query: Query<
        &mut Transform,
        (
            With<Player>,
            Without<Scythe>,
            Without<Enemy>,
            Without<Treat>,
        ),
    >,
    mut commands: Commands,
    mut player_data: ResMut<PlayerData>,
    mut treat_spawner: ResMut<TreatSpawner>,
    mut enemy_spawner: ResMut<EnemySpawner>,
) {
    let mut player_transform = player_query.single_mut();

    // Restart Game if player dies
    // TODO : Whole lot to do for game reset
    if player_data.health <= 0 {
        // Despawn all entities that aren't player
        for entity in &to_despawn {
            console_log!("Despawning {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }

        // Reset Data
        player_data.health = 3;
        player_data.score = 0;

        treat_spawner.num_treats = 0;
        treat_spawner.counter = 0.0;

        enemy_spawner.num_enemies = 0;
        enemy_spawner.num_enemies_killed = 0;
        enemy_spawner.counter = 0.0;

        // Reset Player Position
        player_transform.translation = Vec3::ZERO;
    }
}

const FIXED_ENEMY_SPAWN: f32 = 5.0;

pub fn enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_spawner: ResMut<EnemySpawner>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    // Update Timer
    enemy_spawner.counter += time.delta_seconds();

    // Check timer // TODO : Limit total number of enemies spawned at a time
    if enemy_spawner.counter > FIXED_ENEMY_SPAWN {
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

// TODO: This needs to be pivotted to be signals based
pub fn handle_ally_scythes(
    mut commands: Commands,
    ally_scythe_query: Query<
        (&GlobalTransform, Entity),
        (With<Scythe>, With<TargetsEnemies>, Without<FlyingAway>),
    >,
    enemy_query: Query<(&GlobalTransform, Entity), With<Enemy>>,
    enemy_scythe_query: Query<&Children, (With<Enemy>, Without<FlyingAway>)>,
    mut enemy_spawner: ResMut<EnemySpawner>,
    mut treat_spawner: ResMut<TreatSpawner>,
    mut player_data: ResMut<PlayerData>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    for (enemy_transform, enemy) in enemy_query.iter() {
        // Check if something has already hit this enemy
        let mut collided = false;

        for (scythe_transform, scythe_entity) in ally_scythe_query.iter() {
            if !collided
                && scythe_transform
                    .translation()
                    .distance(enemy_transform.translation())
                    < 30.0
            {
                collided = true;
                console_log!("Collision: {:?}", enemy);

                // Destroy enemy
                commands.entity(enemy).despawn_recursive();
                enemy_spawner.num_enemies -= 1;

                // Increment Score
                player_data.score += 1;

                // Handle scythe
                commands.entity(scythe_entity).despawn();

                // Spawn treat for each scythe
                let Ok(children) = enemy_scythe_query.get(enemy) else {
                    return;
                };

                for _ in children {
                    treat_spawner.num_treats += 1;
                    // Random Vec distance from death
                    let random_offset = Vec3::new(
                        rng.next_u32() as f32 % 30.0,
                        rng.next_u32() as f32 % 30.0,
                        0.0,
                    );
                    commands.spawn(TreatBundle::new_at(
                        enemy_transform.translation() + random_offset,
                    ));
                }
            }
        }
    }
}

pub fn handle_enemy_scythes(
    mut commands: Commands,
    enemy_scythe_query: Query<
        (&GlobalTransform, Entity),
        (With<Scythe>, With<TargetsPlayer>, Without<FlyingAway>),
    >,
    player_query: Query<&GlobalTransform, (With<Player>, Without<Scythe>)>,
    mut player_data: ResMut<PlayerData>,
) {
    let player_transform = player_query.single();

    for (scythe_transform, scythe_entity) in enemy_scythe_query.iter() {
        if scythe_transform
            .translation()
            .distance(player_transform.translation())
            < 30.0
        {
            console_log!("Player taking damage");

            // Decrement Player Health
            player_data.health -= 1;

            // Handle Scythe
            commands.entity(scythe_entity).despawn();
        }
    }
}

pub fn handle_scythe_collision(
    mut commands: Commands,
    mut ally_scythe_query: Query<
        (&GlobalTransform, &Transform, &mut Scythe, Entity),
        (
            With<Scythe>,
            With<TargetsEnemies>,
            Without<TargetsPlayer>,
            Without<FlyingAway>,
        ),
    >,
    mut enemy_scythe_query: Query<
        (&GlobalTransform, &Transform, &mut Scythe, Entity),
        (
            With<Scythe>,
            With<TargetsPlayer>,
            Without<TargetsEnemies>,
            Without<FlyingAway>,
        ),
    >,
) {
    for (a_scythe_gt, a_scythe_t, mut ally_scythe_str, ally_scythe) in ally_scythe_query.iter_mut()
    {
        let mut collided = false;

        for (e_scythe_gt, e_scythe_t, mut enemy_scythe_str, enemy_scythe) in
            enemy_scythe_query.iter_mut()
        {
            if !collided
                && a_scythe_gt
                    .translation()
                    .distance(e_scythe_gt.translation())
                    < 40.0
            {
                console_log!("We getting here?");
                // Don't want repeat collisions
                collided = true;

                // See which scythe loses
                // Evaluate flying away to loser, decrement strength of winner
                if ally_scythe_str.0 >= enemy_scythe_str.0 {
                    commands.entity(enemy_scythe).insert(FlyingAway::new(
                        Vec2::new(e_scythe_t.translation.y, -e_scythe_t.translation.x).extend(0.0),
                    ));
                    ally_scythe_str.0 -= 1;
                } else {
                    commands.entity(ally_scythe).insert(FlyingAway::new(
                        Vec2::new(a_scythe_t.translation.y, -a_scythe_t.translation.x).extend(0.0),
                    ));
                    enemy_scythe_str.0 -= 1;
                }
            }
        }
    }
}

const FLY_AWAY_TIMEOUT: f32 = 0.25;
const FLY_AWAY_SPEED: f32 = 10.0;

pub fn handle_flying_away(
    mut commands: Commands,
    mut flying_away_scythe: Query<
        (&mut Transform, &mut FlyingAway, Entity),
        (With<Scythe>, With<FlyingAway>),
    >,
    time: Res<Time>,
) {
    // Fly away til time out
    for (mut flying_scythe_t, mut fly_tracker, scythe) in flying_away_scythe.iter_mut() {
        fly_tracker.counter += time.delta_seconds();

        if fly_tracker.counter >= FLY_AWAY_TIMEOUT {
            // Destroy scythe
            commands.entity(scythe).despawn_recursive();
        } else {
            // Move the homie
            flying_scythe_t.translation +=
                fly_tracker.trajectory * FLY_AWAY_SPEED * time.delta_seconds();
        }
    }
}

const FIXED_TREAT_SPAWN: f32 = 7.0;

pub fn treat_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut treat_spawner: ResMut<TreatSpawner>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    // Update Timer
    treat_spawner.counter += time.delta_seconds();

    // Check timer
    if treat_spawner.counter > FIXED_TREAT_SPAWN {
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
