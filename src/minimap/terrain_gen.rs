// Experimental feature - Uses minimap details to generate 2d terrain behavior
// This includes slightly randomizing walls and creating/rendering explorable regions


use crate::{components::*, minimap::*, resources::*, };


// Initial generation function
pub fn terrain_generation(
    mut commands: Commands,
    mut mw: ResMut<WallGrid>,
    mut mg: ResMut<MapGrid>,
    map_cam: Query<(&Camera, &GlobalTransform)>,
) {

    // Determine how to switch between certain patterns, especially for textures/rules
    // Example - cave with walls bending randomly, vs square labrynth vs outdoor forests, etc...

    square_generation(mw.as_ref(), mg.as_ref());
}


// Most basic generator - Creates empty squares, with thin walls if applicable
pub fn square_generation(mw: &WallGrid, mg: &MapGrid){
    for y in 0..mg.dim_y{
        for x in 0..mg.dim_x{
            // Create a 'room' with floor sprite
            // If there are walls in the current tile, 'render' them.
            // Add some randomness to the room such as plants or collectable items
            
        }
    }
}