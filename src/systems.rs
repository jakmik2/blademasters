use std::f32::consts::PI;

use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

use crate::components::{prelude::*, *};
use crate::utils::logging::*;
use crate::{console_log, resources::*, GameState, SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Game);

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    // TODO

    // Add player
    // TODO : Change to scene based start
    commands.spawn(PlayerBundle::new());
}

pub fn update_ui(
    mut query: Query<&mut Text, With<PlayerDisplay>>,
    player_data_query: Query<(&Xp, &Health), With<Player>>,
    score: Res<Score>,
) {
    let mut display_text = query.single_mut();
    let Ok((xp, health)) = player_data_query.get_single() else {
        return;
    };

    // Udpate displaye
    display_text.sections[0].value = format!(
        "Health: {:?}\nScore: {:?}\nXp: {:?}",
        health.0, score.0, xp.0
    );
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_transform, player_speed) = query.single_mut();
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    let new_player_position = player_transform.translation
        + direction.try_normalize().unwrap_or(Vec2::ZERO).extend(0.0)
            * player_speed.0
            * time.delta_seconds();

    player_transform.translation = new_player_position;
}

const ENEMY_SPEED: f32 = 45.0;

pub fn hunt_player(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<
        (&mut Transform, Entity, Option<&Children>),
        (With<Enemy>, Without<Player>),
    >,
    mut enemy_spawner: ResMut<EnemySpawner>,
    scythe_query: Query<&Scythe>,
    time: Res<Time>,
) {
    let Ok((player_transform, mut health)) = player_query.get_single_mut() else {
        return;
    };

    'enemies: for (mut enemy_transform, enemy, children) in enemy_query.iter_mut() {
        let distance_to_player = enemy_transform
            .translation
            .distance(player_transform.translation);

        if distance_to_player < 30.0 && health.0 != 0 {
            // Close enough to act
            commands.entity(enemy).despawn_recursive();
            enemy_spawner.num_enemies -= 1;

            // Take Damage
            health.0 -= 1;
        } else if distance_to_player > 120.0 {
            // Move Towards Player
            let diff_vec = player_transform.translation - enemy_transform.translation;

            let unit_vec = diff_vec.normalize();
            enemy_transform.translation += unit_vec * ENEMY_SPEED * time.delta_seconds();
        } else if children.is_some() {
            for child in children.unwrap() {
                if scythe_query.get(*child).is_ok() {
                    continue 'enemies;
                }
            }
            // Move Towards Player : DON'T love duplicate code
            let diff_vec = player_transform.translation - enemy_transform.translation;

            let unit_vec = diff_vec.normalize();
            enemy_transform.translation += unit_vec * ENEMY_SPEED * time.delta_seconds();
        }
    }
}

const TREAT_VEL: f32 = 500.0;

pub fn add_scythe(
    query: Query<(&Transform, Entity), (With<Player>, Without<Treat>)>,
    mut treat_query: Query<(&mut Transform, Entity), (With<Treat>, Without<Player>)>,
    mut commands: Commands,
    skill_tracker: Res<SkillTracker>,
    time: Res<Time>,
) {
    let Ok((player_transform, player_entity)) = query.get_single() else {
        return;
    };

    // TODO : This is horribly optimized
    for (mut treat_transform, treat) in treat_query.iter_mut() {
        let dist_to_player = player_transform
            .translation
            .distance(treat_transform.translation);

        if dist_to_player < 15.0 {
            // Destroy the treat
            commands.entity(treat).despawn_recursive();

            // Insert as child
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn((
                    ScytheBundle::new_with_speed(skill_tracker.get(LevelOptions::ScytheSpeed)),
                    TargetsEnemies,
                ));
            });
        } else if dist_to_player < skill_tracker.get(LevelOptions::TreatRadius) {
            // Move towards player
            // Move Towards Player
            let diff_vec = treat_transform.translation - player_transform.translation;

            let unit_vec = diff_vec.normalize();
            treat_transform.translation -= unit_vec
                * TREAT_VEL
                * (1.0 / dist_to_player * dist_to_player)
                * time.delta_seconds();
        }
    }
}

pub fn move_scythe(
    mut query: Query<
        (&mut Transform, &ScytheSpeed),
        (With<Scythe>, Without<Player>, Without<FlyingAway>),
    >,
    time: Res<Time>,
) {
    for (mut scythe_transform, scythe_speed) in query.iter_mut() {
        // Rotate scythe around parent position ROT_VEL * time passed
        let rot_position = Vec2::from_angle(scythe_speed.0 * time.delta_seconds())
            .rotate(scythe_transform.translation.truncate());

        let new_pos = rot_position.extend(0.0);

        // Rotate Around Parent
        scythe_transform.translation = new_pos;
        // Rotate actual scythe
        scythe_transform.rotate(Quat::from_rotation_z(scythe_speed.0 * time.delta_seconds()));
    }
}

