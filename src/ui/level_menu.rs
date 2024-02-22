use bevy::prelude::*;
use bevy_rand::{prelude::WyRand, resource::GlobalEntropy};
use rand_core::RngCore;

use crate::{components::prelude::LevelOptions, console_log, utils::*, GameState, SkillTracker};

use super::NORMAL_BUTTON;

#[derive(Component)]
pub struct LevelMenu;

pub fn setup_pause_menu(mut commands: Commands, mut rng: ResMut<GlobalEntropy<WyRand>>) {
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

    let option_1 = LevelOptions::get_random(rng.next_u32());

    let mut option_2 = LevelOptions::get_random(rng.next_u32());

    while option_1 == option_2 {
        option_2 = LevelOptions::get_random(rng.next_u32());
    }

    let mut option_3 = LevelOptions::get_random(rng.next_u32());

    while option_1 == option_3 || (option_2 == option_3) {
        option_3 = LevelOptions::get_random(rng.next_u32())
    }

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
                    option_1,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        option_1.to_string(),
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
                    option_2,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        option_2.to_string(),
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
                    option_3,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        option_3.to_string(),
                        button_text_style.clone(),
                    ),));
                });
        });
}

pub fn levelup_action(
    interaction_query: Query<(&Interaction, &LevelOptions), (Changed<Interaction>, With<Button>)>,
    mut skill_tracker: ResMut<SkillTracker>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // Hard coding for the time being
    for (interaction, level_option) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match level_option {
                LevelOptions::TreatRadius => {
                    skill_tracker.increment(LevelOptions::TreatRadius, 1.0)
                }
                LevelOptions::ScytheSpeed => {
                    skill_tracker.increment(LevelOptions::ScytheSpeed, 5.0)
                }
                LevelOptions::TreatChance => {
                    skill_tracker.increment(LevelOptions::TreatChance, 1.0)
                }
            }

            game_state.set(GameState::Game);
        }
    }
}
