#![allow(dead_code, unused)]
#![allow(non_snake_case)]
use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel,MouseMotion};
use bevy::render::camera::Projection;
use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::time::FixedTimestep;
use bevy::transform::transform_propagate_system;
use rand::*;


#[derive(Component)]
struct Camera {
    speed: f32,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "3D".to_string(),
                width: 1920.,
                height: 1080.,
                
                ..default()
            },
            add_primary_window: true,
            exit_on_all_closed: true,
            close_when_requested: true,
        }))
        .add_startup_system(setup)
        .add_system(move_camera)
        .add_system(spawn_cubes)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.7)))
        .add_plugin(WireframePlugin)
        .run();
}
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // camera
    let translation = Vec3::new(0.0, 50.0, 0.0000000000001);
    let radius = translation.length();
    commands.spawn((Camera3dBundle {

        transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
        Camera { speed: 10.0 }
    ));
    // camera hitbox
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {
            size: 5.0,
        })),
        transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
        visibility: Visibility {
            is_visible: false,
        },
        ..default()
    },
        Camera { speed: 10.0 }
    ));




    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 25.0, 0.0)),
        ..default()
    });


    // normal cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: -2.0,
            max_x: 2.0,
            min_y: -2.0,
            max_y: 2.0,
            min_z: -2.0,
            max_z: 2.0,
        })),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
        ..default()
    }));

    // center cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {
            size: 0.5,
        })),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });
}



fn spawn_cubes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {


    let mut rng = thread_rng();
}

fn move_camera(mut cameras: Query<(&mut Transform, &Camera)>, keys: Res<Input<KeyCode>>, timer: Res<Time>) {
    for (mut transform, camera) in &mut cameras {
        let dir_x = transform.local_x();
        let dir_y = transform.local_y();
        if keys.pressed(KeyCode::A) && transform.translation.x > -50.0 {
            transform.translation += -dir_x * timer.delta_seconds() * camera.speed;
            println!("{}", transform.translation.x);

        }
        if keys.pressed(KeyCode::D) && transform.translation.x < 50.0 {
            transform.translation += dir_x * timer.delta_seconds() * camera.speed;
            println!("{}", transform.translation.x);

        }
        if keys.pressed(KeyCode::W) && transform.translation.y < 25.0 {
            transform.translation += dir_y * timer.delta_seconds() * camera.speed;
            println!("{}", transform.translation.z);
        }
        if keys.pressed(KeyCode::S) && transform.translation.y > -25.0 {
            transform.translation += -dir_y * timer.delta_seconds() * camera.speed;
            println!("{}", transform.translation.z);
        }




    }
}

