use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::render::camera::Viewport;
use bevy::render::view::visibility::RenderLayers;
use bevy::window::PrimaryWindow;
use bevy::prelude::*;
use serde::*;
use std::fs::*;
use std::io::Read;
// use crate::components::*;

pub mod draw_map;
pub use draw_map::*;
pub mod mb_menu;
pub use mb_menu::*;

use crate::components::Position;


#[derive(Component)]
pub struct TileComp;

// Alternate tile type - this one holds wall data as well for faster checks
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Tile {
    pub walls: [bool; 4], // Tuple representing the 4 directions (NSEW) and if we can move in those directions
    // pub kind: bool,
    // TODO - Turn kind into an enum for more features later
}

#[derive(Resource, Serialize, Deserialize, Copy, Clone)]
pub struct Wall {
    pub vis: bool,  // Is the wall visible yet (EG, render it or not)
    pub pres: bool, // Is there a wall?
    // pub kind: bool, // Flag that indicates if it's a normal wall or something else
    // TODO - replace the bool above with an enum when adding more features
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct WallGrid {
    pub walls: Vec<Wall>,
    pub dim_x: i32,
    pub dim_y: i32
}

impl WallGrid {
    pub fn new(width: i32, height:i32) -> WallGrid{
        // Creates a new vector using the provided dimensions
        WallGrid{
            dim_x: width,
            dim_y: height,
            walls: vec![Wall {vis:false, pres:false}; ((width+1) * height + (height+1) * width) as usize],
        }

    }

    // Helper function to determine 'where' the wall is relative to a line provided
    // Expects the coordinates to be a distance of 1, and no further
    pub fn wall_index(&self, x1:i32, y1:i32, x2:i32, y2:i32) -> Result<usize, String> {
        // Error handling: Invalid Line, Dest out of Bounds, 

        if (x1-x2).abs() + (y1-y2).abs() != 1 {
            // Distance is not equal to 1 - cannot create a new line
            return Err(String::from("Invalid Line"))
        }
        if x1 > self.dim_x || x2 > self.dim_x || x1 < 0 || x2 < 0 
         || y1 > self.dim_y|| y2 > self.dim_y || y1 < 0 || y2 < 0 {
            // Start or End coordinate is outside of our map's vector
            return Err(String::from("Coordinate out of bounds"))
        }

        // println!("Valid line; dim_x: {}  dim_y: {}", self.dim_x, self.dim_y);
        // Line is valid - translate to the Vector positions
        let x_diff = x1 - x2;
        let y_diff = y1 - y2;
        let mut index = 0;
        match (x_diff, y_diff) {
            // Odd Rows 
            (1, 0) => { // Leftward
                 // Take Y coordinate, multiply by (x+y+1), add X - 1
                index = (2 * self.dim_x + 1) * y1 + (x1 - 1);
            }, 
            (-1, 0) => { // Rightward
                // Take Y coordinate, multiply by (x+y+1), add current X
                index = (2 * self.dim_x + 1) * y1 + (x1);
            }, 
            // Even Rows
            (0, 1) => { // Downward
                index = (2 * self.dim_x + 1) * y1 - 1 - self.dim_y + x1;

            },
            (0, -1) => { // Upward
                index = (2 * self.dim_x + 1) * y1 + self.dim_y + x1;
 
            }, 
            _ => {
                return Err(String::from("Unknown Match case in WallGrid"))
            }
        };
        // println!("Wall added {}", index);
        Ok(index as usize)

    }

    // Add wall - final validation is done by wall_index()
    // If wall already existed - just overrides with a new default wall
    pub fn add_wall(&mut self, x1:i32, y1:i32, x2:i32, y2:i32){
        // Unpack result of wall_index (If we get out of bounds, handle it)
        if let Ok(wall_loc) = self.wall_index(x1, y1, x2, y2){
            // Update entry in WallGrid
            
            self.walls[wall_loc] = Wall {vis:true, pres: true};
        }
        else {
            // Wall index was invalid (Not within the bounds of the map)
            // Do error handling here later if we need to
        }
    }
    
