use bevy::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct FlyingAway {
    pub counter: f32,
    pub trajectory: Vec3,
}

impl FlyingAway {
    pub fn new(trajectory: Vec3) -> Self {
        Self {
            trajectory,
            counter: 0.0,
        }
    }
}
