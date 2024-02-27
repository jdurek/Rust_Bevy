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

use bevy_roguelike::minimap;
use prelude::*;


#[derive(Component)]
struct MainCamera;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Use this spot for loading in basic resources and initializations - including creating the first camera so we can display something
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    
    commands.spawn(camera);

    
    
    // minimap_draw::build_init(commands);
    // map::build_map(commands);
}

fn minimap_setup(mut commands: Commands){
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    // Camera starts pointed at 0,0 coordinate (Middle of screen)
    // camera.transform.translation.x += 1280.0 / 4.0;
    // camera.transform.translation.y += 720.0 / 4.0;
    
    commands.spawn((camera, MainCamera));

    // Initializes resources - all functions assume the MapGrid and WallGrid resource exist
    // minimap::build_init(commands);

    // Loads in a predefined map from a filepath to replace our blank template
    println!("Loading map from /assets/maps...");
    minimap::build_from_file(commands, "assets\\maps\\TestMap.json");
    println!("Loaded!");
}



// Will need to use the MapBuildStates (Load/Render) specifically (Not yet implemented)
fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(WindowPlugin{
            primary_window: Some(Window{ 
                title: "Rusty Odyssey".to_string(),
                resolution: (1024 as f32, 720 as f32).into(),  // TODO - change this later for custom resolution (Or update it on the fly)
                ..Default::default()
            }),
            ..Default::default()
        }))
    
    .add_state::<MapBuildState>()

    .add_systems(Startup, minimap_setup) 

    .add_systems(OnEnter(MapBuildState::RenderMap), (despawn_system::<MapCellSprite>, despawn_system::<MapWallSprite>))
    .add_systems(Update, (draw_grid, draw_wall, render_map).run_if(in_state(MapBuildState::RenderMap)))
    
    
    
    .run();
}