    // Opposite of add_wall, but takes the same args
    // If wall didn't exist, can keep going without any issues
    pub fn remove_wall(&mut self, x1:i32, y1:i32, x2:i32, y2:i32){
        if let Ok(wall_loc) = self.wall_index(x1, y1, x2, y2){
            // Update entry in WallGrid
            self.walls[wall_loc] = Wall {vis:true, pres: false};
        }
        else {
            // Wall index was invalid (Not within the bounds of the map)
            // Don't display wall
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct MapGrid {
    pub tiles: Vec<Tile>,
    pub dim_x: i32,
    pub dim_y: i32,
    pub zoom: f32,
}

impl MapGrid {
    pub fn new(width: i32, height:i32) -> MapGrid {
        MapGrid {
            dim_x: width,
            dim_y: height,
            tiles: vec![Tile {walls:[false,false,false,false],}; (width*height) as usize],
            zoom: ZOOM_LEVEL,
        }
    }

    // Translate (x,y) coordinate into Vector index directly
    // Potential TODO - add error handling so we can just call this directly?
    pub fn xy_index(&self, x:i32, y:i32) -> i32 {
        (y * self.dim_x ) + x
    }

    // Given a line of 2 points, figure out which 1-2 grids are involved
    // TODO - Fix the values fed into xy_index and the mappings - some of it's correct? 
    pub fn grid_index(&self, x1:i32, y1:i32, x2:i32, y2:i32) -> Result<[i32;2], String> {

        // Validate the line length
        if (x1-x2 + y1-y2).abs() != 1 {
            // Distance is not equal to 1 - cannot use this line
            return Err(String::from("Invalid Line"))
        }
        if x1 > self.dim_x || x2 > self.dim_x || x1 < 0 || x2 < 0 
         || y1 > self.dim_y || y2 > self.dim_y || y1 < 0 || y2 < 0 {
            // Start or End coordinate is outside of our map
            return Err(String::from("Coordinate out of bounds"))
        }

        // Line is valid - translate into our Vector
        let x_diff = x1 - x2;
        let y_diff = y1 - y2;
        let mut index = [0 , 0];
        // Index array is either [Top Cell, Bottom Cell] or [Left Cell, Right Cell] indexes
        // Value of -1 means there is no associated cell available
        match (x_diff, y_diff) {
            (-1, 0) => { // Rightward - use left end (x2,y2) as index coordinate
                if y2 == 0 {            // Bottom edge of the map
                    index[0] = -1;
                } else {
                    index[0] = self.xy_index(x1, y1-1);
                }

                if y2 == self.dim_y {   // Top edge of the map
                    index[1] = -1;
                } else {
                    index[1] = self.xy_index(x1, y1);
                }
            }, 
            (1, 0) => { // Rightward - use left end (x1,y1) as index coordinate
                if y1 == 0 {            // Bottom edge of the map
                    index[0] = -1;
                } else {
                    index[0] = self.xy_index(x2, y2 -1);
                }

                if y1 == self.dim_y {   // Top edge of map
                    index[1] = -1;
                } else {
                    index[1] = self.xy_index(x2, y2);
                }
            }, 
            (0, -1) => { // Downward - use bottom end (x2,y2) as index coordinate
                if x2 == 0 {            // Left edge of the map
                    index[0] = -1;
                } else {
                    index[0] = self.xy_index(x2-1, y1);
                }

                if x2 == self.dim_x {   // Right edge of the map
                    index[1] = -1;
                } else {
                    index[1] = self.xy_index(x2, y1);
                }
            },
            (0, 1) => { // Upward - use bottom end (x1, y1) as index coordinate 
                if x1 == 0 {            // Left Edge of map
                    index[0] = -1;
                } else {
                    index[0] = self.xy_index(x1-1, y2);
                }

                if x1 == self.dim_x {   // Right edge of map
                    index[1] = -1;
                } else {
                    index[1] = self.xy_index(x1, y2);
                }
            }, 
            _ => {
                return Err(String::from("Unknown Match case in MapGrid"))
            }
        };
        Ok(index)
    }

    // Given a line of 2 points, add 'walls' to the relavant grid entries
    pub fn add_walls(&mut self, x1:i32, y1:i32, x2:i32, y2:i32){
        // Fetch the grid index values - use that to figure out which walls were changed
        if let Ok(grids) = self.grid_index(x1, y1, x2, y2){
            // Since the grid_index function returned fine, we don't need to validate
            let x_diff = x1 - x2;
            let y_diff = y1 - y2;
            // println!("{}{}, {}{}",x_diff, y_diff, grids[0], grids[1]);
            
            match (x_diff, y_diff) {
                (-1, 0) | (1,0) => {    // Horizontal wall - bottom cell/top cell format
                    if grids[0] != -1 {
                        // Bottom cell is valid, update 'Top Wall'
                        self.tiles[grids[0] as usize].walls[UP] = true;
                    }
                    if grids[1] != -1 {
                        // Top cell is valid, update 'Bottom Wall'
                        self.tiles[grids[1] as usize].walls[DOWN] = true;
                    }
                }, 
                (0, -1) | (0, 1) => {    // Vertical wall - left cell/right cell format
                    if grids[0] != -1 {
                        // Left cell is valid, update 'Right Wall'
                        self.tiles[grids[0] as usize].walls[RIGHT] = true;
                    }
                    if grids[1] != -1 {
                        // Right cell is valid, update 'Left Wall'
                        self.tiles[grids[1] as usize].walls[LEFT] = true;
                    }
                }
                // (0,1) => {      // Downard - left cell, right cell

                // }
                _ => { } // Blank, since we've already handled it
            }

        }
        else {
            // Error handling for some issue with the walls provided
        }
        // println!("Walls have been added");
    }


    // Given a line of 2 points, remove 'walls' from the relevant grid entries - just reuse add_walls code above
    // pub fn remove_walls(&mut self, x1:i32, y1:i32, x2:i32, y2:i32){};

    // Validate if a 'movement' is possible given a coordinate and direction
    pub fn validate_move(&self, pos: &Position, dir: i32) -> Result<bool, String> {
        // TODO - replace dir with something more sensible - for now, just reference numpad position (2468)
        // println!("{}, {} | {}, {}", pos.x, pos.y, self.dim_y, self.dim_x);
        let cur_grid = self.tiles[self.xy_index(pos.x, pos.y) as usize];
        match dir {
            2 => { // Down
                if pos.y <= 0 || cur_grid.walls[DOWN] == true { Ok(false) }
                else { Ok(true) }  
            }
            4 => { // Left
                if pos.x <= 0 || cur_grid.walls[LEFT] == true { Ok(false) }
                else { Ok(true) }
            }
            6 => { // Right
                if pos.x >= (self.dim_x - 1) || cur_grid.walls[RIGHT] == true { Ok(false) }
                else { Ok(true) }
            }
            8 => { // Up
                if pos.y >= (self.dim_y - 1) || cur_grid.walls[UP] == true { Ok(false) }
                else { Ok(true) }
            }
            _ => Err("Invalid direction provided".to_string())
        }
        
    }
}

#[derive(Serialize,Deserialize)]
pub struct SavedMap {
    pub w: WallGrid,
    pub m: MapGrid,
}

// All functions in here are intended for the save/load logic
impl SavedMap{
    pub fn new(w: WallGrid, m: MapGrid) -> Self{
        SavedMap{w: w, m: m}
    }
    
    pub fn get_wg(&self) -> WallGrid {
        self.w.clone()
    }

    pub fn get_mg(&self) -> MapGrid {
        self.m.clone()
    }

    pub fn create_from_file(path: String) -> Self{
        // Attempt to open file from given path
        // If it panics, it means our default map is inacessible (and likely all other maps)
        let mut file = File::open(path).unwrap();
        // let mut data = String::new();
        // file.read_to_string(&mut data).unwrap();

        let saved_map = serde_json::from_reader(file);
        saved_map.unwrap()
    }
}

// Initialization function for the initial map grid and wall grid (Game startup)
//| Right now, they're treated as a resource rather than Entities, since only 1 map is loaded at any given time
//| This may change down the line if certain maps need to be cached, although those might just become their own resources
pub fn build_init(mut commands: Commands){
    let mg = MapGrid::new(8,8);
    let wg = WallGrid::new(8,8);
    commands.insert_resource(mg);
    commands.insert_resource(wg);
}

// Initialization function - Loads from file rather than a blank map
pub fn build_from_file(mut commands: Commands, path: &str){
    // File will have a SavedMap json - load into that, then load in the resources from it
    // Just calls insert_resource since that overwrites the resource cleanly - May tweak later if we need to cache previous resource.
    let map_data = SavedMap::create_from_file(path.to_string());
    
    // Use our clone functions so we can let insert_resource own the structs
    commands.insert_resource(map_data.get_mg());
    commands.insert_resource(map_data.get_wg());

}

// Update function to replace the resource - needs a ResMut of the resources
// Just overwrite the resource, unless we need to cache the previous one first

// Helper function to convert from floating point coordinate to pixel it's part of
pub fn coord_to_grid(x: f32, y: f32) -> (i32, i32) {
    // Just use floor function to truncate floating point and return the X/Y values
    (x.floor() as i32, y.floor() as i32)
}

// Helper function to convert from floating point coordinate to nearest grid intersects, and the distance
pub fn coord_to_grid_wall(x: f32, y: f32) -> (i32, i32, f32) {
    // Round to the nearest integer, then use lazy method (total of differences, rather than true-line)
    // TODO - test this function, might need to forcibly convert some values
    let dist = (x - x.round()).abs() + (y - y.round()).abs();
    (x.round() as i32, y.round() as i32, dist)
}



// This section is specifically for the Camera object that exclusively handles the minimap.
// Debating moving this into a camera.rs or render.rs file to indicate it's part of the rendering pipelines?
#[derive(Component, Copy, Clone)]
pub struct MinimapCamera{
    mode: MinimapMode,
}

// Setup function for creating the minimap's viewport camera - defaults to top right corner
// Only renders things that are tagged with RenderLayer layer 2 components
pub fn minimap_camera_setup(mut commands: Commands, window: Query<&Window>){
    let window = window.single();

    let phys_size = 128;
    println!("{}, {}", window.width(), window.height());
    let minimap_pos = UVec2::new((window.width() - phys_size as f32) as u32,0);

    
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 2,
                viewport: Some(Viewport {
                    physical_position: minimap_pos,
                    physical_size: UVec2::new(phys_size,phys_size),
                    ..default()
                }),
                ..default()
            },
            // V12 NOTE - In version 13, the clear_color is moved to the Camera itself
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..default()
        },
        MinimapCamera {mode: MinimapMode::Small},
        RenderLayers::layer(2),
    ));

