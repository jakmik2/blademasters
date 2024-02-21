use std::f32::consts::PI;

use bevy::prelude::*;

pub const DEF_VEL: f32 = 3.0 * PI / 2.0;

#[derive(Component)]
pub struct ScytheSpeed(pub f32);

impl Default for ScytheSpeed {
    fn default() -> Self {
        Self(DEF_VEL)
    }
}

#[derive(Component)]
pub struct TreatRadius(pub f32);

#[derive(Component)]
pub struct ChanceSpawnTreat(pub f32);
