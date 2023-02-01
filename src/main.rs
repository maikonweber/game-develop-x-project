use bevy::transform;
use bevy::{prelude::*, render::camera::ScalingMode,
};
use bevy::core_pipeline::clear_color::ClearColorConfig;

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
        .run();
}


fn spawn_player (mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.5, subdivisions: 60})),
        material: materials.add(Color::rgb(1.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(3.5, 0.5, 1.5),
        ..default()
    });
}


pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camer

    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.8, 0.4, 0.2)),
            ..Default::default()
        },
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(3.0, 0.5, -1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}


