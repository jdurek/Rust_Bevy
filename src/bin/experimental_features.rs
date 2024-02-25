/* 
New Binary File
    Usage: Runs experimental features not yet fully implemented. 
    Most features in here will make their way to the main game loop if they work well enough.
    

    Due to the nature of this, there will be MULTIPLE experimental features, so the changelog will be in a separate comment block.
*/

/* CHANGELOG AND CURRENT SCOPE RECORDS 
 *
 * 2/25/24 - Initial creation - scope is to create a basic 'movement' system to go between grid points
 *      Goal - basic movement, with wall collison detection
 */


mod prelude {
    pub use bevy::prelude::*;
    pub use serde::*;
    pub use bevy_roguelike::components::*;
    pub use bevy_roguelike::resources::*;
    pub use bevy_roguelike::minimap::*;
}

use prelude::*;


// Default to GameplayState::Exploration after initial loading
// Only allow 2 things - movement and checking for menu buttons during that

// Will need to use the MapBuildStates (Load/Render) specifically (Not yet implemented)
fn main() {}