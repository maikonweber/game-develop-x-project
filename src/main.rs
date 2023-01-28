#[allow(unused)]

use bevy::prelude::*;
use bevy::{prelude::Commands, input::InputPlugin};


#[derive(Component)]

struct Person;

struct Name(String);


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "I am a Window".to_string(),
            width: 1200.0,
            height: 1600.0,
            ..default()
        },
        ..default()
    }))
    .add_startup_system(add_people)
    .add_system(hello_word)
    .run();   
}


fn hello_word() {
    println!("Hello Word")
}


fn add_people (mut commands: Commands) {
    commands.spawn(Person);
}



