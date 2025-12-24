//! Lets you play with the [`Environment`] variables directly and see their effect on the world and
//! other Bevy features like the procedural sky
//! 
//! ### Controls
//! 
//! Key         | Control
//! ------------|------------------------
//! Right Arrow | Rotate camera right
//! Left Arrow  | Rotate camera left
//! Up Arrow    | Raise camera vertically
//! Down Arrow  | Lower camera vertically
use bevy::prelude::*;
use bevy::{
    camera::Exposure, core_pipeline::tonemapping::Tonemapping,
    light::{AtmosphereEnvironmentMapLight, SunDisk},
    pbr::Atmosphere, post_process::bloom::Bloom, render::view::Hdr,
};
use kj_bevy_realistic_sun::*;


/// Speed the camera turns at
const CAMERA_TURN_SPEED: f32 = 2.0;
/// Speed that the camera height changes at
const CAMERA_HEIGHT_SPEED: f32 = 2.0;
/// Maximum height the camera can raise to (minimum is zero)
const MAX_CAMERA_HEIGHT: f32 = 3.0;
/// Height of floor under objects
const FLOOR_HEIGHT: f32 = -0.6;
/// Bounding size of example objects
const OBJECT_SIZE: f32 = 0.7;
/// Spacing between objects
const OBJECT_SPACING: f32 = 1.0;
/// Background color for the UI panes
const UI_BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);
/// Size of the font in the UI panes
const UI_FONT_SIZE: f32 = 13.0;

/// Base object that the camera is parented to
#[derive(Clone, Copy, Debug)]
#[derive(Component)]
struct CameraBase;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RealisticSunDirectionPlugin))
        .add_systems(Startup, (spawn_camera, spawn_floor, spawn_objects, spawn_sun, spawn_ui))
        .add_systems(Update, (draw_gizmos, process_camera_input))
        .run();
}

/// Draw gizmos to orient the player in space
fn draw_gizmos(mut gizmos: Gizmos){
    gizmos.axes(Transform::default(), OBJECT_SIZE);
}

fn process_camera_input(
    mut camera_bases: Query<&mut Transform, (With<CameraBase>, Without<Camera>)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<CameraBase>)>,
    input: Res<ButtonInput<KeyCode>>, time: Res<Time>,
){
    let delta = time.delta_secs();
    // rotation
    let mut camera_rotate_input = 0.0;
    if input.pressed(KeyCode::ArrowRight){ camera_rotate_input += 1.0; }
    if input.pressed(KeyCode::ArrowLeft){ camera_rotate_input -= 1.0; }
    // height
    let mut camera_height_input = 0.0;
    if input.pressed(KeyCode::ArrowUp){ camera_height_input += 1.0; }
    if input.pressed(KeyCode::ArrowDown){ camera_height_input -= 1.0; }
    // apply inputs
    for mut transform in &mut camera_bases {
        transform.rotate_axis(Dir3::Y, camera_rotate_input * CAMERA_TURN_SPEED * delta);
    }
    for mut transform in &mut cameras {
        transform.translation.y += camera_height_input * CAMERA_HEIGHT_SPEED * delta;
        transform.translation.y = transform.translation.y.clamp(0.0, MAX_CAMERA_HEIGHT);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn spawn_camera(mut commands: Commands){
    commands.spawn((
        Transform::default(),
        CameraBase,
        Visibility::Visible,
        children![(
            Transform::from_xyz(0.0, 1.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
            Camera3d::default(),
            Hdr,
            Tonemapping::AcesFitted,
            Exposure::SUNLIGHT,
            Bloom::NATURAL,
            Atmosphere::EARTH,
            AtmosphereEnvironmentMapLight::default(),
            // Fxaa::default(),
        )],
    ));
}

/// Spawns a floor plane for shapes to cast shadows on and show the effects of the moving sun light more clearly
fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
){
    let floor_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.0)));
    let floor_material = materials.add(StandardMaterial{
        base_color: Color::srgb(0.5, 0.5, 0.5),
        ..default()
    });
    commands.spawn((
        Transform::from_xyz(0.0, FLOOR_HEIGHT, 0.0),
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_material),
    ));
}

/// Spawns several shapes to cast shadows on the floor to show the effects of the sun light more clearly
fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
){
    let cube_position = Vec3::new(0.0, 0.0, -1.0);
    let sphere_position = Vec3::new(1.0, 0.0, 1.0);
    let torus_position = Vec3::new(-1.0, 0.0, 1.0);
    let cube_mesh = meshes.add(Cuboid::new(OBJECT_SIZE, OBJECT_SIZE, OBJECT_SIZE));
    let sphere_mesh = meshes.add(Sphere::new(OBJECT_SIZE / 2.0));
    let torus_mesh = meshes.add(Torus::new(OBJECT_SIZE / 4.0, OBJECT_SIZE / 2.0));
    let cube_material = materials.add(StandardMaterial{
        base_color: Color::srgb(0.9, 0.1, 0.0),
        ..default()
    });
    let sphere_material = materials.add(StandardMaterial{
        base_color: Color::srgb(0.0, 0.9, 0.1),
        ..default()
    });
    let torus_material = materials.add(StandardMaterial{
        base_color: Color::srgb(0.1, 0.0, 0.9),
        ..default()
    });
    commands.spawn((
        Transform::from_translation(cube_position * OBJECT_SPACING),
        Mesh3d(cube_mesh),
        MeshMaterial3d(cube_material),
    ));
    commands.spawn((
        Transform::from_translation(sphere_position * OBJECT_SPACING),
        Mesh3d(sphere_mesh),
        MeshMaterial3d(sphere_material),
    ));
    commands.spawn((
        Transform::from_translation(torus_position * OBJECT_SPACING),
        Mesh3d(torus_mesh),
        MeshMaterial3d(torus_material),
    ));
}

/// Spawns the sun light entity
fn spawn_sun(mut commands: Commands){
    commands.spawn((
        DirectionalLight{
            illuminance: light_consts::lux::DIRECT_SUNLIGHT,
            shadows_enabled: true,
            ..default()
        },
        SunDisk::EARTH,
        Sun,
    ));
}

/// Spawns the UI elements
fn spawn_ui(mut commands: Commands){
    let font = TextFont{
        font_size: UI_FONT_SIZE,
        ..default()
    };
    let padding = UiRect::axes(Val::Px(8.0), Val::Px(5.0));
    let background_color = BackgroundColor(UI_BACKGROUND_COLOR);
    // left pane - camera
    commands.spawn((
        Node{
            position_type: PositionType::Absolute,
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            padding,
            ..default()
        },
        background_color.clone(),
        children![
            (Text::new("Camera Controls:"), font.clone()),
            (Text::new("Rotate: Left Arrow/Right Arrow"), font.clone()),
            (Text::new("Height: Up Arrow/Down Arrow"), font.clone()),
        ],
    ));
    // right pane - sun
    commands.spawn((
        Node{
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            padding,
            ..default()
        },
        background_color.clone(),
        children![
            (Text::new("Sun Controls:"), font.clone()),
            (Text::new("Time of Day: Q/A"), font.clone()),
            (Text::new("Time of Year: W/S"), font.clone()),
            (Text::new("Latitude: E/D"), font.clone()),
            (Text::new("Axial Tilt: R/F"), font.clone()),
        ],
    ));
}
