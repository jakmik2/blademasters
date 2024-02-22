use bevy::prelude::*;

use self::prelude::TreatPickupRadius;

use super::*;

#[derive(Component)]
pub struct Treat(pub u8);

#[derive(Bundle)]
pub struct TreatBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    treat: Treat,
}

impl TreatBundle {
    pub fn new_at(position: Vec3, str: u8) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position,
                    scale: Vec2::new(10.0, 10.0).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::FUCHSIA,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            treat: Treat(str),
        }
    }
}
