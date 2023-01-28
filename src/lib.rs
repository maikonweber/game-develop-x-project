#[allow(unused)]
pub use bevy::prelude::*;
pub use bevy_renet::renet::*;
pub use bevy_renet::*;
use serde::{
    Deserialize, Serialize
};


#[derive(Debugm Serialize, Deserialize)]
pub enum ClientMessage {
    Ping,
}

#[derive(Debugm Serialize, Deserialize)]
pub enum ServerMessage {
    Pong,
}

pub const PROTOCOL_ID: u64 = 1000;



// use bevy::{
//     prelude::*,
//     render::camera::ScalingMode 
// };

// pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
// pub const RESOLUTION: f32 = 16.0/9.0;
// pub const TILE_SIZE : f32 = 0.10;


fn main() {
    let height: f32 = 900.0;
    App::new() 
    .insert_resource(LogSettings {
        filter: "info, wgpu_core=warn, wgpu_hal=off, rechannel=warn".into(),
        level: bevy::log::Level::DEBUG
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window : WindowDescriptor {
            title: "I am a Window".to_string(),
                    width: height * RESOLUTION,
                    height: height,
                    resizable: true,
                    ..Default::default()        
        },
        ..default()
    }))
    .add_plugin(RenetClientPlugin)
    .add_plugin(create_renet_client())
    .run();       
}
    


fn create_renet_client() -> {
    let current_time = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let client_id = current_time.as_millis() as u64;

    let connection_config = RenetConnectionConfig::default();

    let server_addr = Socket::new(localip().unwrap(), 42069); 
    
}


//     // My Setup


//     // .add_plugins(DefaultPlugins.set(WindowPlugin {
//     //     window: WindowDescriptor {
//     //         title: "I am a Window".to_string(),
//     //         width: 1600.0,
//     //         height: 800.0,
//     //         resizable: true,
//     //         ..Default::default()
//     //     },
//     //     ..default()
//     // }))

// fn spawn_camera(mut commands: Commands) {    

// }





// fn hello_word() {
//     println!("Hello Word")
// }


// fn add_people (mut commands: Commands) {
//     commands.spawn(Person);
// }



