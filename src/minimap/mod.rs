use bevy::window::PrimaryWindow;
use bevy::prelude::*;
use serde::*;
// use crate::components::*;

pub mod draw_map;
pub use draw_map::*;

#[derive(Component)]
pub struct TileStruct;

// Alternate tile type - this one holds wall data as well for faster checks
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub walls: (bool,bool,bool,bool), // Tuple representing the 4 directions (NSEW) and if we can move in those directions
    // pub kind: bool,
    // TODO - Turn kind into an enum for more features later
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Wall {
    pub vis: bool,  // Is the wall visible yet (EG, render it or not)
    pub pres: bool, // Is there a wall?
    // pub kind: bool, // Flag that indicates if it's a normal wall or something else
    // TODO - replace the bool above with an enum when adding more features
}

#[derive(Resource, Serialize, Deserialize)]
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
            walls: vec![Wall {vis:false, pres:true}; ((width+1) * height + (height+1) * width) as usize],
        }

    }

    // Helper function to determine 'where' the wall is relative to a line provided
    // Expects the coordinates to be a distance of 1, and no further
    pub fn wall_index(&self, x1:i32, y1:i32, x2:i32, y2:i32) -> Result<usize, String> {
        // Error handling: Invalid Line, Dest out of Bounds, 

        if x1-x2 + y1-y2 != 1 {
            // Distance is not equal to 1 - cannot create a new line
            return Err(String::from("Invalid Line"))
        }
        if x1 > self.dim_x + 1 || x2 > self.dim_x + 1 || x1 < 0 || x2 < 0 
         || y1 > self.dim_y + 1 || y2 > self.dim_y + 1 || y1 < 0 || y2 < 0 {
            // Start or End coordinate is outside of our map's vector
            return Err(String::from("Coordinate out of bounds"))
        }

        // Line is valid - translate to the Vector positions
        let x_diff = x1 - x2;
        let y_diff = y1 - y2;
        let mut index = 0;
        match (x_diff, y_diff) {
            // Odd Rows 
            (-1, 0) => { // Leftward
                 // Take Y coordinate, multiply by (x+y+1), add X - 1
                index = (2 * self.dim_x + 1) * y1 + (x1 - 1);
            }, 
            (1, 0) => { // Rightward
                // Take Y coordinate, multiply by (x+y+1), add current X
                index = (2 * self.dim_x + 1) * y1 + (x1);
            }, 
            // Even Rows
            (0, -1) => { // Downward
                index = (2 * self.dim_x + 1) * y1 - 1 - self.dim_y - x1;

            },
            (0, 1) => { // Upward
                index = (2 * self.dim_x + 1) * y1 + self.dim_y + x1;
 
            }, 
            _ => {
                return Err(String::from("Unknown Match case in WallGrid"))
            }
        };
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

#[derive(Resource, Serialize, Deserialize)]
pub struct MapGrid {
    pub tiles: Vec<Tile>,
    pub dim_x: i32,
    pub dim_y: i32,
}

impl MapGrid {
    pub fn new(width: i32, height:i32) -> MapGrid {
        MapGrid {
            dim_x: width,
            dim_y: height,
            tiles: vec![Tile {walls:(false,false,false,false),}; (width*height) as usize]
        }
    }

    // Translate (x,y) coordinate into Vector index directly
    // Potential TODO - add error handling so we can just call this directly?
    pub fn xy_index(&self, x:i32, y:i32) -> i32 {
        (y * self.dim_x ) + x
    }

    // Given a line of 2 points, figure out which 1-2 grids are involved
    pub fn grid_index(&self, x1:i32, y1:i32, x2:i32, y2:i32) -> Result<[i32;2], String> {

        // Validate the line length
        if x1-x2 + y1-y2 != 1 {
            // Distance is not equal to 1 - cannot use this line
            return Err(String::from("Invalid Line"))
        }
        if x1 > self.dim_x + 1 || x2 > self.dim_x + 1 || x1 < 0 || x2 < 0 
         || y1 > self.dim_y + 1 || y2 > self.dim_y + 1 || y1 < 0 || y2 < 0 {
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
            (-1, 0) => { // Leftward - use left end (x2,y2) as index coordinate
                if y2 == 0 {            // Bottom edge of the map
                    index[0] = -1;
                } else {
                    index[0] = self.xy_index(x2, y2-1);
                }

                if y2 == self.dim_y {   // Top edge of the map
                    index[1] = -1;
                } else {
                    index[1] = self.xy_index(x2, y2);
                }
            }, 
            (1, 0) => { // Rightward - use right end (x1,y1) as index coordinate
                if y1 == 0 {            // Bottom edge of the map
                    index[0] = -1;
                } else {
                    index[0] = self.xy_index(x1, y1 -1);
                }

                if y1 == self.dim_y {   // Top edge of map
                    index[1] = -1;
                } else {
                    index[1] = self.xy_index(x1, y1);
                }
            }, 
            (0, -1) => { // Downward - use bottom end (x2,y2) as index coordinate
                if x2 == 0 {            // Left edge of the map
                    index[0] = -1;
                } else {
                    index[0] = self.xy_index(x2-1, y2);
                }

                if x2 == self.dim_x {   // Right edge of the map
                    index[1] = -1;
                } else {
                    index[1] = self.xy_index(x2, y2);
                }
            },
            (0, 1) => { // Upward - use bottom end (x1, y1) as index coordinate 
                if x1 == 0 {            // Left Edge of map
                    index[0] = -1;
                } else {
                    index[0] = self.xy_index(x1-1, y1);
                }

                if x1 == self.dim_x {   // Right edge of map
                    index[1] = -1;
                } else {
                    index[1] = self.xy_index(x1, y1);
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
            match (x_diff, y_diff) {
                (-1, 0) => {    // Leftward - bottom cell, top cell
                    // TODO - Insert new walls tuple into Tile Entity for the 4 cases here
                    // This will probably run into mutability issues, but I don't want to rewrite the tile every time...
                },
                (1,0) => {      // Rightward - bottom cell, top cell

                }
                (0, -1) => {    // Upward - left cell, right cell

                }
                (0,1) => {      // Downard - left cell, right cell

                }
                _ => { } // Blank, since we've already handled it
            }

        }
        else {
            // Error handling for some issue with the walls provided
        }
        
    }

    // Given a line of 2 points, remove 'walls' from the relevant grid entries
}

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



// Centralizing the const vars that components are using, mainly because some will likely become dynamic
const ZOOM_LEVEL: f32 = 16.0; // Number of pixels a tile occupies



