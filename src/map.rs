use crate::prelude::*;

// use bevy_ecs_tilemap::prelude::*;

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


#[derive(Component)]
pub struct MapTile;

// ECS style - Tile is just for a quick summary of the type, so we can decide which logic we need to check
// EG, Wall Type is impassible (for now), floor lets you move (But may cause effects), Traversal may have certain conditions (Like a locked door)
#[derive(Component, Clone)]
pub struct TileData {
    pub tile_type: TileType,
    pub tile_position: Position,         // From components.rs 
    // pub tileTexture: TileTextureIndex,  // From bevy_ecs_tilemap tiles mod.rs
    pub tile_visible: Renderable,        // From components.rs
}

// TODO - Update the struct so we can serialize/deserialize (Saving and Lodaing maps and things that have changed)
// #[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub dim_x: i32,
    pub dim_y: i32,
    pub tile_data: Vec<TileData>,
    pub known_tiles: Vec<bool>,
    // pub visibleTiles: Vec<bool>,
}

impl Map {
    pub fn xy_index(&self, x:i32, y:i32) -> usize {
        (y as usize * self.dim_x as usize) + x as usize
    }

    // Creates a 'default' map that is entirely made up of floors and nothing else (Open Field)
    pub fn new(width: i32, height:i32) -> Map {
        Map {
            dim_x: width,
            dim_y: height,
            known_tiles: vec![false; (width*height) as usize],
            tile_data: vec![TileData {tile_type:TileType::Floor, tile_position:Position{x:0,y:0,z:0}, tile_visible:Renderable{visible:true}}; (width*height) as usize],
        }
    }
}

trait MapArchitect {
    fn new(&mut self) -> MapBuilder;
}

#[derive(Resource)]
pub struct MapBuilder {
    pub map: Map,
    pub player_start: Position,
}

impl MapBuilder {
    pub fn new() -> Self
    {
        // Perform call to generate map - If it's a randomly generated one, use https://github.com/thephet/BevyRoguelike/blob/main/src/map_builder/mod.rs#L39 as ref
        // Otherwise, just make a basic grid
        let init_map: Map = Map::new(32,32);
        let pos: Position = Position {x: 0, y: 0, z: 0};
        MapBuilder{
            map: init_map,
            player_start: pos,
        }
    }
}

// Takes 2 main args - the commands, and the MapBuilder (which contains the map to load)
pub fn draw_map(mut commands: Commands, mb: Res<MapBuilder>) {
    for y in 0..mb.map.dim_y {
        for x in 0..mb.map.dim_x {
            let index: usize = mb.map.xy_index(x, y);

            // Using the TileType from TileData, generate the information
            match mb.map.tile_data[index].tile_type 
            {
                TileType::Floor => {
                    commands
                    .spawn(SpriteBundle{
                        sprite: Sprite { color: (Color::GREEN), custom_size: (Some(Vec2::new(1.0, 1.0))), ..Default::default() },
                        visibility: Visibility::Visible,
                        transform: Transform {
                            translation: Vec2::new(x as f32, y as f32).extend(0.0),
                            scale: Vec3::new(1.0, 1.0, 1.0),
                            ..default()
                        },
                        ..Default::default()
                    }) //SpriteBundle
                    .insert(MapTile)
                    .insert(Position {x: x, y: y, z: 0})
                    // .insert(TileSize::Square(1.0))
                    ;
                }
                TileType::Wall => {
                    commands.spawn((
                        MapTile,
                        Position {x, y, z: 0}, 
                        //TileSize::square(1.0),
                        SpriteBundle {
                            sprite: Sprite { color: (Color::BLACK), custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                            visibility: Visibility::Visible,
                            ..Default::default()
                        }
                    ));
                }
                TileType::Traversal => {
                    // Adds entity with specific traversal details to the tile so we can fetch it later
                    // commands.spawn((Position {x,y,z:1}, TraversalData))
                    ()
                }
                _ => {  // Invalid Tile Type provided - Treat it like a wall, but render it as an "Error" so we can see it

                }
            }
        }
    }
    // print!("Map has been drawn (allegedly)");
}

pub fn build_map(mut commands: Commands) {
    // Can expland with a query to adjust which map is being loaded - for now, just call our function
    let mb = MapBuilder::new();
    commands.insert_resource(mb);
}