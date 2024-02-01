use crate::prelude::*;
use crate::prelude::Map;
use std::fs;

use bevy::ecs::system::adapter::new;
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

    pub fn save_map(self, to_file: String) {
        // Attempts to save the struct to a JSON structure, then write to a file
        // For now, it'll just dump a JSON string
        let j = serde_json::to_string(&self);
        let jstr = match j {
            Ok(output) => output,
            Err(error) => panic!("Unable to save map - {:?}", error),
        };
        println!("{}", jstr);

        
    }
}


pub fn placeholder(mut commands:Commands){}

// Debug function for testing the save/load functionality before a test suite is integrated
pub fn save_load_test(mut commands:Commands){
    println!("Performing a Save-Load test, dumping JSONs...");
    // Create a mapBuilder, and call save_map on the default map struct
    let mb = MapBuilderA::new();
    mb.save_map("dummy_path".to_string());
    
    // Create a mapBuilder, edit the map itself, and call save_map
    let mut mb = MapBuilderA::new();
    mb.map = Map::new(16,16);
    mb.save_map("DummyPath".to_string());

}

// TODO - add build_map function here (Move from the map.rs file)

pub struct MapPlugin;
impl Plugin for MapPlugin
{
    fn build(&self, app: &mut App)
    {
        app
        // Test functions
        .add_systems(Startup, save_load_test)
        // When loading in from the menu
        .add_systems(OnEnter(GameplayState::Exploration), placeholder)
        .add_systems(OnExit(GameplayState::Exploration), placeholder)

        // Handle entering a dungeon - fetch the map information and begin loading everything
        .add_systems(OnEnter(TurnState::EnterDungeon), placeholder) // Begin rendering next map, move prior one into cache if needed
        .add_systems(OnExit(TurnState::EnterDungeon),placeholder)   // Return control to the player (Loading and other things has completed)

        // Handle leaving a dungeon (reset map, cache it, etc...)
        .add_systems(OnEnter(TurnState::ExitDungeon), placeholder)  // Handle whatever EnterDungeon usually wouldn't - reset some things on the map like enemies?
        .add_systems(OnExit(TurnState::ExitDungeon), placeholder)

        // Handle movement of entitites on the map on different phases - 
        .add_systems(OnExit(TurnState::PlayerTurn), placeholder)    // Either move player icon or shift the map depending on map size
        .add_systems(OnExit(TurnState::EnemyTurn), placeholder)     // Move enemy icons if they've moved
        .add_systems(OnExit(TurnState::OtherTurn), placeholder)             // Handle other map changes if something's been triggered (EG, water levels, rockslides, etc...)
        
        ;
    }
}