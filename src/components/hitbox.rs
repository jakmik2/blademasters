use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

#[derive(Component)]
pub struct HitBox {
    pub x: f32,
    pub y: f32,
}

impl HitBox {
    pub fn new(dims: Vec2) -> Self {
        Self {
            x: dims.x,
            y: dims.y,
        }
    }

    pub fn intersects(&self, pos: Vec3, other_hb: &HitBox, other_pos: Vec3) -> bool {
        let self_aabb = Aabb2d::new(pos.truncate(), self.into());
        let other_aabb = Aabb2d::new(other_pos.truncate(), other_hb.into());

        self_aabb.intersects(&other_aabb)
    }
}

impl Into<Vec2> for &HitBox {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
