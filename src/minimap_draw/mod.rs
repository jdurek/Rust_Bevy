use crate::prelude::*;
// use crate::prelude::Map;

/* 
    This module is more for me to experiment with 'drawing' a map, and potentially making a GUI that'd let me draw and save maps quickly into the map format
    The drawable mini-map is represented by 2 vectors, 1 for the tiles, and 1 for the walls. 
    The wall vector is not needed for exporting, as we can rebuild from only the tiles - but the walls should make it easier to visualize code-wise.
*/

#[derive(Component)]
pub struct TileStruct;

// Alternate tile type - this one holds wall data as well for faster checks
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub walls: (bool,bool,bool,bool), // Tuple representing the 4 directions (NSEW) and if we can move in those directions
    // pub kind: bool,
    // TODO - Turn kind into an enum for more features later
}

#[derive(Resource, Serialize, Deserialize)]
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

    // Given a wall index, and 2 coordinates - update both WallGrid and MapGrid
    // If wall already exists - either just update the vals, or raise.
    pub fn add_wall(&self, m_grid: MapGrid, x1:i32, y1:i32, x2:i32, y2:i32){
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

    // Translate (x,y) coordinate into Vector index
    pub fn xy_index(&self, x:i32, y:i32) -> usize {
        (y as usize * self.dim_x as usize) + x as usize
    }
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



/* Entity that points to map/wall grids, and stores additional info? */

/* All code below this section will pertain to drawing the map/walls if possible */
const ZOOM_LEVEL: f32 = 15.0; // Check if this conflicts with map.rs const

// Draws only the grid
pub fn draw_grid(mut commands: Commands, mg: Res<MapGrid>) {
    for y in 0..mg.dim_y {
        for x in 0..mg.dim_x {
            let index = coord_to_grid(x as f32, y as f32);
            // Using the tile at index, render on the map - 
            // For now, all tiles will render the same regardless of type, add a match statement later
            commands.spawn(SpriteBundle{
                sprite: Sprite { color: Color::TURQUOISE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                visibility: Visibility::Visible,
                transform: Transform {
                    translation: Vec2::new(x as f32 * ZOOM_LEVEL, y as f32 * ZOOM_LEVEL).extend(0.0),
                    scale: Vec3::new(ZOOM_LEVEL-1.0, ZOOM_LEVEL-1.0, ZOOM_LEVEL),
                    ..default()
                },
                ..Default::default()
            })
            ;
        }
    }
}

// Draws only the walls, this goes on top of the map's grid (Or under it?)
pub fn draw_wall(mut commands: Commands, mw: Res<WallGrid>){

}

// Builds a grid and walls
pub fn build_init(mut commands: Commands){
    let mg = MapGrid::new(16,16);
    // let wg = WallGrid::new();
    commands.insert_resource(mg);
    // commands.insert_resource(wg);
}