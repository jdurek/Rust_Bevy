use bevy::prelude::*;
use serde::{Deserialize, Serialize};


// Exposes most generic components that might be shared among multiple modules
pub mod party;
use party::*;

// Most elements will typically have Z of 0, but sometimes something may be hidden in a tile on a different Z axis (Underground, Above)
// Consider burrowing enemies or avian enemies - they might not be visible until you actually engage with them
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// This just serves as a method to 'hide' certain things, or tweak how it's displayed - can use a glyph from RLTK as placeholder until later when graphics are implemented
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Renderable {
    pub visible: bool,
    // pub glyph: rltk::FontCharType,
}

// Struct denoting something can be interacted with in the overworld (NPCs, items, etc...)
// This is mainly for helping 'render' the interact prompt
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Interactable;

// Simple component that lets us query parties or units involved in combat more quickly
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct InCombat;

// Component for attaching a Health parameter to anything -
// Centralized here since some non-living things may need health 
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Health{
    hp: i32,    // Could set it to unsigned int if we need more HP later
}



// TODO - move this enum to the menu modules
#[derive(Component, Debug)]
pub enum MenuButtonActions {
    New,
    Save,
    Load,
    Undo,
    Redo,
}