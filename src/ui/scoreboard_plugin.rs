use bevy::prelude::*;

use crate::{components::*, Score};

fn add_score_board(
    mut commands: Commands,
    player_query: Query<(Option<&Xp>, Option<&Health>)>,
    asset_server: ResMut<AssetServer>,
    score: Res<Score>,
) {
    let (unpacked_xp, unpacked_health) = player_query.single();

    let xp = match unpacked_xp {
        Some(x) => x.0,
        None => 0,
    };

    let health = match unpacked_health {
        Some(x) => x.0,
        None => 0,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(20.0),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("Health: {:?}\nScore: {:?}\nXp: {:?}", health, score.0, xp),
                    TextStyle {
                        font: asset_server.load("fonts/FFGhost-Regular.ttf"),
                        font_size: 30.0,
                        ..Default::default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..Default::default()
                }),
                Label,
                PlayerDisplay,
            ));
        });
}

#[derive(Default)]
pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_score_board);
    }
}
