use std::f32::consts::{PI, TAU};
use bevy::prelude::*;
use kj_bevy_realistic_sun::*;


// controls
const CAMERA_ROTATE_SPEED: f32 = 1.0;
const ENVIRONMENT_NORMAL_SPEED: f32 = 0.4;
const ENVIRONMENT_SLOW_SPEED: f32 = 0.05;
const ENVIRONMENT_FAST_SPEED: f32 = 2.0;

// general
const FONT_SIZE: f32 = 11.0;
const GIZMO_DISTANCE: f32 = 1000.0;
const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
const UI_BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);

// sun
const SUN_RADIUS: f32 = 10.0;
const SUN_OFFSET: f32 = -5.0;
const SUN_COLOR: Color = Color::srgb(0.8, 0.9, 0.1);
const SUN_PATH_OFFSET: f32 = 5.0;
const SUN_PATH_DAY_RESOLUTION: usize = 50;
const SUN_PATH_YEAR_RESOLUTION: usize = 10;
const SUN_PATH_ALPHA: f32 = 0.05;

// axes
const X_AXIS_COLOR: Color = Color::srgb(0.9, 0.1, 0.0);
const Y_AXIS_COLOR: Color = Color::srgb(0.0, 0.9, 0.1);
const Z_AXIS_COLOR: Color = Color::srgb(0.1, 0.0, 0.9);


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RealisticSunDirectionPlugin))
        .add_systems(Startup, (spawn_entities, spawn_ui))
        .add_systems(Update, (
            process_camera_input,
            process_environment_input,
            update_environment_ui.after(process_environment_input),
            draw_axes, draw_sun_path, draw_sun
        ))
        .run();
}

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
struct CameraControl {
    yaw: f32,
    pitch: f32,
}

#[derive(Clone, Copy, Debug)]
#[derive(Component)]
enum EnvironmentValueText {
    TimeOfDay,
    TimeOfYear,
    Latitude,
    AxialTilt,
}

fn process_camera_input(
    mut cameras: Query<(&mut Transform, &mut CameraControl), With<Camera>>,
    input: Res<ButtonInput<KeyCode>>, time: Res<Time>,
){
    let delta = time.delta_secs();
    // collect inputs
    let mut yaw_input = 0.0;
    let mut pitch_input = 0.0;
    if input.pressed(KeyCode::ArrowRight){ yaw_input -= 1.0; }
    if input.pressed(KeyCode::ArrowLeft){ yaw_input += 1.0; }
    if input.pressed(KeyCode::ArrowUp){ pitch_input += 1.0; }
    if input.pressed(KeyCode::ArrowDown){ pitch_input -= 1.0; }
    // apply inputs
    for (mut transform, mut camera) in &mut cameras {
        camera.yaw += yaw_input * CAMERA_ROTATE_SPEED * delta;
        camera.pitch = (camera.pitch + pitch_input * CAMERA_ROTATE_SPEED * delta).clamp(-PI/2.0, PI/2.0);
        let yaw_rotation = Quat::from_axis_angle(Vec3::Y, camera.yaw);
        let pitch_rotation = Quat::from_axis_angle(Vec3::X, camera.pitch);
        transform.rotation = yaw_rotation * pitch_rotation;
    }
}

