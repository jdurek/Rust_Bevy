// Contains all the functions/setup for the minimap builder tool's menu


use bevy::prelude::*;
use bevy::ecs::world;
use bracket_lib::color::BLACK;

use crate::{components::*, minimap::*, resources::*, };

// Function for the menu pane on the left side of the window
pub fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let btn_style = Style{
        width: Val::Px(150.),
        height: Val::Px(50.),
        align_items: AlignItems::Center,
        ..default()
    };
    let btn_icon_style = Style{
        width: Val::Px(30.),
        position_type: PositionType::Absolute,
        left: Val::Px(10.),
        ..default()
    };
    let btn_text_style = TextStyle{
        font_size: 20.0,
        color: Color::BLACK,
        ..default()
    };

    // Menu items will all be children to a larger bundle (for grouping)
    commands
        .spawn((NodeBundle {
            style: Style{
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }))
        .with_children(|parent| {
            parent
                // Background of the menu grouping
                .spawn(NodeBundle{
                    style: Style{ 
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display header of the menu
                    parent.spawn(TextBundle::from_section("MapBuilder Tool v.0", btn_text_style.clone()));
                    parent
                        .spawn((ButtonBundle {
                            style: btn_style.clone(),
                            background_color: Color::GRAY.into(),
                            ..default()
                        },
                        MenuButtonActions::Save,
                        ))
                        .with_children(|parent| {
                            // Add icon with save floppy - reference https://github.com/bevyengine/bevy/blob/latest/examples/games/game_menu.rs#L459
                            parent.spawn(TextBundle::from_section("Save Map", btn_text_style.clone(),
                    ));
                })
                ;
        });
    });
}

pub fn menu_button_system(
    mut interact_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interact_query {
        // *color = match(*interaction, selected) {
        //     // Match to the different interaction cases - need to define the colors used in advance

        //     (Interaction::None, None) => NORMAL_BUTTON.into()
        // }
    }
}
