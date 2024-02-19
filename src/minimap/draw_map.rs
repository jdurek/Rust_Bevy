/* 
    Contains most functions that trigger Bevy's commands.spawn() relative to maps 
    Also handles some setup needed for the UI drawing
*/

use bevy::ecs::world;

use crate::minimap::*;

// Component is only for query lookup
#[derive(Component)]
pub struct DragLine;
#[derive(Component)]
pub struct LineCoords;

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

// Experimenting with drawing a wall - starts by finding initial coordinate - 
pub fn mouse_wall_gui(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>, 
    // Reference to our Camera so we can translate to world coordinates
    map_cam: Query<(&Camera, &GlobalTransform)>,  //TODO - adjust when more cameras are added
    mut draw_line: Query<(&DragLine, &mut Transform, Entity)>,
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

        let mut start_x: f32 = 0.;
        let mut start_y: f32 = 0.;

        // Display mouse coordinates VIA a textbox 
        let pos_str = format!("({loc_x}, {loc_y})");

        // TODO - Check if the sprites exists or not
        // If they do, we're probably in the pressed or just_released states - handle using the sprite we queried
        // If they don't, handle accordingly 

        // Mouse has been pressed - spawn in 2 sprites (Line itself, and the coords)
        if mouse.just_pressed(MouseButton::Left) {
            // Try to find nearest whole coordinate within reason (EG if a pixel is 16 tiles, 4 tiles near the corner)
            if (loc_x / ZOOM_LEVEL < 0.2 || loc_x / ZOOM_LEVEL > 0.8) &&
               (loc_y / ZOOM_LEVEL < 0.2 || loc_y / ZOOM_LEVEL > 0.8) 
            {
                println!("Close enough to a corner!");
                start_x = world_position.x;
                start_y = world_position.y;

                // TODO - add a component to these bundles specifically for the query
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
                        .with_background_color(Color::BLACK)
                );
    
                // Spawn a 'line' with a default rotation and length of 0
                // Snaps the start of the line to the nearest valid corner
                                
                commands.spawn((SpriteBundle{
                    sprite: Sprite { color: Color::ANTIQUE_WHITE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                    visibility: Visibility::Visible,
                    transform: Transform {
                        translation: Vec2::new(start_x.round() as f32, start_y.round() as f32).extend(5.0),
                        scale: Vec3::new(0., 1.5, 1.),
                        rotation: Quat::from_rotation_z(0.),
                        ..default()
                    },
                    ..Default::default()
                }, DragLine));
                
                
            }
            
            println!("Position: {},{}", loc_x, loc_y);
        }

        if mouse.pressed(MouseButton::Left) {
            // Fetches the sprite to edit (If it exists - it may have not been created)
            for (drag, mut transf, ent) in draw_line.iter_mut(){
                // println!("Dumping transform data... {}, {}, {}", transf.translation, transf.scale, transf.rotation)
                
                // Updates 2 values in the Transform section of the sprite bundle - Scale and Rotation
                // Needs to use the Translation values to fetch the 'point of origin' for some computations
                let norm_pts = (world_position.x - transf.translation.x, world_position.y - transf.translation.y);
                let theta = norm_pts.1.atan2(norm_pts.0);
                // Compute dist value - we can assume it will always be a right angle triangle
                let dist = ((world_position.x - transf.translation.x).abs().powi(2) + (world_position.y - transf.translation.y).abs().powi(2)).sqrt();

                // Update our Transform values
                transf.scale.x = dist;
                transf.rotation = Quat::from_rotation_z(theta);

            }


            // TODO - replace with something like a 'score' board where we only edit a field instead of making a new sprite
            
        }
        // if mouse.just_pressed(MouseButton::Right) {
        //     // Try to find the nearest 'wall' - just check if we're near an int and take other 2 values
        //     // Currently unimplemented for now
        // }

    }   

    // Handle despawning the line entity we just drew - 
    if mouse.just_released(MouseButton::Left) {
        // Clean up the display 
        for a in draw_line.iter(){
            // Despawn the entity - should only be the one anyway
            commands.entity(a.2).despawn();
        }

        // Update WallGrid (Which is a resource, add to our function's queries) - add_wall

        
    }

    
}