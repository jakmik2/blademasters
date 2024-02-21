use bevy::prelude::*;
use bevy_rand::{prelude::WyRand, resource::GlobalEntropy};

use crate::{console_log, utils::*, GameState, LevelOptions};

use super::NORMAL_BUTTON;

#[derive(Component)]
pub struct LevelMenu;

pub fn setup_pause_menu(
    mut commands: Commands,
    level_options: Res<LevelOptions>,
    _rng: ResMut<GlobalEntropy<WyRand>>,
) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let _button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: Color::OLIVE,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            LevelMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    LevelButtonAction::First,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Button 1",
                        button_text_style.clone(),
                    ),));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    LevelButtonAction::Second,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Faster Swords",
                        button_text_style.clone(),
                    ),));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    LevelButtonAction::Third,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Button 3",
                        button_text_style.clone(),
                    ),));
                });
        });
}

#[derive(Component)]
pub enum LevelButtonAction {
    First,
    Second,
    Third,
}

pub fn levelup_action(
    interaction_query: Query<
        (&Interaction, &LevelButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut level_options: ResMut<LevelOptions>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // Hard coding for the time being
    for (interaction, level_up_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match level_up_action {
                LevelButtonAction::First => level_options.treat_pick_up_radius += 1,
                LevelButtonAction::Second => level_options.scythe_speed *= 1.5,
                LevelButtonAction::Third => level_options.spawn_treat += 1,
            }

            game_state.set(GameState::Game);
        }
    }
}
