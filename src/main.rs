use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// mod map;
mod components;
mod resources;
// mod map_pipeline;
mod minimap;

mod prelude {
    pub use bevy::prelude::*;
    pub use serde::*;
    // pub use crate::map::*;
    pub use crate::components::*;
    pub use crate::resources::*;
    // pub use crate::map_pipeline::*;
    pub use crate::minimap::*;
}

use prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct GUICamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Use this spot for loading in basic resources and initializations - including creating the first camera so we can display something
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    
    commands.spawn(camera);

    let ldtk_handle = asset_server.load("LDtk_resources.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
    
    // minimap_draw::build_init(commands);
    // map::build_map(commands);
}

fn minimap_setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    // Camera starts pointed at 0,0 coordinate (Middle of screen)
    // camera.transform.translation.x += 1280.0 / 4.0;
    // camera.transform.translation.y += 720.0 / 4.0;
    
    commands.spawn((camera, MainCamera));

    minimap::draw_map::build_init(commands);
}

fn main() {
    /* Following section is for rendering LDTK sprite-maps */
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
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))

    // The following was part of a Ldtk experimentation for a tileset rendering
    // .add_plugins(LdtkPlugin)
    // .insert_resource(LevelSelection::index(0))

    .add_state::<GameplayState>()
    .add_state::<TurnState>()
    .add_state::<MenuState>()

    // States are loaded in - Begin loading in our main logic
    .add_systems(Startup, setup)
    

    // .add_plugins(MapPlugin)
    .register_ldtk_entity::<PlayerBundle>("Player_Cursor")
    .add_systems(Update, (move_player_from_input, translate_grid_coord_entitites))
    
    // TODO - Figure out the schedule stuff so I can split the build_map and draw_map properly - Update is not the correct system, but it doesn't panic.
    // .add_systems(Update, map::draw_map)
    .run();
}



#[derive(Component, Default)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<Player>>,
    input: Res<Input<KeyCode>>,
){
    let key = input.get_pressed().next().cloned();
    let mut pos:GridCoords = GridCoords::new(0,0);

    if let Some(key) = key {
        match key {
            KeyCode::Up | KeyCode::W => pos = GridCoords::new(0,1),
            KeyCode::Left | KeyCode::A => pos = GridCoords::new(-1, 0),
            KeyCode::Down | KeyCode::S => pos = GridCoords::new(0, -1),
            KeyCode::Right | KeyCode::D => pos = GridCoords::new(1,0),
            _=> ()
        }
    }
    for mut player_grid_coords in players.iter_mut() {
        let dest = *player_grid_coords + pos;
        *player_grid_coords = dest;
    }
    
}

fn translate_grid_coord_entitites(
    mut grid_coord_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
){
    for (mut transform, grid_coords) in grid_coord_entities.iter_mut() {
        transform.translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(16))
        .extend(transform.translation.z);
    }
}