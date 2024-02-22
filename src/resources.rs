pub mod skill_tracker;

pub use skill_tracker::*;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub usize);

#[derive(Resource)]
pub struct EnemySpawner {
    pub num_enemies: usize,
    pub num_enemies_killed: usize,
    pub counter: f32,
}

#[derive(Resource)]
pub struct TreatSpawner {
    pub num_treats: usize,
    pub counter: f32,
}
