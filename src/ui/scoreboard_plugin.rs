use bevy::prelude::*;

use crate::{components::PlayerDisplay, PlayerData};

fn add_score_board(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    player_data: Res<PlayerData>,
) {
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
                    format!(
                        "Health: {:?}\nScore: {:?}",
                        player_data.health, player_data.score
                    ),
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
