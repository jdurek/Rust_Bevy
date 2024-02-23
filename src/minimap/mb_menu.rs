// Contains all the functions/setup for the minimap builder tool's menu


use bevy::prelude::*;

use crate::{components::*, minimap::*, resources::*, };


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MBMenuState {
    #[default]
    Awaiting,
    Save,
    Load,
    New,
}

// Defining a few menu constants, mainly for hover/click colors
const NORMAL_BUTTON: Color = Color::GRAY;
const HOVERED_BUTTON: Color = Color::DARK_GRAY;
const HOVERED_PRESSED: Color = Color::DARK_GREEN;
const PRESSED_BUTTON: Color = Color::GREEN;


// Plugin that manages the menu itself (Mainly state changes)
pub fn menu_plugin(app: &mut App){
    // TODO - move some of map_builder into this 
    // Mainly for add_systems OnEnter() and OnExit() for specific states
    app
        .add_systems(OnEnter(MBMenuState::Save), save_gui)
        // Add a system that checks to see if it's completed saving yet or not?
        .add_systems(OnExit(MBMenuState::Save), save_complete)
    
    ;
}

// Function for the menu pane on the left side of the window
pub fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let btn_style = Style{
        width: Val::Px(150.),
        height: Val::Px(50.),
        margin: UiRect::all(Val::Px(20.0)),
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
        .spawn(NodeBundle {
            style: Style{
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
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
                    // TODO - figure out how to add a slight padding to this so it's not glued to the top
                    parent.spawn(TextBundle::from_section("MapBuilder Tool v.0", btn_text_style.clone()));
                    
                    // Save Button
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

                    // Load Button
                    parent
                        .spawn((ButtonBundle {
                            style: btn_style.clone(),
                            background_color: Color::GRAY.into(),
                            ..default()
                        },
                        MenuButtonActions::Load,
                        ))
                        .with_children(|parent| {
                            // Add icon with save floppy - reference https://github.com/bevyengine/bevy/blob/latest/examples/games/game_menu.rs#L459
                            parent.spawn(TextBundle::from_section("Load Map", btn_text_style.clone(),
                            
                            ));
                        })
                    ;

                    // New Button
                    parent
                        .spawn((ButtonBundle {
                            style: btn_style.clone(),
                            background_color: Color::GRAY.into(),
                            ..default()
                        },
                        MenuButtonActions::New,
                        ))
                        .with_children(|parent| {
                            // Add icon with save floppy - reference https://github.com/bevyengine/bevy/blob/latest/examples/games/game_menu.rs#L459
                            parent.spawn(TextBundle::from_section("New Map", btn_text_style.clone(),
                            
                            ));
                        })
                    ;
                })
            ;
        })
    ;
}

pub fn menu_button_system(
    mut interact_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interact_query {
        *color = match(*interaction, selected) {
            // Match to the different interaction cases - need to define the colors used in advance
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

pub fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonActions), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MBMenuState>>,
    
){
    for(interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed{
            match menu_button_action{
                MenuButtonActions::Save => {
                    // Serialize the map data into a string (eventually, a file or DB store)
                    // The WallGrid and MapGrid have the appropiate labels, so we can serialize them
                    menu_state.set(MBMenuState::Save);
                }
                MenuButtonActions::New => {
                    // First, do a warning pop-up to make sure the user's sure they want to start anew
                    // To do this, we'll change state into New and just handle it within that

                    // For now, we'll just reset the map without any warnings and add that safeguard later
                    menu_state.set(MBMenuState::New);
                }
                _ => {
                    // Unimplemented case
                    println!("{:?} has not been implemented yet!", menu_button_action);
                }
            }
        }
    }
}

// Handles bringing up the Save GUI/tools (Or just open the file explorer for saving)
fn save_gui() {

}

// Cleanup anything we might need to after completing a save (Since we have to wait for it to completely save)
fn save_complete() {

}


// Handles the LoadMap interface (Or just open the file explorer and await it to complete)
// This will also need to either clean up the previous map, or have a toggle between them
fn load_gui(){}
// Clean up after load is complete
fn load_complete(){}

// Handles the NewMap interface (Very similar to load, just with some config steps like map size)
fn new_gui(){}
// Clean up after new is completed
fn new_complete(){}

