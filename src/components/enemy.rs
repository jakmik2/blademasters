use bevy::prelude::*;

use crate::{console_log, utils::*};

use super::{prelude::ScytheBundle, Collider, TargetsPlayer};

#[derive(Component)]
pub struct Enemy;

impl Enemy {
    pub fn spawn(mut commands: Commands, enemy_query: Query<Entity, Added<Enemy>>) {
        let Ok(enemy) = enemy_query.get_single() else {
            return;
        };

        // Configure the enemy when entity is added to the scene
        console_log!("Adding scythes;");
        commands.entity(enemy).with_children(|parent| {
            parent.spawn((ScytheBundle::new_at(Vec2::NEG_ONE, 3), TargetsPlayer));
            parent.spawn((ScytheBundle::new_at(Vec2::ONE, 2), TargetsPlayer));
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
    pub fn new() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::ZERO,
                    scale: Vec2::new(1.0, 1.0).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::BISQUE,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            enemy: Enemy,
        }
    }

    pub fn new_at(position: Vec2) -> Self {
        console_log!("Spawning Enemy!");
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec2::new(30.0, 30.0).extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::BISQUE,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            enemy: Enemy,
        }
    }
}
