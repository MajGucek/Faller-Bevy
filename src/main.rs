#![allow(dead_code, unused)]
#![allow(non_snake_case)]

use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel,MouseMotion};
use bevy::render::camera::Projection;
use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::shape::Cube;
use bevy::time::FixedTimestep;
use bevy::transform::transform_propagate_system;
use rand::*;


#[derive(Component)]
struct Camera {
    speed: f32,
}

#[derive(Component)]
struct Blocks {
    speed: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Resource)]
struct Cubes {
    cube_count: u32,
}

impl Cubes {
    fn increase(&mut self) {
        self.cube_count += 1;
    }

    fn decrease(&mut self) {
        self.cube_count -= 1;
    }
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
        .insert_resource(Cubes {cube_count: 0})
        .add_system(move_camera)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.05))
                .with_system(spawn_cubes),
        )
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.7)))
        .add_plugin(WireframePlugin)
        .add_system(move_cubes)
        .add_system(despawn_cubes)
        .run();
}

fn despawn_cubes(mut commands: Commands, mut query: Query<(Entity, &mut Transform, &Movable)>, mut cubes: ResMut<Cubes>) {
    for (entity, mut transform, movable) in query.iter_mut() {
        if movable.auto_despawn == true && transform.translation.y > 25.0 {
            commands.entity(entity).despawn();
            cubes.decrease();
        }
    }
}

fn move_cubes(mut blocks: Query<(&mut Transform, &Blocks)>, timer: Res<Time>) {
    for (mut transform, block) in &mut blocks {
        let dir_y = transform.local_y();
        transform.translation += dir_y * timer.delta_seconds() * block.speed;
    }
}

fn spawn_cubes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, mut cubes: ResMut<Cubes>, query: Query<Entity, With<Blocks>>) {
        // generate cubes in range [-25, 25]
        let mut rng = thread_rng();
        let mut x_range: f32 = rng.gen_range(-25.0..=25.0);
        let mut y_range: f32 = rng.gen_range(-15.0..=-5.0);
        let mut z_range: f32 = rng.gen_range(-25.0..=25.0);
        let mut coords: Vec3 = Vec3::new(x_range, y_range, z_range);

        if cubes.cube_count < 50 {
                commands.spawn((PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube {
                        size: 5.0,
                    })),
                    material: materials.add(Color::MIDNIGHT_BLUE.into()),
                    transform: Transform::from_translation(coords),
                    ..default()
                },
                                Blocks { speed: 10.0 },
                                Movable { auto_despawn: true },
                ));
                cubes.increase();
        }
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
        if keys.pressed(KeyCode::S) && transform.translation.z < 25.0 {
            transform.translation -= dir_y * timer.delta_seconds() * camera.speed;
            println!("{}", transform.translation.z);
        }
        if keys.pressed(KeyCode::W) && transform.translation.z > -25.0 {
            transform.translation += dir_y * timer.delta_seconds() * camera.speed;
            println!("{}", transform.translation.z);
        }
    }
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
}

