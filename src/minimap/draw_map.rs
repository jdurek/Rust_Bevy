/* 
    Contains most functions that trigger Bevy's commands.spawn() relative to maps 
    Also handles some setup needed for the UI drawing
*/

use crate::minimap::*;


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