use bevy::prelude::*;

use self::prelude::*;

use super::*;

#[derive(Component)]
pub struct Treat(pub u8);

const SIZE: Vec2 = Vec2::new(16.0, 16.0);

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
                        custom_size: Some(SIZE),
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
    hit_box: HitBox,
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
            treat: Treat(str),
            hit_box: HitBox::new(SIZE / 2.0),
        }
    }
}
