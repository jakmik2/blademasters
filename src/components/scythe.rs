use std::f32::consts::PI;

use bevy::prelude::*;

use super::{prelude::ScytheSpeed, Collider};

#[derive(Component, Clone, Copy)]
pub struct Scythe(pub u8);

#[derive(Bundle)]
pub struct ScytheBundle {
    collider: Collider,
    sprite_bundle: SpriteBundle,
    scythe: Scythe,
    scythe_speed: ScytheSpeed,
}

impl ScytheBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::ONE * 3.0,
                    rotation: Quat::from_rotation_z(-PI/2.),
                    scale: Vec2::new(0.5, 0.5).extend(0.0),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            collider: Collider,
            scythe: Scythe(2),
            scythe_speed: ScytheSpeed::default(),
        }
    }

    pub fn new_with_speed(speed: f32, texture: Handle<Image>) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::ONE * 3.0,
                    rotation: Quat::from_rotation_z(-PI/2.),
                    // TODO scale issues
                    scale: Vec2::new(0.5, 0.5).extend(0.0),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            collider: Collider,
            scythe: Scythe(2),
            scythe_speed: ScytheSpeed(speed),
        }
    }

    pub fn new_at(rel_pos: Vec2, str: u8, place: i8, texture: Handle<Image>) -> Self {
        Self {
            
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: rel_pos.extend(0.0).normalize() * 3.0,
                    rotation: match place {
                        // rotate the enemy's blade based on number of scythe
                        // TODO find correct angles and correct PINWHEELING
                        0 => Quat::from_rotation_z(-PI/2.),
                        1 => Quat::from_rotation_z(PI/4.),
                        2 => Quat::from_rotation_z((3.*PI)/4.),
                        _ => Quat::from_rotation_z(-PI/4.),
                    },
                    // TODO scale issues
                    scale: Vec2::new(0.5, 0.5).extend(0.0),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            collider: Collider,
            scythe: Scythe(str),
            scythe_speed: ScytheSpeed::default(),
        }
    }
}
