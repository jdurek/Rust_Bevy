//Stores basic overview on character classes

use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};


// List of class names - skills are handled separately
// Refer to design doc for more details on the classes themselves
#[derive(Serialize, Deserialize, InspectorOptions)]
pub enum CharClass {
    Rookie,
    Squire,
    Survivalist,
    Herbalist,
    Explorer
}