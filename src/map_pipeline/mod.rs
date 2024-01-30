use crate::prelude::*;
use crate::prelude::Map;
use std::fs;

use serde_json::*;


/* Serves as the central rendering hub for the minimap, and chief authority on what's stored where */

// mod map;
// pub use map::*;

// Sub-modules for specific functions, such as saving/loading map data


#[derive(Resource, Serialize, Deserialize)]
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
        let init_map: Map = Map::new(1,1);
        let pos: Position = Position {x: 0, y: 0, z: 0};
        MapBuilderA{
            map: init_map,
            player_start: pos,
            foe_locs: Vec::new(),
        }

    }

    pub fn load_map(file_path: String) -> Self {
        // Todo - determine if map_data is a struct passed into the arg, or a filepath that we need to fetch from
        let data = fs::read_to_string(file_path).expect("Unable to read file");
        let map_data:MapBuilderA =  serde_json::from_str(&data).unwrap();
        map_data
    }
}