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
//! Q           | Advance time of day
//! A           | Reverse time of day
//! W           | Advance time of year
//! S           | Reverse time of year
//! E           | Increase latitude
//! D           | Decrease latitude
//! R           | Increase axial tilt
//! F           | Decrease axial tilt
//! Shift       | Increase speed of environment changes
//! Ctrl        | Decrease speed of environment changes

use std::f32::consts::{PI, TAU};
use bevy::prelude::*;
use bevy::{
    camera::Exposure, core_pipeline::tonemapping::Tonemapping,
    light::{AtmosphereEnvironmentMapLight, SunDisk},
    pbr::Atmosphere, post_process::bloom::Bloom, render::view::Hdr,
};
use kj_bevy_realistic_sun::*;

/// Speed that values in [`Environment`] change at in radians per second
const SUN_NORMAL_SPEED: f32 = 0.4;
/// Speed that values in [`Environment`] change when holding the slow button
const SUN_SLOW_SPEED: f32 = 0.05;
/// Speed that values in [`Environment`] change when holding the fast button
const SUN_FAST_SPEED: f32 = 2.0;
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

/// Marker component for the text labels used to display environment values in the UI
#[derive(Component)]
enum EnvironmentOutputLabel {
    TimeOfDay,
    TimeOfYear,
    Latitude,
    AxialTilt,
}

/// Base object that the camera is parented to
#[derive(Clone, Copy, Debug)]
#[derive(Component)]
struct CameraBase;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RealisticSunDirectionPlugin))
        .add_systems(Startup, (spawn_camera, spawn_floor, spawn_objects, spawn_sun, spawn_ui))
        .add_systems(Update, (
            draw_gizmos, process_camera_input, process_sun_input,
            update_labels.after(process_sun_input),
        ))
        .run();
}

/// Draw gizmos to orient the player in space
fn draw_gizmos(mut gizmos: Gizmos){
    gizmos.axes(Transform::default(), OBJECT_SIZE);
}

/// Takes player input for the camera and updates the camera position accordingly
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

/// Takes player input for the sun and updates the [`Environment`] accordingly
fn process_sun_input(
    mut environment: ResMut<Environment>, input: Res<ButtonInput<KeyCode>>, time: Res<Time>,
){
    let delta = time.delta_secs();
    // initialize input variables
    let mut time_of_day_input = 0.0;
    let mut time_of_year_input = 0.0;
    let mut latitude_input = 0.0;
    let mut axial_tilt_input = 0.0;
    // get inputs and store them in input variables
    if input.pressed(KeyCode::KeyQ){ time_of_day_input += 1.0; }
    if input.pressed(KeyCode::KeyA){ time_of_day_input -= 1.0; }
    if input.pressed(KeyCode::KeyW){ time_of_year_input += 1.0; }
    if input.pressed(KeyCode::KeyS){ time_of_year_input -= 1.0; }
    if input.pressed(KeyCode::KeyE){ latitude_input += 1.0; }
    if input.pressed(KeyCode::KeyD){ latitude_input -= 1.0; }
    if input.pressed(KeyCode::KeyR){ axial_tilt_input += 1.0; }
    if input.pressed(KeyCode::KeyF){ axial_tilt_input -= 1.0; }
    let speed_modifier_pressed: bool = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
    let slow_modifier_pressed: bool = input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);
    let speed: f32 = if slow_modifier_pressed { SUN_SLOW_SPEED }
        else if speed_modifier_pressed { SUN_FAST_SPEED }
        else { SUN_NORMAL_SPEED };
    // apply inputs to `Environment`
    environment.time_of_day += time_of_day_input * speed * delta;
    environment.time_of_year += time_of_year_input * speed * delta;
    environment.latitude += latitude_input * speed * delta;
    environment.axial_tilt += axial_tilt_input * speed * delta;
    // clamp/loop environment values as needed
    if environment.time_of_day > PI { environment.time_of_day -= TAU; }
    if environment.time_of_day < -PI { environment.time_of_day += TAU; }
    if environment.time_of_year > PI { environment.time_of_year -= TAU; }
    if environment.time_of_year < -PI { environment.time_of_year += TAU; }
    environment.latitude = environment.latitude.clamp(-PI/2.0, PI/2.0);
    environment.axial_tilt = environment.axial_tilt.clamp(-PI/2.0, PI/2.0);
}

/// Updates UI labels marked with [`EnvironmentOutputLabel`]
fn update_labels(
    mut labels: Query<(&mut Text, &EnvironmentOutputLabel)>, environment: Res<Environment>
){
    for (mut text, label) in &mut labels {
        text.0 = match label {
            EnvironmentOutputLabel::TimeOfDay => {
                let hours = environment.time_of_day * conversion::RAD_TO_HOURS + 12.0;
                let seconds = hours * 60.0 * 60.0;
                let total_seconds_int = seconds.round() as isize;
                let hours_int = total_seconds_int / (60 * 60);
                let minutes_int = total_seconds_int / 60 % 60;
                format!("{:.3} rad ({:02}:{:02})", environment.time_of_day, hours_int, minutes_int)
            },
            EnvironmentOutputLabel::TimeOfYear => format!("{:.3} rad", environment.time_of_year),
            EnvironmentOutputLabel::Latitude => format!(
                "{:.3} rad ({:.1} deg)",
                environment.latitude,
                environment.latitude * conversion::RAD_TO_DEG,
            ),
            EnvironmentOutputLabel::AxialTilt => format!(
                "{:.3} rad ({:.1} deg)",
                environment.axial_tilt,
                environment.axial_tilt * conversion::RAD_TO_DEG,
            ),
        };
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
    commands.insert_resource(
        Environment::default()
            .with_axial_tilt(Environment::AXIAL_TILT_EARTH)
            .with_latitude(Environment::LATITUDE_NEW_JERSEY)
            .with_hours_since_noon(-2.0)
    );
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
            (Text::new("Misc Controls:"), font.clone()),
            (Text::new("Camera Rotate: Left Arrow/Right Arrow"), font.clone()),
            (Text::new("Camera Height: Up Arrow/Down Arrow"), font.clone()),
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
            sun_control_row_bundle("Time of Day: Q/A", font.clone(), EnvironmentOutputLabel::TimeOfDay),
            sun_control_row_bundle("Time of Year: W/S", font.clone(), EnvironmentOutputLabel::TimeOfYear),
            sun_control_row_bundle("Latitude: E/D", font.clone(), EnvironmentOutputLabel::Latitude),
            sun_control_row_bundle("Axial Tilt: R/F", font.clone(), EnvironmentOutputLabel::AxialTilt),
            (Text::new("Speed/slow sun change input: Shift/Ctrl"), font.clone()),
        ],
    ));
}

fn sun_control_row_bundle<S>(
    label: S, font: TextFont, value: EnvironmentOutputLabel
) -> impl Bundle where S: Into<String> {
    (
        Node{
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Val::Px(10.0),
            ..default()
        },
        children![
            (Text::new(label.into()), font.clone()),
            (Text::new("0.0"), font.clone(), value),
        ]
    )
}
