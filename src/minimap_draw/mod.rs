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
}

#[derive(Resource, Serialize, Deserialize)]
pub struct MapGrid {

}