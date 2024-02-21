use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

use crate::*;

use self::components::prelude::*;
use self::systems::*;
use self::ui::*;

// use crate::*;
// use components::*;

pub struct GamePlugin;

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
        .insert_resource(LevelOptions {
            treat_pick_up_radius: 0,
            scythe_speed: DEF_VEL,
            spawn_treat: 0,
        })
        .init_state::<GameState>()
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(ScoreboardPlugin)
        .add_systems(Startup, setup)
        // Make sure everything is spawned before any frame
        .add_systems(
            First,
            (treat_spawn, Enemy::spawn, enemy_spawner, add_scythe)
                .run_if(in_state(GameState::Game)),
        )
        // Movement needs to be handled at a fixed step
        .add_systems(
            FixedUpdate,
            (move_scythe, move_player, hunt_player).run_if(in_state(GameState::Game)),
        )
        // Update As Frequently as possible
        .add_systems(
            Update,
            (
                (button_system, levelup_action).run_if(in_state(GameState::LevelUp)),
                level_up.run_if(in_state(GameState::Game)),
            ),
        )
        // Late adds to ensure preframe resolution
        .add_systems(
            PostUpdate,
            handle_scythe_collision.run_if(in_state(GameState::Game)),
        )
        .add_systems(OnEnter(GameState::LevelUp), (setup_pause_menu)) // Build Pause Menu
        .add_systems(OnExit(GameState::LevelUp), despawn_screen::<LevelMenu>) // Clean up Pause Menu
        .add_systems(OnEnter(GameState::Game), apply_levelup)
        .add_systems(
            Last,
            (
                handle_flying_away,
                handle_player_health,
                handle_ally_scythes,
                handle_enemy_scythes,
                update_ui,
            )
                .run_if(in_state(GameState::Game)),
        );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Game,
    Pause,
    LevelUp,
}

fn level_up(mut player_data: ResMut<PlayerData>, mut game_state: ResMut<NextState<GameState>>) {
    // When Enough xp, level up!
    if player_data.is_changed() && player_data.score >= 1 {
        player_data.score = 0;

        game_state.set(GameState::LevelUp);
    }
}
