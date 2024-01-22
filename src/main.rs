use bevy::prelude::*;



#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
    .add_systems(Startup, add_people)
    .add_systems(Update, (hello_world, (greet_people, update_people).chain()))
    .run();
}

fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ford Prefect".to_string())));
    commands.spawn((Person, Name("Arthur Dent".to_string())));
    commands.spawn((Person, Name("Marvin Dprs".to_string())));
}

fn greet_people(query: Query<&mut Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
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