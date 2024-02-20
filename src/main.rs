mod components;
mod resources;
mod systems;
mod utils;

use components::prelude::Enemy;
use resources::*;
use systems::*;

use bevy::{asset::AssetMetaCheck, prelude::*};

// use bevy_prng::WyRand;
use bevy_rand::prelude::*;

use utils::logging::*;

const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1280.0;

fn main() {
    console_log!("It's working!");
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#combative-survivors-canvas".into()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawner {
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
            health: 3,
        })
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup)
        // Make sure everything is spawned before any frame
        .add_systems(
            First,
            (treat_spawn, Enemy::spawn, enemy_spawner, add_scythe),
        )
        // Movement needs to be handled at a fixed step
        .add_systems(FixedUpdate, (move_scythe, move_player, hunt_player))
        // Update As Frequently as possible
        // .add_systems(Update, (move_scythe, move_player, hunt_player))
        // Late adds to ensure preframe resolution
        .add_systems(PostUpdate, handle_scythe_collision)
        .add_systems(
            Last,
            (
                handle_flying_away,
                handle_player_health,
                handle_ally_scythes,
                handle_enemy_scythes,
                update_ui,
            ),
        );
    }
}
