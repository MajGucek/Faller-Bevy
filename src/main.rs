#![allow(dead_code, unused)]
#![allow(non_snake_case)]
use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel,MouseMotion};
use bevy::render::camera::Projection;
use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::time::FixedTimestep;


/*
#[derive(Component)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}
*/





#[derive(Resource, Default)]
pub struct NextDir(pub Dir);
// Left = from_z, Right = from_x
impl NextDir {
    pub fn switch(&mut self) {
        match self.0 {
            Dir::Left => { self.0 = Dir::Right; },
            Dir::Right => { self.0 = Dir::Left; },
        }
    }
}
#[derive(Default)]
pub enum Dir {
    #[default]
    Left,
    Right
}

#[derive(Resource, Default)]
struct BlockCount {
    count: u32,
}


#[derive(Component)]
struct VelocityZ {
    z: f32,
}

#[derive(Component)]
struct VelocityX {
    x: f32,
}

#[derive(Resource, Default)]
struct Clicked {
    counter: u32,
}
impl Clicked {
    fn click(&mut self) {
        self.counter += 1;
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
        .insert_resource(Clicked{counter: 0})
        .insert_resource(NextDir::default())
        .add_system(user_click)
        //.add_system(pan_orbit_camera)
        .add_plugin(WireframePlugin)
        .add_system(spawn_cubes)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(144.0))
                .with_system(spawn_cubes),
        )
        .insert_resource(BlockCount {count: 0})
        .add_system(moving_cube_z)

        // .add_system(moving_cube_x)
        .run();
}
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    // camera
    let translation = Vec3::new(20.0, 20.0, 20.0);
    let radius = translation.length();
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }));



    // light
    let entity_spawn: Vec3 = Vec3::new(0.0, 8.0, 0.0);
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(entity_spawn),
        ..default()
    });


    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: -2.5,
            max_x: 2.5,
            min_y: -50.0 + 2.5,
            max_y: 2.5,
            min_z: -2.5,
            max_z: 2.5,
        })),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    }));

}

fn user_click(mut click: ResMut<Clicked>, keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Key1) {
        click.click();
    }
}

fn spawn_cubes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, mut dir: ResMut<NextDir>, mut block_count: ResMut<BlockCount>, click_count: Res<Clicked>) {
        if click_count.counter != 0 {
            if block_count.count < 2 {
                match dir.0 {
                    Dir::Left => {
                        commands.spawn((PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
                            material: materials.add(Color::SEA_GREEN.into()),
                            transform: Transform::from_translation(Vec3::new(0.0, 5.0, -15.0)),
                            ..default()
                        },
                                        VelocityZ { z: -15.0 },
                        ));
                    },
                    Dir::Right => {
                        commands.spawn((PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
                            material: materials.add(Color::SEA_GREEN.into()),
                            transform: Transform::from_translation(Vec3::new(-15.0, 5.0, 0.0)),
                            visibility: Visibility {
                                is_visible: false,
                            },
                            ..default()
                        },
                                        VelocityX { x: -15.0 },
                        ));
                    }
                }
                dir.switch();
                block_count.count += 1;
            }
        }
}


fn moving_cube_z(mut cubes: Query<(&mut Transform, &mut VelocityZ)>, timer: Res<Time>, keys: Res<Input<KeyCode>>) {
    for (mut transform, cube) in &mut cubes {
        let dir = transform.local_z();
        let down = transform.local_y();
        if !keys.just_pressed(KeyCode::Space) {
            transform.translation += -dir * cube.z * timer.delta_seconds();
        } else {
            transform.translation += down * cube.z * timer.delta_seconds();
        }
    }
}

fn moving_cube_x(mut cubes: Query<(&mut Transform, &mut VelocityX)>, timer: Res<Time>) {
    if timer.delta_seconds() % 2.0 == 1.0 {
        for (mut transform, cube) in &mut cubes {
            let dir = transform.local_x();
            transform.translation += -dir * cube.x * timer.delta_seconds();
        }
    }
}

/*
// Zoom with scroll wheel, orbit with left mouse click.
fn pan_orbit_camera(windows: Res<Windows>, mut ev_motion: EventReader<MouseMotion>, mut ev_scroll: EventReader<MouseWheel>, input_mouse: Res<Input<MouseButton>>, mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>, ) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Left;
    //let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }


    }
}
fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

 */