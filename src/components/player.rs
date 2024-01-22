use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;


#[derive(Component, InspectorOptions)]
pub struct PlayerComponent {
    // Stores the main details on the 'player' itself - this is separate from a 'character unit' - think like the guild as a whole, etc...
    pub guild_level: f32,
    pub guild_name: String,
    pub num_members: f32,
    
}

impl PlayerComponent {
    pub fn new() -> Self {
        // Placeholder values for everything
        PlayerComponent {
            guild_level: 0,
            guild_name: "Generics".to_string(),
            num_members: 1, // TODO - decide if we want to start this at 0 and add our character to it automatically? 
        }
    }
}