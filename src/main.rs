mod components;
mod resources;
mod systems;
mod utils;

use components::prelude::Enemy;
use resources::*;
use systems::*;

use bevy::{asset::AssetMetaCheck, prelude::*};

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
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player, hunt_player, move_scythe)) // Move at fixed step
        .add_systems(
            PreUpdate,
            (Enemy::spawn, enemy_spawner, treat_spawn, add_scythe),
        ) // Handle spawns before frame
        .add_systems(
            Update,
            (
                handle_ally_scythes,
                handle_enemy_scythes,
                handle_player_health,
            ),
        )
        .add_systems(PostUpdate, (update_ui));
    }
}
