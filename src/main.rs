use bevy::prelude::*;



#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, HelloPlugin))
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