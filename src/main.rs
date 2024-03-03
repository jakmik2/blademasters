mod components;
mod game;
mod resources;
mod systems;
mod ui;
mod utils;

use game::*;
use resources::*;

use bevy::{asset::AssetMetaCheck, prelude::*};

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use utils::fps_display::*;
use utils::logging::*;

use ui::*;

const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1280.0;

fn main() {
    console_log!("It's working!");
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, (setup_fps_counter, initial_setup))
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .add_plugins((GamePlugin, splash::splash_plugin, main_menu::menu_plugin))
        .run();
}

fn initial_setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Splash);

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    // TODO
}
