use bevy::{
    gizmos::aabb,
    math::bounding::{Aabb2d, AabbCast2d, Bounded2d, BoundingVolume},
    prelude::*,
    render::primitives::Aabb,
};

pub mod enemy;
pub mod flying_away;
pub mod hitbox;
pub mod levelups;
pub mod player;
pub mod scythe;
pub mod treat;

pub mod prelude {
    pub use crate::components::enemy::*;
    pub use crate::components::flying_away::*;
    pub use crate::components::hitbox::*;
    pub use crate::components::levelups::*;
    pub use crate::components::player::*;
    pub use crate::components::scythe::*;
    pub use crate::components::treat::*;
}

// Common Components

#[derive(Component)]
pub struct Health(pub usize);

#[derive(Component)]
pub struct Xp(pub usize);

#[derive(Component)]
pub struct PlayerDisplay;

#[derive(Component)]
pub struct TargetsEnemies;

#[derive(Component)]
pub struct TargetsPlayer;
