use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

// Basic management of tilemaps - These follow ECS logic for rendering.

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Traversal,
    Special,
}

// Simple list of floor types
#[derive(PartialEq, Copy, Clone)]
pub enum FloorType {
    Solid,
    Soft,           
    Liquid,         // Water or bodies of liquids where you can't walk normally
    Damaging,       // Lava, poison bogs, thorns, etc...
    Path,
    Void,
}

// Simple types for traversal methods - all of them will typically include one "Destination" that you can proceed to, although some may have multiple destinations
#[derive(PartialEq, Copy, Clone)]
pub enum TraversalType {
    Door,
    StairUp,
    StairDown,
    Warp,
    Hole,
    Flight, // Special case for when you can fly upwards (Or downwards if you were already flying)
}

// ECS style - Tile is just for a quick summary of the type, so we can decide which logic we need to check
// EG, Wall Type is impassible (for now), floor lets you move (But may cause effects), Traversal may have certain conditions (Like a locked door)
#[derive(Component)]
pub struct Tile {
    tileType: TileType,
}

