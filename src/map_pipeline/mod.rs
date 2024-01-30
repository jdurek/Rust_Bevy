use crate::prelude::*;


/* Serves as the central rendering hub for the minimap, and chief authority on what's stored where */

mod map;
pub use map::*;

// Sub-modules for specific functions, such as saving/loading map data


#[derive(Resource)]
pub struct MapBuilderA {
    pub map: Map,
    pub player_start: Position,
    pub foe_locs: Vec<Position>  // The on-field enemies will know their pathfinding and starting direction internally
    // pub theme - 'skybox' or coloring for our map
}

impl MapBuilderA {
    pub fn new() -> Self
    {
        // Creates default map object of 1x1 - load function will replace the map element entirely


    }

    pub fn load_map(map_data:i32) -> Self {
        // Todo - determine if map_data is a struct passed into the arg, or a filepath that we need to fetch from
    }
}