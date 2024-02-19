use bevy::prelude::*;

use super::Collider;

#[derive(Component)]
pub struct Scythe(u8);

#[derive(Bundle)]
pub struct ScytheBundle {
    collider: Collider,
    sprite_bundle: SpriteBundle,
    scythe: Scythe,
}

impl ScytheBundle {
    pub fn new() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::ONE * 3.0,
                    scale: Vec2::new(0.5, 0.5).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::ALICE_BLUE,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            scythe: Scythe(2),
        }
    }

    pub fn new_at(rel_pos: Vec2) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: rel_pos.extend(0.0).normalize() * 3.0,
                    scale: Vec2::new(0.5, 0.5).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::ALICE_BLUE,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            scythe: Scythe(2),
        }
    }
}