pub fn handle_player_health(
    to_despawn: Query<Entity, Or<(With<Scythe>, With<Enemy>)>>,
    mut player_query: Query<
        (&mut Transform, &mut Xp, &mut Health),
        (
            With<Player>,
            Without<Scythe>,
            Without<Enemy>,
            Without<Treat>,
        ),
    >,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut treat_spawner: ResMut<TreatSpawner>,
    mut enemy_spawner: ResMut<EnemySpawner>,
) {
    let Ok((mut player_transform, mut xp, mut health)) = player_query.get_single_mut() else {
        return;
    };

    // Restart Game if player dies
    // TODO : Whole lot to do for game reset
    if health.0 <= 0 {
        // Despawn all entities that aren't player
        for entity in &to_despawn {
            console_log!("Despawning {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }

        // Reset Data
        xp.0 = 0;
        health.0 = 10;
        score.0 = 0;

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
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    // Update Timer
    enemy_spawner.counter += time.delta_seconds();

    // Check timer // TODO : Limit total number of enemies spawned at a time
    if enemy_spawner.counter > FIXED_ENEMY_SPAWN {
        enemy_spawner.num_enemies += 1;
        enemy_spawner.counter = 0.0;

        // Construct random location
        let mut pos = Vec2::new(
            (rng.next_u32() as f32 % SCREEN_WIDTH) - SCREEN_WIDTH / 2.0,
            rng.next_u32() as f32 % SCREEN_HEIGHT - SCREEN_HEIGHT / 2.0,
        );

        // Only an edge
        pos = match rng.next_u32() % 4 {
            0 => Vec2::new(pos.x, SCREEN_WIDTH / 2.0),
            1 => Vec2::new(pos.x, -SCREEN_WIDTH / 2.0),
            2 => Vec2::new(pos.x, SCREEN_HEIGHT / 2.0),
            3 => Vec2::new(pos.x, -SCREEN_HEIGHT / 2.0),
            _ => Vec2::ZERO, // SHOULD BE UNREACHABLE
        };

        // Spawn an enemy in a random place!
        commands.spawn(EnemyBundle::new_at(pos));
    }
}

// TODO: This needs to be pivotted to be signals based
pub fn handle_ally_scythes(
    mut commands: Commands,
    scythes_query: Query<
        &Scythe,
        (
            With<TargetsPlayer>,
            Without<TargetsEnemies>,
            Without<FlyingAway>,
        ),
    >,
    mut ally_scythe_query: Query<
        (&GlobalTransform, &HitBox, &mut Scythe, Entity),
        (
            With<Scythe>,
            With<TargetsEnemies>,
            Without<TargetsPlayer>,
            Without<FlyingAway>,
        ),
    >,
    enemy_query: Query<(&GlobalTransform, &HitBox, Entity), With<Enemy>>,
    enemy_scythe_query: Query<&Children, With<Enemy>>,
    mut player_query: Query<&mut Xp, With<Player>>,
    mut enemy_spawner: ResMut<EnemySpawner>,
    mut treat_spawner: ResMut<TreatSpawner>,
    mut score: ResMut<Score>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let Ok(mut xp) = player_query.get_single_mut() else {
        return;
    };

    for (enemy_transform, enemy_hb, enemy) in enemy_query.iter() {
        // Check if something has already hit this enemy
        let mut collided = false;

        for (scythe_transform, scythe_hb, mut scythe, scythe_entity) in ally_scythe_query.iter_mut()
        {
            if !collided
                && scythe_hb.intersects(
                    scythe_transform.translation(),
                    enemy_hb,
                    enemy_transform.translation(),
                )
            {
                collided = true;
                console_log!("Collision: {:?}", enemy);

                // Destroy enemy
                commands.entity(enemy).despawn_recursive();
                enemy_spawner.num_enemies -= 1;

                // Increment Score
                score.0 += 1;
                xp.0 += 1;

                // Handle scythe, reduce str
                scythe.0 -= 1;
                if scythe.0 <= 0 {
                    commands.entity(scythe_entity).insert(FlyingAway::new(
                        Vec2::new(
                            scythe_transform.translation().y,
                            -scythe_transform.translation().x,
                        )
                        .normalize()
                        .extend(0.0),
                    ));
                }

                // Spawn treat for each scythe
                let Ok(children) = enemy_scythe_query.get(enemy) else {
                    return;
                };

                'spawn_scythe_treats: for child in children {
                    // 50 50 chance of spawning one of the children
                    if rng.next_u32() % 2 == 0 {
                        continue 'spawn_scythe_treats;
                    }

                    let despawning_scythe = scythes_query.get(*child).unwrap_or(&Scythe(1));

                    treat_spawner.num_treats += 1;
                    // Random Vec distance from death
                    let random_offset = Vec3::new(
                        rng.next_u32() as f32 % 30.0,
                        rng.next_u32() as f32 % 30.0,
                        0.0,
                    );
                    commands.spawn(TreatBundle::new_at(
                        enemy_transform.translation() + random_offset,
                        despawning_scythe.0,
                    ));
                }
            }
        }
    }
}

pub fn handle_enemy_scythes(
    mut commands: Commands,
    enemy_scythe_query: Query<
        (&GlobalTransform, &HitBox, Entity),
        (With<Scythe>, With<TargetsPlayer>, Without<FlyingAway>),
    >,
    mut player_query: Query<
        (&GlobalTransform, &HitBox, &mut Health),
        (With<Player>, Without<Scythe>),
    >,
) {
    let Ok((player_transform, player_hb, mut health)) = player_query.get_single_mut() else {
        return;
    };

    for (scythe_transform, scythe_hb, scythe_entity) in enemy_scythe_query.iter() {
        if scythe_hb.intersects(
            scythe_transform.translation(),
            player_hb,
            player_transform.translation(),
        ) {
            console_log!("Player taking damage");

            // Decrement Player Health
            health.0 -= 1;

            // Handle Scythe
            commands.entity(scythe_entity).despawn_recursive();
        }
    }
}

pub fn handle_scythe_collision(
    mut commands: Commands,
    treat_odds_query: Query<&ChanceSpawnTreat>,
    mut ally_scythe_query: Query<
        (&GlobalTransform, &Transform, &HitBox, &mut Scythe, Entity),
        (
            With<Scythe>,
            With<TargetsEnemies>,
            Without<TargetsPlayer>,
            Without<FlyingAway>,
        ),
    >,
    mut enemy_scythe_query: Query<
        (&GlobalTransform, &Transform, &HitBox, &mut Scythe, Entity),
        (
            With<Scythe>,
            With<TargetsPlayer>,
            Without<TargetsEnemies>,
            Without<FlyingAway>,
        ),
    >,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut treat_spawner: ResMut<TreatSpawner>,
) {
    for (a_scythe_gt, a_scythe_t, a_scythe_hb, mut ally_scythe_str, ally_scythe) in
        ally_scythe_query.iter_mut()
    {
        let mut collided = false;

        for (e_scythe_gt, e_scythe_t, e_scythe_hb, mut enemy_scythe_str, enemy_scythe) in
            enemy_scythe_query.iter_mut()
        {
            if !collided
                && a_scythe_hb.intersects(
                    a_scythe_gt.translation(),
                    e_scythe_hb,
                    e_scythe_gt.translation(),
                )
            {
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

                // Chance of a treat when scythes collide
                let treat_odds = treat_odds_query.single();

                if rng.next_u32() % 500 < treat_odds.0 {
                    treat_spawner.num_treats += 1;

                    // Spawn a Treat in a random place!
                    commands.spawn(TreatBundle::new_at(
                        e_scythe_gt.translation(),
                        (rng.next_u32() % 4) as u8,
                    ));
                }
            }
        }
    }
}

const FLY_AWAY_TIMEOUT: f32 = 0.25;
const FLY_AWAY_SPEED: f32 = 10.0;
const FLY_AWAY_ROT: f32 = PI;

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

            flying_scythe_t.rotate(Quat::from_rotation_z(FLY_AWAY_ROT * time.delta_seconds()));
        }
    }
}

const FIXED_TREAT_SPAWN: f32 = 7.0;

pub fn treat_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut treat_spawner: ResMut<TreatSpawner>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
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
        commands.spawn(TreatBundle::new_at(
            pos.extend(0.0),
            (rng.next_u32() % 4) as u8,
        ));
    }
}

pub fn apply_levelup(
    skill_tracker: Res<SkillTracker>,
    mut scythe_speeds: Query<&mut ScytheSpeed, With<TargetsEnemies>>,
    mut treat_pickup: Query<&mut TreatPickupRadius, With<Treat>>,
    mut treat_drop_odds: Query<&mut ChanceSpawnTreat, With<Player>>,
) {
    // Check upgrade
    if skill_tracker.is_changed() {
        for mut scythe_speed in scythe_speeds.iter_mut() {
            scythe_speed.0 = skill_tracker.get(LevelOptions::ScytheSpeed);
        }

        for mut treat_radius in treat_pickup.iter_mut() {
            treat_radius.0 = skill_tracker.get(LevelOptions::TreatRadius);
        }

        let mut odds = treat_drop_odds.single_mut();

        odds.0 = skill_tracker.get(LevelOptions::TreatChance) as u32;
    }
}