fn process_environment_input(
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
    let speed: f32 = if slow_modifier_pressed { ENVIRONMENT_SLOW_SPEED }
        else if speed_modifier_pressed { ENVIRONMENT_FAST_SPEED }
        else { ENVIRONMENT_NORMAL_SPEED };
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

fn update_environment_ui(
    mut text_labels: Query<(&mut Text, &EnvironmentValueText)>,
    environment: Res<Environment>,
){
    for (mut text, label) in &mut text_labels {
        text.0 = match label {
            EnvironmentValueText::TimeOfDay => {
                let hours = environment.time_of_day * conversion::RAD_TO_HOURS + 12.0;
                let seconds = hours * 60.0 * 60.0;
                let total_seconds_int = seconds.round() as isize;
                let hours_int = total_seconds_int / (60 * 60);
                let minutes_int = total_seconds_int / 60 % 60;
                format!("{:.3} rad ({:02}:{:02})", environment.time_of_day, hours_int, minutes_int)
            },
            EnvironmentValueText::TimeOfYear => format!("{:.3} rad", environment.time_of_year),
            EnvironmentValueText::Latitude => format!(
                "{:.3} rad ({:.1} deg)",
                environment.latitude,
                environment.latitude * conversion::RAD_TO_DEG,
            ),
            EnvironmentValueText::AxialTilt => format!(
                "{:.3} rad ({:.1} deg)",
                environment.axial_tilt,
                environment.axial_tilt * conversion::RAD_TO_DEG,
            ),
        };
    }
}

fn draw_sun(mut gizmos: Gizmos, environment: Res<Environment>){
    // draw sun
    let direction_to_sun = -calculate_sun_direction(
        environment.time_of_day, environment.time_of_year,
        environment.latitude, environment.axial_tilt,
    );
    let sun_position = direction_to_sun * (GIZMO_DISTANCE + SUN_OFFSET);
    let sun_rotation = Quat::look_at_rh(Vec3::ZERO, direction_to_sun, Vec3::Y);
    let sun_gizmo_isometry = Isometry3d::new(sun_position, sun_rotation);
    gizmos.circle(sun_gizmo_isometry, SUN_RADIUS, SUN_COLOR);
}

fn draw_sun_path(mut gizmos: Gizmos, environment: Res<Environment>){
    // draw sun day path
    let sun_path_color = SUN_COLOR.with_alpha(SUN_PATH_ALPHA);
    let step = 1.0 / (SUN_PATH_DAY_RESOLUTION as f32) * TAU;
    for i in 0..SUN_PATH_DAY_RESOLUTION {
        let t_0 = i as f32 * step;
        let t_1 = (i+1) as f32 * step;
        let dir_0: Vec3 = -calculate_sun_direction(t_0, environment.time_of_year, environment.latitude, environment.axial_tilt);
        let dir_1: Vec3 = -calculate_sun_direction(t_1, environment.time_of_year, environment.latitude, environment.axial_tilt);
        gizmos.line(dir_0 * GIZMO_DISTANCE, dir_1 * (GIZMO_DISTANCE + SUN_PATH_OFFSET), sun_path_color);
    }
    // draw sun year path
    let step = 1.0 / (SUN_PATH_YEAR_RESOLUTION as f32) * PI;
    for i in 0..SUN_PATH_YEAR_RESOLUTION {
        let t_0 = i as f32 * step;
        let t_1 = (i+1) as f32 * step;
        let dir_0: Vec3 = -calculate_sun_direction(environment.time_of_day, t_0, environment.latitude, environment.axial_tilt);
        let dir_1: Vec3 = -calculate_sun_direction(environment.time_of_day, t_1, environment.latitude, environment.axial_tilt);
        gizmos.line(dir_0 * GIZMO_DISTANCE, dir_1 * (GIZMO_DISTANCE + SUN_PATH_OFFSET), sun_path_color);
    }
}

fn draw_axes(mut gizmos: Gizmos){
    // axes
    gizmos.circle(Quat::look_to_rh(Vec3::Y, Vec3::Z), GIZMO_DISTANCE, Y_AXIS_COLOR);
    gizmos.circle(Quat::look_to_rh(Vec3::X, Vec3::Y), GIZMO_DISTANCE, X_AXIS_COLOR);
    gizmos.circle(Quat::look_to_rh(Vec3::Z, Vec3::Y), GIZMO_DISTANCE, Z_AXIS_COLOR);
}

fn spawn_entities(mut commands: Commands){
    commands.insert_resource(
        Environment::default()
            .with_axial_tilt(Environment::AXIAL_TILT_EARTH)
            .with_latitude(Environment::LATITUDE_NEW_JERSEY)
            .with_date(-1.2)
            .with_hours_since_noon(-3.0)
    );
    // spawn camera
    commands.spawn((
        Camera3d::default(),
        Camera{
            clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR),
            ..default()
        },
        CameraControl{ yaw: PI, ..default() },
    ));
}

fn spawn_ui(mut commands: Commands){
    let font = TextFont {
        font_size: FONT_SIZE,
        ..default()
    };
    commands.spawn((
        Node{
            position_type: PositionType::Absolute,
            left: Val::ZERO,
            bottom: Val::ZERO,
            flex_direction: FlexDirection::Column,
            padding: UiRect::axes(Val::Px(7.0), Val::Px(4.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(UI_BACKGROUND_COLOR),
        children![
            (Text::new("-Environment Controls-"), font.clone()),
            label_value_row_bundle("Time of Day: Q/A", font.clone(), EnvironmentValueText::TimeOfDay),
            label_value_row_bundle("Time of Year: W/S", font.clone(), EnvironmentValueText::TimeOfYear),
            label_value_row_bundle("Latitude: E/D", font.clone(), EnvironmentValueText::Latitude),
            label_value_row_bundle("Axial Tilt: R/F", font.clone(), EnvironmentValueText::AxialTilt),
            (Text::new("-Camera Controls-"), font.clone()),
            (Text::new("Yaw/Pan: Left/Right Arrow"), font.clone()),
            (Text::new("Pitch/Tilt: Up/Down Arrow"), font.clone()),
        ]
    ));
}

fn label_value_row_bundle<S>(
    label_string: S, font: TextFont, marker: EnvironmentValueText,
) -> impl Bundle where S: Into<String>{
    (
        Node{
            flex_direction: FlexDirection::Row,
            flex_grow: 1.0,
            column_gap: Val::Px(10.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        children![
            (Text::new(label_string.into()), font.clone()),
            (Text::new("VALUE"), font.clone(), marker),
        ],
    )
}
