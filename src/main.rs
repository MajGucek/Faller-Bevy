use bevy::prelude::*;
use std::f32::consts::TAU;
use bevy::window::PresentMode;


#[derive(Component)]
struct Rotatable {
    speed: f32,
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "3D".to_string(),
                width: 1500.,
                height: 900.,
                
                ..default()
            },
            add_primary_window: true,
            exit_on_all_closed: true,
            close_when_requested: true,
        }))
        .add_startup_system(setup)
        .add_system(rotate_cube)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {


    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }));

    // Spawn a cube to rotate.
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::LIME_GREEN.into()),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Rotatable { speed: 0.3 },
    ));

}


fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>, keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Left) {
        for (mut transform, cube) in &mut cubes {
            transform.rotate_y(-cube.speed * TAU * timer.delta_seconds());
        }
    } else if keys.pressed(KeyCode::Right) {
        for (mut transform, cube) in &mut cubes {
            transform.rotate_y(cube.speed * TAU * timer.delta_seconds());
        }
    } else if keys.pressed(KeyCode::Up) {
        for (mut transform, cube) in &mut cubes {
            transform.rotate_x(-cube.speed * TAU * timer.delta_seconds());
        }
    } else if keys.pressed(KeyCode::Down) {
        for (mut transform, cube) in &mut cubes {
            transform.rotate_x(cube.speed * TAU * timer.delta_seconds());
        }
    }




}














