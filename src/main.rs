
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::{prelude::*, render::camera::ScalingMode,
};    



const WIDTH_BACKGROUD: i32 = 200;
const HEIGHT_BACKGROUD: i32 = 200;


mod Hex;

// PlayerBall - >

#[derive(Component)]
struct PlayerBall;



// Constantes -> 

const BallColor: Color = Color::rgb(0.7, 0.5, 0.7);




fn moviment_ball(mut positions: Query<(&PlayerBall, &mut Transform)>) {
    for (_head , mut transform) in positions.iter_mut() {
        transform.translation.y += 2.;
    }
}


fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window : WindowDescriptor { 
                fit_canvas_to_parent: true,
                ..default()
             },
             ..default()
            })
        )
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(moviment_ball)
        .run();
}


fn spawn_player (mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.5, subdivisions: 60})),
        material: materials.add(BallColor.into()),
        transform: Transform::from_xyz(3.5, 0.5, 1.5),
        ..default()
    })
   .insert(PlayerBall);
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size : 8.0 })),
        material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        ..Default::default()
    });

    // Camera de Frente estilo tabuleiro
    commands.spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0)
            )),
            ..Default::default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}



