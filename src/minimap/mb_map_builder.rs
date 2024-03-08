// Contains the main logic needed for mouse interactivity on the MapBuilder's GUI

use bevy::prelude::*;
use bevy::ecs::world;
use bracket_lib::color::BLACK;

use crate::{components::*, minimap::*, resources::*, };


// Allows us to create a wall by left-clicking near a corner point and dragging to another valid corner
// Includes logic to 'snap' the line so we can continue to draw 
pub fn mouse_wall_gui(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>, 
    // Reference to our Camera so we can translate to world coordinates
    map_cam: Query<(&Camera, &GlobalTransform)>,  //TODO - adjust when more cameras are added
    mut draw_line: Query<(&DragLine, &mut Transform, &mut Position, Entity)>,
    mut next_state: ResMut<NextState<MapBuildState>>,
    mut mw: ResMut<WallGrid>,
    mut mg: ResMut<MapGrid>,
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
        let rounded_positions = (world_position.x.round() + mg.zoom/2., world_position.y.round() + mg.zoom/2.);
        // These two are localized X/Y values within a pixel (0 to ZOOM_LEVEL - 1)
        let loc_x = rounded_positions.0.rem_euclid(mg.zoom);
        let loc_y = rounded_positions.1.rem_euclid(mg.zoom);

        let mut start_x: f32 = 0.;
        let mut start_y: f32 = 0.;

        // Display mouse coordinates VIA a textbox 
        let pos_str = format!("({loc_x}, {loc_y})");

        // TODO - Check if the sprites exists or not
        // If they do, we're probably in the pressed or just_released states - handle using the sprite we queried
        // If they don't, handle accordingly 

        // Left-click has been pressed - spawn in 2 sprites (Line itself, and the coords)
        if mouse.just_pressed(MouseButton::Left) {
            // Try to find nearest whole coordinate within reason (EG if a pixel is 16 tiles, 4 tiles near the corner)
            if (loc_x / mg.zoom < 0.2 || loc_x / mg.zoom > 0.8) &&
               (loc_y / mg.zoom < 0.2 || loc_y / mg.zoom > 0.8) 
            {
                println!("Close enough to a corner!");
                // TODO - snap the start_x and _y values to the corner itself (Multiple of ZOOM_LEVEL)
                // start_x = world_position.x;
                // start_y = world_position.y;

                start_x = ((world_position.x + mg.zoom /2.) / mg.zoom).round() * mg.zoom - mg.zoom / 2.;
                start_y = ((world_position.y + mg.zoom /2.) / mg.zoom).round() * mg.zoom - mg.zoom / 2.;

                println!("Position: {},{}", start_x, start_y);

                // TODO - add a component to these bundles specifically for the query
                // commands.spawn(
                //     TextBundle::from_section(
                //         pos_str,
                //         TextStyle {
                //             ..Default::default()
                //             },
                //         )
                //         .with_text_alignment(TextAlignment::Right)
                //         .with_style(Style {
                //             position_type: PositionType::Absolute,
                //             bottom: Val::Px(5.0),
                //             right: Val::Px(5.0),
                //             ..default()
                //         })
                //         .with_background_color(Color::BLACK)
                // );
    
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
                }, 
                DragLine,
                Position{x:start_x as i32, y:start_y as i32, z:0},
                ));
                
                
            }
            
            
        }

        // Hold Left Click - reposition the wall to end where the mouse currently is
        if mouse.pressed(MouseButton::Left) {
            // Fetches the sprite to edit (If it exists - it may have not been created)
            for (_drag, mut transf, mut pos, _ent) in draw_line.iter_mut(){
                // println!("Dumping transform data... {}, {}, {}", transf.translation, transf.scale, transf.rotation)
                
                // Updates 2 values in the Transform section of the sprite bundle - Scale and Rotation
                // Needs to use the Translation values to fetch the 'point of origin' for some computations
                let norm_pts = (world_position.x - pos.x as f32, world_position.y - pos.y as f32);
                let theta = norm_pts.1.atan2(norm_pts.0);
                // Compute dist value - we can assume it will always be a right angle triangle
                let dist = ((world_position.x - pos.x as f32).abs().powi(2) + (world_position.y - pos.y as f32).abs().powi(2)).sqrt();

                // Update our Transform values
                // TODO - adjust the sensitivity of the grid-snapping
                if dist > mg.zoom * 0.90 && dist < mg.zoom * 1.2 {
                    // Line is long enough that it could snap onto a valid point
                    
                    if (loc_x / mg.zoom < 0.1 || loc_x / mg.zoom > 0.9) &&
                       (loc_y / mg.zoom < 0.1 || loc_y / mg.zoom > 0.9) {
                        // print!("Snapping line");

                        let old_x = pos.x as f32;
                        let old_y = pos.y as f32;

                        
                        pos.x = (((world_position.x + ZL /2.) / ZL).round() * ZL - ZL / 2.) as i32;
                        pos.y = (((world_position.y + ZL /2.) / ZL).round() * ZL - ZL / 2.) as i32;

                        // Write the new wall to the map (Convert the coordinates)
                        
                        mg.add_walls(
                            ((old_x + ZL /2.) / ZL) as i32,
                            ((old_y + ZL /2.) / ZL) as i32,
                            ((pos.x as f32 + ZL /2.) / ZL) as i32,
                            ((pos.y as f32 + ZL /2.) / ZL) as i32,
                        );
                        
                        // Man, flipping between int/float is fun
                        mw.add_wall(
                            ((old_x + ZL /2.) / ZL) as i32,
                            ((old_y + ZL /2.) / ZL) as i32,
                            ((pos.x as f32 + ZL /2.) / ZL) as i32,
                            ((pos.y as f32 + ZL /2.) / ZL) as i32,
                        );

                        // Trigger RenderMap state so the wall gets shown
                        next_state.set(MapBuildState::RenderMap);
                    }
                }

                if dist < mg.zoom {
                    transf.scale.x = dist;
                    transf.translation.x = (pos.x as f32 + world_position.x)/ 2. ;
                    transf.translation.y = (pos.y as f32 + world_position.y)/ 2. ;
                } else {
                    transf.scale.x = mg.zoom;
                    // Force the translation to remain within fixed radius
                    transf.translation.x = pos.x as f32 + (theta.cos() * mg.zoom)/ 2. ;
                    transf.translation.y = pos.y as f32 + (theta.sin() * mg.zoom)/ 2. ;
                }
                transf.rotation = Quat::from_rotation_z(theta);
                
                
                

            }
            
        }
        
        // Right-click has been pressed - remove a wall if we meet conditions
        if mouse.just_pressed(MouseButton::Right) {
            // Currently unimplemented - 

            // Check to see if the mouse coordinate is within .2 of a line
            // If more than 1 valid wall is within distance of the click, uses the closest one
            // If a tie occurs, then it will not remove a wall. Tie has some leeway (.1) 

            // Currently hardcoded to accept all X/Y combinations nearish the line, except for near intersections
            if  (loc_x / mg.zoom < 0.2 && loc_x / mg.zoom > 0.05) || 
                (loc_x / mg.zoom > 0.8 && loc_x / mg.zoom < 0.95) ||
                (loc_y / mg.zoom < 0.2 && loc_y / mg.zoom > 0.05) || 
                (loc_y / mg.zoom > 0.8 && loc_y / mg.zoom < 0.95)
            {
                // Location is close enough to a valid line, assume the line is floor, ciel depending on the axis we're on
                // Determine if we're on X or Y axis wall (Or both)

                

            }

            
        }

    }   

    // Handle despawning the line entity - 
    if mouse.just_released(MouseButton::Left) {
        // Clean up the display 
        for a in draw_line.iter(){
            // Despawn the entity - should only be the one anyway
            commands.entity(a.3).despawn();
        }

        // Check to see if we missed a wall creation (Snap-in-place logic handled most of it)

        
    }

    
}
