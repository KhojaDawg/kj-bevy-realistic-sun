use bevy::prelude::*;
use kj_bevy_realistic_sun::*;


/// Speed the camera turns at
const CAMERA_TURN_SPEED: f32 = 1.0;
/// Height of floor under objects
const FLOOR_HEIGHT: f32 = -0.6;
/// Bounding size of example objects
const OBJECT_SIZE: f32 = 0.5;
/// Spacing between objects
const OBJECT_SPACING: f32 = 1.0;

/// Base object that the camera is parented to
#[derive(Clone, Copy, Debug)]
#[derive(Component)]
struct CameraBase;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RealisticSunDirectionPlugin))
        .add_systems(Startup, (spawn_camera, spawn_floor, spawn_objects, spawn_sun))
        .add_systems(Update, process_camera_input)
        .run();
}

fn process_camera_input(
    mut camera_bases: Query<&mut Transform, With<CameraBase>>,
    input: Res<ButtonInput<KeyCode>>, time: Res<Time>,
){
    let delta = time.delta_secs();
    let mut camera_rotate_input = 0.0;
    if input.pressed(KeyCode::ArrowRight){ camera_rotate_input += 1.0; }
    if input.pressed(KeyCode::ArrowLeft){ camera_rotate_input -= 1.0; }
    for mut transform in &mut camera_bases {
        transform.rotate_axis(Dir3::Y, camera_rotate_input * CAMERA_TURN_SPEED * delta);
    }
}

fn spawn_camera(mut commands: Commands){
    commands.spawn((
        Transform::default(),
        CameraBase,
        children![(
            Transform::from_xyz(0.0, 0.7, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
            Camera3d::default(),
        )],
    ));
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
){
    let floor_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.0)));
    let floor_material = materials.add(StandardMaterial::default());
    commands.spawn((
        Transform::from_xyz(0.0, FLOOR_HEIGHT, 0.0),
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_material),
    ));
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
){
    let cube_position = Vec3::new(0.0, 0.0, 0.0);
    let cube_mesh = meshes.add(Cuboid::new(OBJECT_SIZE, OBJECT_SIZE, OBJECT_SIZE));
    let cube_material = materials.add(StandardMaterial{
        base_color: Color::srgb(0.9, 0.1, 0.0),
        ..default()
    });
    commands.spawn((
        Transform::from_translation(cube_position * OBJECT_SPACING),
        Mesh3d(cube_mesh),
        MeshMaterial3d(cube_material),
    ));
}

fn spawn_sun(mut commands: Commands){
    commands.spawn((
        DirectionalLight{
            shadows_enabled: true,
            ..default()
        },
        Sun,
    ));
}
