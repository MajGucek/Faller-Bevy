use bevy::prelude::*;
use std::f32::consts::TAU;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::ecs::system::{OptionResMutState, ResMutState, ResState};
use bevy::ecs::system::lifetimeless::SResMut;
use bevy::input::mouse::{MouseWheel,MouseMotion};
use bevy::render::camera::Projection;
use bevy::window::PresentMode;
use bevy::window::CursorGrabMode;
use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::render::{render_resource::WgpuFeatures, settings::WgpuSettings};
use rand::*;
use bevy::time::Stopwatch;

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

#[derive(Component)]
struct Cube {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Component, Deref, DerefMut)]
pub struct TimerForTree (pub Timer);


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
        .add_system(pan_orbit_camera)
        .add_plugin(WireframePlugin)
        .add_system(spawn_cubes)
        .run();
}
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    // camera
    let translation = Vec3::new(0.0, 0.0, 5.0);
    let radius = translation.length();
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),

        ..default()
    },
                    PanOrbitCamera {
                        radius,
                        ..Default::default()
                    },
    ));




    // light
    let entity_spawn: Vec3 = Vec3::new(0.0, 8.0, 0.0);
    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(entity_spawn),
        ..default()
    }));

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            material: materials.add(Color::LIME_GREEN.into()),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Wireframe,
        Cube {x: 0.0, y: 0.0, z: 0.0 },
    ));
}

fn spawn_cubes(mut commands: Commands, time: Res<Time>, timer: Res<TimerForTree>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut rng = thread_rng();
    let time_fac: f32 = rng.gen_range(1.0..4.0);

    if timer.tick(time.delta()).just_finished() {



        commands.spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0} )),
            material: materials.add(Color::AZURE.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0)),
            ..default()
        },
            Wireframe,
        ))
            .insert_resource(TimerForTree (Timer::from_seconds(time_fac, TimerMode::Once)));
    }
}








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