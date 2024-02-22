use bevy::{prelude::*, utils::HashMap};

use crate::components::levelups::{LevelOptions, LevelOptions::*};
use crate::components::prelude::DEFAULT_SCYTHYE_VELOCITY;

#[derive(Resource)]
pub struct SkillTracker {
    skills: HashMap<LevelOptions, f32>,
}

impl SkillTracker {
    pub fn get(&self, key: LevelOptions) -> f32 {
        self.skills.get(&key).unwrap().to_owned()
    }

    pub fn increment(&mut self, key: LevelOptions, amt: f32) {
        let option = self.skills.get_mut(&key).unwrap();

        *option += amt;
    }
}

impl Default for SkillTracker {
    fn default() -> Self {
        // Create base skills
        let mut skills: HashMap<LevelOptions, f32> = HashMap::new();

        skills.insert(ScytheSpeed, DEFAULT_SCYTHYE_VELOCITY);
        skills.insert(TreatRadius, 50.0);
        skills.insert(TreatChance, 0.0);

        Self { skills }
    }
}
