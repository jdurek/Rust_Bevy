use crate::prelude::*;
use crate::prelude::Map;

/* 
    This module is more for me to experiment with 'drawing' a map, and potentially making a GUI that'd let me draw and save maps quickly into the map format
    The drawable mini-map is represented by 2 vectors, 1 for the tiles, and 1 for the walls. 
    The wall vector is not needed for exporting, as we can rebuild from only the tiles - but the walls should make it easier to visualize code-wise.
*/

#[derive(Resource, Serialize, Deserialize)]
pub struct Wall {
    pub vis: bool,  // Is the wall visible yet (EG, render it or not)
    pub pres: bool, // Is there a wall?
}

#[derive(Resource, Serialize, Deserialize)]
pub struct WallGrid {
    pub walls: Vec<Wall>,
    pub dim_x: i32,
    pub dim_y: i32
}

impl WallGrid {
    // Helper function to determine 'where' the wall is relative to a line provided
    // Expects the coordinates to be a distance of 1, and no further
    pub fn wall_index(&self, x1:i32, y1:i32, x2:i32, y2:i32) -> Result<usize, Err> {
        // Error handling: Invalid Line, Dest out of Bounds, 

        if(x1-x2 + y1-y2 != 1){
            // Distance is not equal to 1 - cannot create a new line
            return Err("Invalid Line")
        }
        if(x1 > dim_x + 1 | x2 > dim_x + 1 | x1 < 0 | x2 < 0 
         | y1 > dim_y + 1 | y2 > dim_y + 1| y1 < 0 | y2 < 0){
            // Start or End coordinate is outside of our map's vector
            return Err("Coordinate out of bounds")
        }

        // Line is valid - translate to the Vector positions
        let x_diff = x1 - x2;
        let y_diff = y1 - y2;
        let mut index = 0;
        match (x_diff, y_diff) {
            // Odd Rows 
            (-1, 0) => { // Leftward
                 // Take Y coordinate, multiply by (x+y+1), add X - 1
                index = (2 * dim_x + 1) * y1 + (x1 - 1);
            }, 
            (1, 0) => { // Rightward
                // Take Y coordinate, multiply by (x+y+1), add current X
                index = (2 * dim_x + 1) * y1 + (x1);
            }, 
            // Even Rows
            (0, -1) => { // Downward
                index = (2 * dim_x + 1) * y1 - 1 - dim_y - x1;

            },
            (0, 1) => { // Upward
                index = (2 * dim_x + 1) * y1 + dim_y + x1;
 
            }, 
        };

    }

    // Given a wall index, and 2 coordinates - update both WallGrid and MapGrid
    // If wall already exists - either just update the vals, or raise.
    pub fn add_wall(&self, &mGrid: MapGrid, x1:i32, y1:i32, x2:i32, y2:i32){
        // Unpack result of wall_index (If we get out of bounds, handle it)

        // Update entry in WallGrid

        // Determine which cells need to be updated (1-2 cells), and update those values
    }
    
    // Opposite of add_wall, but takes the same args
    // If wall didn't exist, can keep going without any issues
    pub fn remove_wall(){

    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct MapGrid {
    pub tiles: Vec<TileData>,
    pub dim_x: i32,
    pub dim_y: i32,
}

impl MapGrid {
    pub fn new(width: i32, height:i32) -> MapGrid {
        MapGrid {
            dim_x: width,
            dim_y: height,
            tile_data: vec![TileData {tile_type:TileType::Floor,}; (width*height) as usize]
        }
    }

    // Translate (x,y) coordinate into Vector index
    pub fn xy_index(&self, x:i32, y:i32) -> usize {
        (y as usize * self.dim_x as usize) + x as usize
    }
}

// Helper function to convert from floating point coordinate to pixel it's part of
pub fn coord_to_grid(x: f32, y: f32) -> (i32, i32) {
    // Just use floor function to truncate floating point and return the X/Y values
    (x.floor(), y.floor())
}

// Helper function to convert from floating point coordinate to nearest grid intersects, and the distance
pub fn coord_to_grid(x: f32, y: f32) -> (i32, i32, f32) {
    // Round to the nearest integer, then use lazy method (total of differences, rather than true-line)
    // TODO - test this function, might need to forcibly convert some values
    let dist = (x - x.round()).abs() + (y - y.round()).abs();
    (x.round(), y.round(), dist)
}
