use bevy::prelude::*;

use super::NORMAL_BUTTON;

#[derive(Component)]
pub struct LevelMenu;

pub fn setup_pause_menu(mut commands: Commands) {
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
                .spawn((ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button 1",
                        button_text_style.clone(),
                    ));
                });

            parent
                .spawn((ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button 2",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button 3",
                        button_text_style.clone(),
                    ));
                });
        });
}
