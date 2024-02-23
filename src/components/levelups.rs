use core::fmt;
use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{console_log, utils::*};

pub const DEFAULT_SCYTHYE_VELOCITY: f32 = 3.0 * PI / 2.0;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LevelOptions {
    #[default]
    ScytheSpeed,
    TreatRadius,
    TreatChance,
}

impl LevelOptions {
    pub fn get_random(rnum: u32) -> Self {
        let n = rnum % 3;

        console_log!("{:?}", n);

        match n {
            0_u32 => LevelOptions::TreatChance,
            1_u32 => LevelOptions::TreatRadius,
            2_u32 => LevelOptions::ScytheSpeed,
            _ => panic!("This shouldn't be reachable"),
        }
    }
}

impl fmt::Display for LevelOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LevelOptions::ScytheSpeed => write!(f, "Scythe Speed"),
            LevelOptions::TreatRadius => write!(f, "Treat Radius"),
            LevelOptions::TreatChance => write!(f, "Treat Chance"),
        }
    }
}

#[derive(Component)]
pub struct ScytheSpeed(pub f32);

impl Default for ScytheSpeed {
    fn default() -> Self {
        Self(DEFAULT_SCYTHYE_VELOCITY)
    }
}

#[derive(Component)]
pub struct TreatPickupRadius(pub f32);

#[derive(Component)]
pub struct ChanceSpawnTreat(pub u32);
