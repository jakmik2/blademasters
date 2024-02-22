use std::f32::consts::PI;

use bevy::{prelude::*, render::texture};
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

use crate::{console_log, utils::*};

use super::{prelude::ScytheBundle, Collider, TargetsPlayer};

#[derive(Component)]
pub struct Enemy;

impl Enemy {
    pub fn spawn(
        mut commands: Commands,
        enemy_query: Query<Entity, Added<Enemy>>,
        mut rng: ResMut<GlobalEntropy<WyRand>>,
        asset_server: Res<AssetServer>,
    ) {
        let Ok(enemy) = enemy_query.get_single() else {
            return;
        };

        // Configure the enemy when entity is added to the scene
        console_log!("Adding scythes;");

        let r_off =
            Vec2::new(rng.next_u32() as f32 % 30.0, rng.next_u32() as f32 % 30.0).normalize();

        let rot_one = Vec2::from_angle(2.0 * PI / 3.0).rotate(r_off);
        let rot_two = Vec2::from_angle(4.0 * PI / 3.0).rotate(r_off);

        commands.entity(enemy).with_children(|parent| {
            parent.spawn((ScytheBundle::new_at(
                            r_off,
                            2,
                            0,
                            asset_server.load("blades/blade00.png")), TargetsPlayer)
                        );
            parent.spawn((ScytheBundle::new_at(
                            rot_one,
                            2,
                            1,
                            asset_server.load("blades/blade00.png")), TargetsPlayer)
                        );
            parent.spawn((ScytheBundle::new_at(
                            rot_two,
                            2,
                            2,
                            asset_server.load("blades/blade00.png")), TargetsPlayer)
                        );
        });
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    collider: Collider,
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
}

impl EnemyBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::ZERO,
                    scale: Vec2::new(0.25, 0.25).extend(0.0),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            collider: Collider,
            enemy: Enemy,
        }
    }

    pub fn new_at(position: Vec2, texture: Handle<Image>) -> Self {
        console_log!("Spawning Enemy!");
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec2::new(30., 30.).extend(0.0),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            collider: Collider,
            enemy: Enemy,
        }
    }
}
