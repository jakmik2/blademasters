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

const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1280.0;

fn main() {
    console_log!("It's working!");
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .add_plugins(GamePlugin)
        .run();
}
