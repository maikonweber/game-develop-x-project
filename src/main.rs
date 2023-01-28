#[allow(unused)]

use bevy::prelude::*;


fn main() {
    App::new()
    .add_system(hello_word)
    .run();   
}


fn hello_word() {
    println!("Hello Word")
}





