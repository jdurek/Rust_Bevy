use bevy::prelude::*;

// This file holds all generic components that don't have much complexity - such as positionings


// Most elements will typically have Z of 0, but sometimes something may be hidden in a tile on a different Z axis (Underground, Above)
// Consider burrowing enemies or avian enemies - they might not be visible until you actually engage with them
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// This just serves as a method to 'hide' certain things, or tweak how it's displayed - can use a glyph from RLTK as placeholder until later when graphics are implemented
#[derive(Component)]
pub struct Renderable {
    pub visible: bool,
    pub glyph: rltk::FontCharType,
}