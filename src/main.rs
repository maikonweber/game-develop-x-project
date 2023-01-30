use bevy::transform;
use bevy::{prelude::*, render::camera::ScalingMode,
};
use bevy::core_pipeline::clear_color::ClearColorConfig;



use yew::prelude::*;
use stylist::{css, Style};
use yew::prelude::*;

#[function_component]
fn Appy() -> Html {    
    let s = css!(
        r#"
            width: 100%;
            background-color: red;
            height: 75px;
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 10px;
            color : ${color};
            span, ${sel_div} {
                background-color: blue;

            }

        
            @media screen and ${breakpoint} {
                display: flex;

            }
        "#,
        color = "black",
        sel_div = "div.selected",
        breakpoint = "(max-width: 500px)",
    );

    let style = Style::new(s).expect("msg");

    return html! {

        <>
      
        <div class={style}> { "Hello Word"} 
        </div>

        </>
    };
}

// We load "App" and render it.



fn main() {
    yew::Renderer::<Appy>::new().render();
    App::new()
    .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}



#[derive(Component)]
struct Uiplugin {
    
}

#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}

#[derive(Component)]
struct PlayerXp(u32);

#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
struct MainMenuUi;


#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Player;


#[derive(Component)]
struct Friendly;


#[derive(Bundle)]





struct PlayerBundle {
    xp : PlayerXp,
    name: PlayerName, 
    health: Health,
    _p : Player,
}





#[derive(Resource, Default, Debug)]
struct StartingLevel(usize);



fn check_zero_health (
    mut query: Query<(&Health, &mut Transform, Option<&Player>)>,
) {
    for (health , mut transform, player) in query.iter_mut() {
        eprintln!("Entity at {} has {} HP.", transform.translation, health.hp);
    
    if health.hp <= 0.0 {
        transform.translation = Vec3::ZERO;
    }

    if let Some(player) = player {

    }
}
}

fn query_player(mut q: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = q.single_mut();

    
    // do something with the player and its transform
}



/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
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
        transform: Transform::from_xyz(1.5, 0.5, -1.5),
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


