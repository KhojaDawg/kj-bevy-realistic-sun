//! Demonstrates the bare minimum needed to get the realistic sun direction working in Bevy
use std::f32::consts::TAU;
use bevy::prelude::*;
use kj_bevy_realistic_sun::*;


/// Day length in seconds
const DAY_LENGTH: f32 = 20.0;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RealisticSunDirectionPlugin))
        .add_systems(Startup, setup_essentials) // spawns the essentials
        .add_systems(Startup, setup_secondary) // spawns other example specific entities
        .add_systems(Update, update_time_of_day)
        .run();
}

/// Main setup function - sets up the essentials for the library to work
fn setup_essentials(mut commands: Commands){
    // insert resource
    let environment = Environment::default()
        .with_latitude_deg(40.0)
        .with_axial_tilt(Environment::AXIAL_TILT_EARTH);
    commands.insert_resource(environment);
    // spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.7, -10.0)
            .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
    ));
    // spawn sun light
    commands.spawn((
        DirectionalLight{
            shadows_enabled: true,
            ..default()
        },
        Sun,
    ));
}

/// Secondary setup function - sets up example specific things like the environment and UI
fn setup_secondary(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    // materials
    let grey_material = materials.add(StandardMaterial{
        base_color: Color::srgb(0.5, 0.5, 0.5),
        ..default()
    });
    let blue_material = materials.add(StandardMaterial{
        base_color: Color::srgb(0.0, 0.1, 1.0),
        ..default()
    });
    // floor
    commands.spawn((
        Transform::default(),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.0)))),
        MeshMaterial3d(grey_material),
    ));
    // spawn shapes
    commands.spawn((
        Transform::from_xyz(0.0, 0.5, 0.0),
        Mesh3d(meshes.add(Torus::new(0.5, 1.0))),
        MeshMaterial3d(blue_material),
    ));
}

fn update_time_of_day(
    mut environment: ResMut<Environment>,
    time: Res<Time>,
){
    let time_step = TAU / DAY_LENGTH;
    environment.time_of_day += time_step * time.delta_secs()
}
