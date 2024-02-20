use bevy::prelude::*;

pub mod enemy;
pub mod flying_away;
pub mod player;
pub mod scythe;
pub mod treat;

pub mod prelude {
    pub use crate::components::enemy::*;
    pub use crate::components::flying_away::*;
    pub use crate::components::player::*;
    pub use crate::components::scythe::*;
    pub use crate::components::treat::*;
}

// Common Components
#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct PlayerDisplay;

#[derive(Component)]
pub struct TargetsEnemies;

#[derive(Component)]
pub struct TargetsPlayer;
