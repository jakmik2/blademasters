use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

use crate::*;

use self::components::prelude::*;
use self::components::Xp;
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
        .insert_resource(Score(0))
        .insert_resource(SkillTracker::default())
        .init_state::<GameState>()
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(ScoreboardPlugin)
        .add_systems(Startup, setup)
        // Make sure everything is spawned before any frame
        .add_systems(
            First,
            (
                treat_spawn,
                Enemy::spawn,
                Player::spawn,
                Scythe::spawn,
                Treat::spawn,
                enemy_spawner,
                add_scythe,
            )
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

fn level_up(mut xp_query: Query<&mut Xp>, mut game_state: ResMut<NextState<GameState>>) {
    let option_xp = xp_query.get_single_mut();

    // When Enough xp, level up!
    match option_xp {
        Ok(mut xp) => {
            if xp.0 >= 5 {
                xp.0 = 0;
                game_state.set(GameState::LevelUp);
            }
        }
        _ => (),
    }
}
