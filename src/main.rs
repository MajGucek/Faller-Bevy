#![allow(dead_code, unused)]
#![allow(non_snake_case)]
#![feature(core_panic)]

use bevy::prelude::*;
use bevy::prelude::shape::Cube;
use bevy::time::FixedTimestep;
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
    intensity: u32,
}
impl Cubes {
    fn increase(&mut self) {
        self.cube_count += 1;
    }

    fn decrease(&mut self) {
        self.cube_count -= 1;
    }


    fn increase_intensity(&mut self) {
        self.intensity += 10;
    }
}

#[derive(Resource)]
struct Death {
    is_game_over: bool,
}

impl Death {
    pub fn die(&mut self) {
        self.is_game_over = true;
    }
}



#[derive(Resource)]
struct Score {
    score: u64,
}


#[derive(Resource)]
struct Counter {
    count: u32,
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
        .insert_resource(Counter { count: 0 })
        .insert_resource(Score { score: 0 })
        .insert_resource(Cubes { cube_count: 0, intensity: 100 })
        .insert_resource(Death { is_game_over: false })
        .add_system(move_camera)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.01))
                .with_system(spawn_cubes),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(increase_speed),
        )
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.7)))
        .add_system(move_cubes)
        .add_system(despawn_cubes)
        .add_system(player_collision_checker)
        .add_system(death_scene)
        .add_system(increase_intensity)
        .run();
}



fn increase_speed(mut blocks: Query<(&mut Blocks)>) {
    for (mut block) in &mut blocks {
        block.speed += 10.0;
    }
}


fn increase_intensity(
    mut cubes: ResMut<Cubes>
) {
    cubes.increase_intensity();
}

fn despawn_cubes(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Movable)>,
    mut cubes: ResMut<Cubes>
) {
    for (entity, mut transform, movable) in query.iter_mut() {
        if movable.auto_despawn == true && transform.translation.y > 55.0 {
            commands.entity(entity).despawn();
            cubes.decrease();
        }
    }
}

fn move_cubes(
    mut blocks: Query<(&mut Transform, &mut Blocks)>,
    timer: Res<Time>,
    mut score: ResMut<Score>,
    death: Res<Death>,
) {
    if !death.is_game_over {
        score.score += 1;
    }

    for (mut transform, block) in &mut blocks {
        let dir_y = transform.local_y();
        transform.translation += dir_y * timer.delta_seconds() * block.speed;
    }
}

fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cubes: ResMut<Cubes>, query: Query<Entity, With<Blocks>>,
    death: Res<Death>,
) {
    if death.is_game_over == false {
        // generate cubes in range [-25, 25]
        let mut rng = thread_rng();
        let mut x_range: i32 = rng.gen_range(-50..=50); //   <-    ->
        let mut z_range: i32 = rng.gen_range(-50..=50); // ↑      ↓

        let mut y_range: i32 = rng.gen_range(-150..=-125); // 3D

        let mut coords: Vec3 = Vec3::new(x_range as f32, y_range as f32, z_range as f32);


        if cubes.cube_count < cubes.intensity {
            commands.spawn((PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: -2.5, // left - right
                    max_x: 2.5,
                    min_y: -2.5, // lenght 3D
                    max_y: 2.5,
                    min_z: -2.5, // up down
                    max_z: 2.5,
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
}

fn move_camera(
    mut cameras: Query<(&mut Transform, &Camera)>,
    keys: Res<Input<KeyCode>>, timer: Res<Time>
) {
    for (mut transform, camera) in &mut cameras {
        let dir_x = transform.local_x();
        let dir_y = transform.local_y();
        if keys.pressed(KeyCode::A) && transform.translation.x > -40.0 {
            transform.translation += -dir_x * timer.delta_seconds() * camera.speed;
            // println!("Moved Left");

        }
        if keys.pressed(KeyCode::D) && transform.translation.x < 40.0 {
            transform.translation += dir_x * timer.delta_seconds() * camera.speed;
            // println!("Moved Right");

        }
        if keys.pressed(KeyCode::S) && transform.translation.z < 40.0 {
            transform.translation -= dir_y * timer.delta_seconds() * camera.speed;
            // println!("Moved Down");
        }
        if keys.pressed(KeyCode::W) && transform.translation.z > -40.0 {
            transform.translation += dir_y * timer.delta_seconds() * camera.speed;
            // println!("Moved Up");
        }
    }
}

fn player_collision_checker(
    mut commands: Commands,
    mut blocks: Query<(&Transform, &Blocks)>,
    mut cameras: Query<(&Transform, &Camera)>,
    mut death: ResMut<Death>
) {
    for (mut BlockTransform, block) in blocks.iter_mut() {
        for (mut CameraTransform, camera) in &mut cameras {

            if (CameraTransform.translation.x > BlockTransform.translation.x - 2.5 && CameraTransform.translation.x < BlockTransform.translation.x + 2.5) &&
                (CameraTransform.translation.y > BlockTransform.translation.y - 2.5 && CameraTransform.translation.y < BlockTransform.translation.y + 2.5) &&
                (CameraTransform.translation.z > BlockTransform.translation.z - 2.5 && CameraTransform.translation.z < BlockTransform.translation.z + 2.5) {
                death.die();
            }





        }
    }
}

fn death_scene(
    mut commands: Commands,
    death: Res<Death>,
    entities: Query<Entity, Without<Camera>>,
    score: Res<Score>,
    mut counter: ResMut<Counter>,
) {
    if death.is_game_over == true {
        if counter.count == 0 {
            for entity in &entities {
                commands.entity(entity).despawn_recursive();
            }
            println!("Score: {}", score.score);
        }
        counter.count += 1;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // camera
    let translation = Vec3::new(0.0, 50.0, 0.0000000000001);
    let radius = translation.length();
    commands.spawn((Camera3dBundle {

        transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
                    Camera { speed: 10.0 }
    ));
}