    // Spawn a random sprite that takes up the full space for testing the rendering
    commands.spawn((SpriteBundle{
        sprite: Sprite { color: Color::TURQUOISE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
        visibility: Visibility::Visible,
        transform: Transform {
            translation: Vec2::new(-32., -32.).extend(0.0),
            scale: Vec3::new(64., 64., 0.),
            ..default()
        },
        ..Default::default()
    }, 
    RenderLayers::layer(2),
    ));
    // println!("Viewport camera set up, along with dummy sprite");
}

// Toggles visibility of the minimap camera
pub fn minimap_camera_vis_togle(mut commands: Commands, mut m_cam: Query<&mut Camera, With<MinimapCamera>>){
    let mut camera = m_cam.single_mut();
    camera.is_active = !camera.is_active;
}

// Toggles style of the minimap (Corner, opaque overlay, fullscreen, off, etc...)
pub fn minimap_camera_style_toggle(
    mut commands: Commands, 
    mut m_cam: Query<&mut Camera, With<MinimapCamera>>,
    mut m_mode: Query<&mut MinimapCamera>,
    mut mg: ResMut<MapGrid>,
){
    let mut camera = m_cam.single_mut();
    let mut viewport = camera.viewport.as_mut().unwrap();
    let mut mc = m_mode.single_mut();
    // Currently hardcoded to go in a loop - Small, Medium, Large, None, back to Small
    // TODO - make sure this works, or do I need to do m_mode.single_mut() and edit from that?
    match mc.mode {
        MinimapMode::Small => {
            mc.mode = MinimapMode::Medium;
            // Change zoom value - for now, it'll be hardcoded 8/16/24
            mg.zoom = 16.;
        }
        MinimapMode::Medium => {
            mc.mode = MinimapMode::Large;
            mg.zoom = 24.;
        }
        MinimapMode::Large => {
            mc.mode = MinimapMode::None;
            // Just toggle visibility of the map (Or set zoom to 0) - 
        }
        _ => {  // Case of it being None (Or an invalid mode - set to Small)
            mc.mode = MinimapMode::Small;
            mg.zoom = 8.;
            // Adjust the viewport size
            // Adjust the minimap multipliers (Where are these stored? In the MG resource?)
        }
    }
}

// Centralizing the const vars that components are using, mainly because some will likely become dynamic
// MapGrid now stores zoom - this const is for a 'default' zoom level on new map grids
const ZOOM_LEVEL: f32 = 16.0; // Number of pixels a tile occupies by default
const ZL: f32 = ZOOM_LEVEL;

// Shorthand for accessing the Tile walls direction more clearly
const DOWN: usize = 0;
const LEFT: usize = 1;
const UP: usize = 2;
const RIGHT: usize = 3;


const VIEWPORT_SMALL: u32 = 128;
const VIEWPORT_FULL: u32 = 512;
const VIEWPORT_MEDIUM: u32 = 256;

#[derive(Copy, Clone)]
pub enum MinimapMode {
    Small,
    Medium,
    Large,
    None
}