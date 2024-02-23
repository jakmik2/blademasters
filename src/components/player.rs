use bevy::prelude::*;
use levelups::ChanceSpawnTreat;

use crate::{console_log, utils::*};

use super::*;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(
        mut commands: Commands,
        player_query: Query<Entity, Added<Player>>,
        asset_server: Res<AssetServer>,
    ) {
        let Ok(player) = player_query.get_single() else {
            return;
        };

        commands.entity(player).with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec2::ZERO.extend(0.0),
                    ..Default::default()
                },
                texture: asset_server.load("textures/cats/cat01.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(53., 60.)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    speed: Speed,
    transform: TransformBundle,
    xp: Xp,
    health: Health,
    collider: Collider,
    treat_chance: ChanceSpawnTreat,
}

impl PlayerBundle {
    pub fn new() -> Self {
        console_log!("Adding Player!");
        Self {
            transform: TransformBundle::from_transform(Transform {
                translation: Vec2::ZERO.extend(0.0),
                ..Default::default()
            }),
            collider: Collider,
            player: Player,
            speed: Speed(150.0),
            health: Health(10),
            xp: Xp(0),
            treat_chance: ChanceSpawnTreat(0),
        }
    }
}
