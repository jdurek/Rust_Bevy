// Supporting file that handles mainly components for a given tile location

use bevy::prelude::*;

use crate::{components::*, minimap::*, resources::*, };



/*  Basic overview of how this is used for the minimap
    In order to associate 'events' with a given location on a minimap, doing the following
    1) New entity created, with Coordinate, TileData, and other supporting components
    2) When player moves, check all TileEvents to see if we're on top of one (All entities with TileEvent should have Position)

*/
#[derive(Component)]
pub struct TileEvent;

#[derive(Component)]
pub struct TransitionTile {
    pub dest: String,  // Filepath to the destination map
    pub loc: Position, // Location on the destination map to be spawned at
}

#[derive(Component)]
pub struct TrapTile {
    pub trap_type: String, // TODO -replace with enum
    pub loc: Option<Position>, // Optional secondary location - for arrow traps as an example (Where is it shooting from?)
}

impl TrapTile {
    // Obtains specific details to the trap (Damage, etc...)
    pub fn fetch_trap(&self){}
}

#[derive(Component)]
pub struct EventTile {
    // TODO - add parameters to this (Method to fetch specific event from file?)
}

impl EventTile{
    // Obtains event data and stores it so we can access it upon reaching the tile
    pub fn fetch_event(&self){
        
    }
}