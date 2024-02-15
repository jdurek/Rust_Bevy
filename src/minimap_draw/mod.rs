use bevy::window::PrimaryWindow;

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
const ZOOM_LEVEL: f32 = 16.0; // Check if this conflicts with map.rs const

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
                    scale: Vec3::new(ZOOM_LEVEL-1., ZOOM_LEVEL-1., 0.),
                    ..default()
                },
                ..Default::default()
            });
        }
    }
}

// Draws only the walls, this goes on top of the map's grid (Or under it?)
pub fn draw_wall(mut commands: Commands, mw: Res<WallGrid>){
    // Iterating over wall-grid means we flip between horizontal and vertical walls
    for x in 0..mw.dim_x + 1 {
        for h in 0..mw.dim_x{
            //Index will be x + h*(x+y+1)
            // Check if the wall is enabled or not
            if mw.walls[(x+h*(mw.dim_x+mw.dim_y+1)) as usize].pres == true {
                commands.spawn(SpriteBundle{
                    sprite: Sprite { color: Color::ANTIQUE_WHITE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                    visibility: Visibility::Visible,
                    transform: Transform {
                        translation: Vec2::new(h as f32 * ZOOM_LEVEL, x as f32 * ZOOM_LEVEL - ZOOM_LEVEL/2.).extend(0.0),
                        scale: Vec3::new(ZOOM_LEVEL, 1.5, 1.),
                        ..default()
                    },
                    ..Default::default()
                });
            }
        }
    }
    for y in 0..mw.dim_y {
        for v in 0..mw.dim_y + 1 {
            // Index will be dim_x + v + y*(x+y+1)
            if mw.walls[(mw.dim_x + v + y*(mw.dim_x+mw.dim_y+1)) as usize].pres == true {
                commands.spawn(SpriteBundle{
                    sprite: Sprite { color: Color::ANTIQUE_WHITE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                    visibility: Visibility::Visible,
                    transform: Transform {
                        translation: Vec2::new(v as f32 * ZOOM_LEVEL - ZOOM_LEVEL/2., y as f32 * ZOOM_LEVEL).extend(0.0),
                        scale: Vec3::new(1.5, ZOOM_LEVEL, 1.),
                        ..default()
                    },
                    ..Default::default()
                });
            }
        }
    }
    
}

// Builds a grid and walls
pub fn build_init(mut commands: Commands){
    let mg = MapGrid::new(16,16);
    let wg = WallGrid::new(16,16);
    commands.insert_resource(mg);
    commands.insert_resource(wg);
}

// TODO - add this coordinate system transformation (Camera to world coords)
// https://bevy-cheatbook.github.io/cookbook/cursor2world.html


// Experimenting with drawing a wall - 
pub fn mouse_wall_gui(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>, 
    // Reference to our Camera so we can translate to world coordinates
    map_cam: Query<(&Camera, &GlobalTransform)>,  //TODO - adjust when more cameras are added
) {
    // Fetch camera information
    let (camera, camera_transform) = map_cam.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // Cursor is defined at world_position, a x/y pair
        // Match these to the size of our ZOOM_LEVEL - 
        // NOTE - true 0,0 is the center of the 1st sprite, so we shift all values over by half a sprite for 0,0 to be bottom left corner of sprite
        let rounded_positions = (world_position.x.round() + ZOOM_LEVEL/2., world_position.y.round() + ZOOM_LEVEL/2.);
        // These two are localized X/Y values within a pixel (0 to ZOOM_LEVEL - 1)
        let loc_x = rounded_positions.0.rem_euclid(ZOOM_LEVEL);
        let loc_y = rounded_positions.1.rem_euclid(ZOOM_LEVEL);

        if mouse.just_pressed(MouseButton::Left) {
            // Try to find nearest whole coordinate within reason (EG if a pixel is 16 tiles, 4 tiles near the corner)
            if (loc_x / ZOOM_LEVEL < 0.2 || loc_x / ZOOM_LEVEL > 0.8) &&
               (loc_y / ZOOM_LEVEL < 0.2 || loc_y / ZOOM_LEVEL > 0.8) 
            {
                println!("Close enough to a corner!");
            }
            
            println!("Position: {},{}", loc_x, loc_y);
        }

        if mouse.pressed(MouseButton::Left) {
            // Update the sprite to follow the current position
        }
        // if mouse.just_pressed(MouseButton::Right) {
        //     // Try to find the nearest 'wall' - just check if we're near an int and take other 2 values
        //     // Currently unimplemented for now
        // }

        // Display mouse coordinates VIA a textbox 
        let pos_str = format!("({loc_x}, {loc_y})");
        
        commands.spawn(
            TextBundle::from_section(
                pos_str,
                TextStyle {
                    ..Default::default()
                },
            )
            .with_text_alignment(TextAlignment::Right)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            })
        );


    }

    if mouse.just_released(MouseButton::Left) {

        // Figure out if the new 'wall' is valid - wall_index

        // Update WallGrid (Which is a resource, add to our function's queries) - add_wall

        
    }

    
}