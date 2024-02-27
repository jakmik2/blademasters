use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

use crate::{console_log, utils::*};

use super::{prelude::*, TargetsPlayer};

const DEFAULT_SPRITE_SIZE: Vec2 = Vec2::new(53., 60.);

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

        let n = (rng.next_u32() % 8) + 2;

        let texture: Handle<Image> = asset_server.load(format!("textures/cats/cat0{:?}.png", n));

        // Configure the enemy when entity is added to the scene
        console_log!("Adding scythes;");

        let r_off =
            Vec2::new(rng.next_u32() as f32 % 30.0, rng.next_u32() as f32 % 30.0).normalize();

        let rot_one = Vec2::from_angle(2.0 * PI / 3.0).rotate(r_off);
        let rot_two = Vec2::from_angle(4.0 * PI / 3.0).rotate(r_off);

        commands.entity(enemy).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(DEFAULT_SPRITE_SIZE),
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn((ScytheBundle::new_at(r_off, 2), TargetsPlayer));
            parent.spawn((ScytheBundle::new_at(rot_one, 2), TargetsPlayer));
            parent.spawn((ScytheBundle::new_at(rot_two, 2), TargetsPlayer));
        });
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    hit_box: HitBox,
    transform: TransformBundle,
    enemy: Enemy,
}

impl EnemyBundle {
    pub fn new_at(position: Vec2) -> Self {
        console_log!("Spawning Enemy!");
        Self {
            transform: TransformBundle::from_transform(Transform::from_xyz(
                position.x, position.y, 0.0,
            )),
            hit_box: HitBox::new(DEFAULT_SPRITE_SIZE / 2.0),
            enemy: Enemy,
        }
    }
}
