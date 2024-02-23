use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct Treat(pub u8);

impl Treat {
    pub fn spawn(
        mut commands: Commands,
        query: Query<Entity, Added<Treat>>,
        asset_server: Res<AssetServer>,
    ) {
        for treat in query.iter() {
            commands.entity(treat).with_children(|parent| {
                parent.spawn(SpriteBundle {
                    texture: asset_server.load("textures/treat.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        }
    }
}

#[derive(Bundle)]
pub struct TreatBundle {
    transform: TransformBundle,
    collider: Collider,
    treat: Treat,
}

impl TreatBundle {
    pub fn new_at(position: Vec3, str: u8) -> Self {
        Self {
            transform: TransformBundle::from_transform(Transform {
                translation: position,
                scale: Vec2::ONE.extend(0.0),
                ..Default::default()
            }),
            collider: Collider,
            treat: Treat(str),
        }
    }
}
