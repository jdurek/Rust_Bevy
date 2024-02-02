use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod map;
mod components;
mod resources;
mod map_pipeline;

mod prelude {
    pub use bevy::prelude::*;
    pub use serde::*;
    pub use crate::map::*;
    pub use crate::components::*;
    pub use crate::resources::*;
    pub use crate::map_pipeline::*;
}

use prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

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
    
    // map::build_map(commands);
}

fn main() {
    App::new()
    // .add_plugins(DefaultPlugins
    //     .set(WindowPlugin{
    //         primary_window: Some(Window{ 
    //             title: "Rusty Odyssey".to_string(),
    //             resolution: (1024 as f32, 720 as f32).into(),  // TODO - change this later for custom resolution (Or update it on the fly)
    //             ..Default::default()
    //         }),
    //         ..Default::default()
    //     }))
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(LdtkPlugin)
    .insert_resource(LevelSelection::index(0))

    .add_state::<GameplayState>()
    .add_state::<TurnState>()
    .add_state::<MenuState>()

    // States are loaded in - Begin loading in our main logic
    .add_systems(Startup, setup)
    

    // .add_plugins(MapPlugin)
    
    // TODO - Figure out the schedule stuff so I can split the build_map and draw_map properly - Update is not the correct system, but it doesn't panic.
    // .add_systems(Update, map::draw_map)
    .run();
}


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (greet_people));
    }
}


fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ford Prefect".to_string())));
    commands.spawn((Person, Name("Arthur Dent".to_string())));
    commands.spawn((Person, Name("Marvin Dprs".to_string())));
}


#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&mut Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0);
        }    
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Marvin Dprs" {
            name.0 = "Marvin Shut".to_string();
            break;
        }
    }
}