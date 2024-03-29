use std::f32::consts::PI;

use bevy::{gizmos::aabb, math::bounding::Aabb2d, prelude::*};
use bevy_rand::{prelude::WyRand, resource::GlobalEntropy};
use rand_core::RngCore;

use crate::{console_log, utils::*};

use super::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Scythe(pub u8);

impl Scythe {
    pub fn spawn(
        mut commands: Commands,
        query: Query<(&Transform, Entity), Added<Scythe>>,
        mut rng: ResMut<GlobalEntropy<WyRand>>,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        for (transform, scythe) in query.iter() {
            let texture: Handle<Image> = asset_server.load("textures/blade-preview.png");

            let texture_atlas = TextureAtlasLayout::from_grid(
                Vec2::new(32.0, 32.0), // The size of each image
                5,                     // The number of columns
                4,                     // The number of rows
                None,                  // Padding
                None,                  // Offset
            );

            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let n = (rng.next_u32() % 20) as usize;

            commands.entity(scythe).with_children(|parent| {
                parent.spawn(SpriteSheetBundle {
                    texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_handle,
                        index: n,
                    },
                    sprite: Sprite {
                        custom_size: Some(DEFAULT_CUSTOM_SIZE),
                        ..Default::default()
                    },
                    transform: Transform {
                        rotation: Quat::from_rotation_z(
                            (Vec2::ONE).angle_between(transform.translation.truncate()) - PI / 2.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        }
    }
}

#[derive(Bundle)]
pub struct ScytheBundle {
    hit_box: HitBox,
    transform: TransformBundle,
    scythe: Scythe,
    scythe_speed: ScytheSpeed,
}

const DEFAULT_CUSTOM_SIZE: Vec2 = Vec2::new(32., 32.);

impl ScytheBundle {
    pub fn new(rel_pos: Vec2, dist: f32, str: u8, speed: f32) -> Self {
        Self {
            transform: TransformBundle::from_transform(Transform {
                translation: rel_pos.extend(0.0).normalize() * dist,
                ..Default::default()
            }),
            hit_box: HitBox::new(Vec2::new(16.0, 32.0) / 2.0),
            scythe: Scythe(str),
            scythe_speed: ScytheSpeed(speed),
        }
    }

    pub fn new_with_speed(speed: f32) -> Self {
        Self::new(Vec2::ONE, 100.0, 2, speed)
    }

    pub fn new_at(rel_pos: Vec2, str: u8) -> Self {
        Self::new(rel_pos, 100.0, str, DEFAULT_SCYTHYE_VELOCITY)
    }
}
