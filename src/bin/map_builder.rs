/* 
New Binary File
    Usage: Runs the Minimap Editor GUI
    GUI will support the following functions - 
        Open map from file(s)
        Edit map parameters (Tiles, Walls, Links between maps)
        Save Map to file(s)
    Potential far future features
        Hot-testing map (Load the map in game engine)
*/

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;


// TODO - figure out the binary pathings if I need to
mod prelude {
    pub use bevy::prelude::*;
    pub use serde::*;
    pub use bevy_roguelike::components::*;
    pub use bevy_roguelike::resources::*;
    pub use bevy_roguelike::minimap::*;
}



use prelude::*;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct GUICamera;

fn minimap_setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    // Camera starts pointed at 0,0 coordinate (Middle of screen)
    // camera.transform.translation.x += 1280.0 / 4.0;
    // camera.transform.translation.y += 720.0 / 4.0;
    
    commands.spawn((camera, MainCamera));

    bevy_roguelike::minimap::build_init(commands);
}



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
    // .init_state::<MapBuildState>()   // Seems like Bevy .13 uses this instead of add_state

    .add_systems(Startup, minimap_setup)
    .add_systems(Startup, menu_setup)
    
    // .add_systems(OnEnter(MapBuildState::LoadingMap), systems)
    // TODO - figure out a cleaner way to move between states - for now, render_map func just handles the state swap
    .add_systems(OnEnter(MapBuildState::RenderMap), (despawn_system::<MapCellSprite>, despawn_system::<MapWallSprite>))
    .add_systems(Update, (draw_grid, draw_wall, render_map).run_if(in_state(MapBuildState::RenderMap)))
    .add_systems(Update, mouse_wall_gui.run_if(in_state(MapBuildState::Drawing)))
    
    // Following system is just for the menu selections (Highlight, OnClick of valid menu slot)
    

    .run();
}