/* 
    Contains most functions that trigger Bevy's commands.spawn() relative to maps 
    Also handles some setup needed for the UI drawing
*/

use bevy::prelude::*;
use bevy::ecs::world;

use crate::{components::*, minimap::*, resources::*, };

// Component is only for query lookup
#[derive(Component)]
pub struct DragLine;

#[derive(Component)]
pub struct MapCellSprite;
#[derive(Component)]
pub struct MapWallSprite;
#[derive(Component)]
pub struct MapCellIcon;

#[derive(Component)]
pub struct SelectedOption;

// Renders only the grid
pub fn draw_grid(mut commands: Commands, mg: Res<MapGrid>) {
    // TODO - change the shifts to account for movement of the map - this assumes 0,0 is always bottom left
    // For now - I've added mg.zoom * 0 at the end - just apply the coordinate shift (X/Y-specific to it (new resource?))
    let bl_x_shift = mg.dim_x as f32 * mg.zoom / 2. - mg.zoom/2. + mg.zoom * 0.;
    let bl_y_shift = mg.dim_y as f32 * mg.zoom / 2. - mg.zoom/2. + mg.zoom * 0.;
    for y in 0..mg.dim_y {
        for x in 0..mg.dim_x {
            let index = coord_to_grid(x as f32, y as f32);
            // Using the tile at index, render on the map - 
            // For now, all tiles will render the same regardless of type, add a match statement later
            commands.spawn((SpriteBundle{
                sprite: Sprite { color: Color::TURQUOISE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                visibility: Visibility::Visible,
                transform: Transform {
                    translation: Vec2::new(x as f32 * mg.zoom - bl_x_shift, y as f32 * mg.zoom - bl_y_shift).extend(0.0),
                    scale: Vec3::new(mg.zoom -1., mg.zoom -1., 0.),
                    ..default()
                },
                ..Default::default()
            }, 
            MapCellSprite, 
            RenderLayers::layer(2),
            ));
        }
    }
}

// Renders only the walls, this goes on top of the map's grid (Or under it?)
pub fn draw_wall(mut commands: Commands, mw: Res<WallGrid>, mg: Res<MapGrid>){
    let bl_x_shift = mg.dim_x as f32 * mg.zoom / 2. - mg.zoom/2. + mg.zoom * 0.;
    let bl_y_shift = mg.dim_y as f32 * mg.zoom / 2. - mg.zoom/2. + mg.zoom * 0.;
    // Iterating over wall-grid means we flip between horizontal and vertical walls
    for x in 0..mw.dim_x + 1 {
        for h in 0..mw.dim_x{
            //Index will be x + h*(x+y+1)
            // Check if the wall is enabled or not
            let index =(h+x*(mw.dim_x+mw.dim_y+1)) as usize;
            if mw.walls[(h+x*(mw.dim_x+mw.dim_y+1)) as usize].pres == true {
                commands.spawn((SpriteBundle{
                    sprite: Sprite { color: Color::ANTIQUE_WHITE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                    visibility: Visibility::Visible,
                    transform: Transform {
                        translation: Vec2::new(h as f32 * mg.zoom - bl_x_shift, x as f32 * mg.zoom - mg.zoom/2. - bl_y_shift).extend(0.0),
                        scale: Vec3::new(mg.zoom, 1.5, 1.),
                        ..default()
                    },
                    ..Default::default()
                }, 
                MapWallSprite, 
                RenderLayers::layer(2),
                ));
            }
        }
    }
    for y in 0..mw.dim_y {
        for v in 0..mw.dim_y + 1 {
            // Index will be dim_x + v + y*(x+y+1)
            if mw.walls[(mw.dim_x + v + y*(mw.dim_x+mw.dim_y+1)) as usize].pres == true {
                commands.spawn((SpriteBundle{
                    sprite: Sprite { color: Color::ANTIQUE_WHITE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                    visibility: Visibility::Visible,
                    transform: Transform {
                        translation: Vec2::new(v as f32 * mg.zoom - mg.zoom/2. - bl_x_shift, y as f32 * mg.zoom - bl_y_shift).extend(0.0),
                        scale: Vec3::new(1.5, mg.zoom, 1.),
                        ..default()
                    },
                    ..Default::default()
                }, 
                RenderLayers::layer(2),
                ));
            }
        }
    }
    
}


// Simple function to handle moving out of the render state
pub fn render_map(
    mut commands: Commands,
    mut next_state: ResMut<NextState<MapBuildState>>,
) {
    next_state.set(MapBuildState::Drawing);
}


// Generic despawner for any component marker we come up with later
// TODO - Transform this to be more map-specific (Cleanup, caching maps for faster load, etc...)
pub fn despawn_system<M: Component>(mut commands: Commands, query: Query<Entity, With<M>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn();
    });
}


