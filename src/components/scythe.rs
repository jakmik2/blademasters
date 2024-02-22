use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rand::{prelude::WyRand, resource::GlobalEntropy};
use rand_core::RngCore;

use crate::{console_log, utils::*};

use super::{
    prelude::{ScytheSpeed, DEFAULT_SCYTHYE_VELOCITY},
    Collider,
};

#[derive(Component, Clone, Copy)]
pub struct Scythe(pub u8);

impl Scythe {
    pub fn spawn(
        mut commands: Commands,
        query: Query<Entity, Added<Scythe>>,
        mut rng: ResMut<GlobalEntropy<WyRand>>,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        for scythe in query.iter() {
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
                        rotation: Quat::from_rotation_z(3.0 * PI / 2.0),
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
    collider: Collider,
    // sprite_sheet_bundle: SpriteSheetBundle,
    transform: TransformBundle,
    scythe: Scythe,
    scythe_speed: ScytheSpeed,
}

const DEFAULT_CUSTOM_SIZE: Vec2 = Vec2::new(32., 32.);

impl ScytheBundle {
    pub fn new(rel_pos: Vec2, str: u8, speed: f32) -> Self {
        Self {
            // sprite_sheet_bundle: SpriteSheetBundle {
            transform: TransformBundle::from_transform(Transform {
                translation: rel_pos.extend(0.0).normalize() * 100.0,
                ..Default::default()
            }),
            //     sprite: Sprite {
            //         custom_size: Some(DEFAULT_CUSTOM_SIZE),
            //         // color: Color::ALICE_BLUE,
            //         ..Default::default()
            //     },
            //     texture,
            //     atlas: TextureAtlas { layout, index: () },
            //     ..Default::default()
            // },
            collider: Collider,
            scythe: Scythe(str),
            scythe_speed: ScytheSpeed(speed),
        }
    }

    pub fn new_with_speed(speed: f32) -> Self {
        Self::new(Vec2::ONE, 2, speed)
    }

    pub fn new_at(rel_pos: Vec2, str: u8) -> Self {
        Self::new(rel_pos, str, DEFAULT_SCYTHYE_VELOCITY)
    }
}
