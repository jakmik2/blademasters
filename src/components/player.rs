use bevy::prelude::*;
use levelups::ChanceSpawnTreat;

use crate::{console_log, utils::*};

use super::*;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    speed: Speed,
    sprite_bundle: SpriteBundle,
    xp: Xp,
    health: Health,
    collider: Collider,
    treat_chance: ChanceSpawnTreat,
}

impl PlayerBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        console_log!("Adding Player!");
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec2::ZERO.extend(0.0),
                    scale: Vec2::new(30.0, 30.0).extend(0.0),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            collider: Collider,
            player: Player,
            speed: Speed(150.0),
            health: Health(10),
            xp: Xp(0),
            treat_chance: ChanceSpawnTreat(0),
        }
    }
}
