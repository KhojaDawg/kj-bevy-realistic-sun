//! # Bevy Realistic Sun
//! 
//! Controls the sun light direction in Bevy using realistic parameters instead of just XYZ
//! rotation. Allows for day night cycles where the sun arcs across the sky realistically based on
//! your game settings's latitude, time of year, and even the axial tilt of the planet you're on.
//! Sacrifice a small amount of direct creative control for a little more immersion, or use it to
//! make your game's day/night cycle feel more natural.
//! 
//! **Note:** this is done in a simplified way that is not perfectly astronomically precise, it just
//! allows control of the sun direction using parameters that make its motion feel more real and
//! allow for cool effects like the sun not setting during the summer solstice at
//! high enough latitudes.
//! 
//! ### Bevy Version Compatability
//!
//! | Realistic Sun | Bevy |
//! |--------------:|-----:|
//! |         0.1.0 | 0.20 |
//! |         0.0.5 | 0.19 |
//! |         0.0.4 | 0.18 |
//! |         0.0.3 | 0.17 |
//! 
//! ### Basic Usage
//! 
//! 1. Add the [`RealisticSunDirectionPlugin`] to your game's plugins
//!    ```rust,no_run
//!    # use bevy::app::App;
//!    # use kj_bevy_realistic_sun::RealisticSunDirectionPlugin;
//!    # let mut app = App::new();
//!    app.add_plugins(RealisticSunDirectionPlugin);
//!    ```
//! 
//! 2. add a [`SunParameters`] resource to the world
//!    ```rust,no_run
//!    # use bevy::app::App;
//!    # use kj_bevy_realistic_sun::SunParameters;
//!    # let mut app = App::new();
//!    let sun_params = SunParameters::default()
//!        .with_axial_tilt(SunParameters::AXIAL_TILT_EARTH)
//!        .with_latitude_deg(30.0)
//!        .with_hours_since_noon(-2.0)
//!        .with_date(SunParameters::DATE_SPRING);
//!    app.insert_resource(sun_params);
//!    ```
//! 
//! 3. Add an entity with both a [`DirectionalLight`](https://docs.rs/bevy/0.17.3/bevy/light/struct.DirectionalLight.html)
//!    and [`SunController`] components.
//!    ```rust,no_run
//!    # use bevy::ecs::prelude::Commands;
//!    # use bevy::ecs::world::CommandQueue;
//!    # use bevy::light::DirectionalLight;
//!    # use bevy::prelude::World;
//!    # use kj_bevy_realistic_sun::SunController;
//!    # let mut command_queue = CommandQueue::default();
//!    # let world = World::default();
//!    # let mut commands = Commands::new(&mut command_queue, &world);
//!    commands.spawn((
//!        SunController,
//!        DirectionalLight::default(),
//!    ));
//!    ```
//! 
//! Now whenever you update the variables in [`SunParameters`] from any schedule, the light with the
//! [`SunController`] component attached will orient itself accordingly on the next frame.

use bevy::prelude::*;

pub mod conversion;
mod params;
pub use params::SunParameters;


/// Adds the systems and resources needed for [`SunController`] components to update their
/// attached [`Transform`s](Transform)
/// 
/// ```no_run
/// # use bevy::app::App;
/// # use kj_bevy_realistic_sun::RealisticSunDirectionPlugin;
/// fn main() {
///     let app = App::new()
///         .add_plugins(RealisticSunDirectionPlugin);
/// }
/// ```
/// 
/// Adds an [`SunParameters`] resource with default values, but those values can be overridden by
/// just adding your own [`SunParameters`]
pub struct RealisticSunDirectionPlugin;
impl Plugin for RealisticSunDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SunParameters::default());
        app.add_systems(Update, update_sun_lights);
    }
}

/// Attach to a
/// [`DirectionalLight`](https://docs.rs/bevy/0.17.3/bevy/light/struct.DirectionalLight.html)
/// representing your sun to have its direction controlled by this crate
/// 
/// Any Entity with this component attached will have its [`Transform`] updated every frame to point
/// the way the sun would be pointing given the current values in the [`SunParameters`] resource.
/// Intended for use with a `DirectionalLight` but can work on anything with a [`Transform`]
/// 
/// ```no_run
/// # use bevy::ecs::prelude::Commands;
/// # use bevy::ecs::world::CommandQueue;
/// # use bevy::light::DirectionalLight;
/// # use bevy::prelude::World;
/// # use kj_bevy_realistic_sun::SunController;
/// # let mut command_queue = CommandQueue::default();
/// # let world = World::default();
/// # let mut commands = Commands::new(&mut command_queue, &world);
/// commands.spawn((
///     DirectionalLight::default(),
///     SunController,
/// ));
/// ```
#[derive(Clone, Copy, Debug)]
#[derive(Component)]
#[require(Transform, DirectionalLight)]
pub struct SunController;

/// Runs once per frame, updating every entity with a [`SunController`] component to face in
/// a calculated direction
/// 
/// Direction is calculated based on the values in the [`SunParameters` resource](SunParameters)
fn update_sun_lights(
    mut lights: Query<&mut Transform, With<SunController>>,
    environment: Res<SunParameters>,
){
    for mut transform in &mut lights {
        transform.look_to(environment.sun_dir(), Vec3::Y);
    }
}

/// Calculates the [`Vec3`] direction that the sun should be facing
pub fn calculate_sun_direction(time_of_day: f32, time_of_year: f32, latitude: f32, axial_tilt: f32) -> Dir3 {
    let earth_tilt_angle: f32 = -time_of_year.cos() / 2.0 * axial_tilt;
    let earth_tilt_rotation: Quat = Quat::from_rotation_x(earth_tilt_angle);
    let time_of_day_rotation: Quat = Quat::from_rotation_z(time_of_day);
    let latitude_rotation: Quat = Quat::from_rotation_x(latitude);
    let final_rotation: Quat = latitude_rotation * time_of_day_rotation * earth_tilt_rotation;
    let light_direction: Dir3 = final_rotation * Dir3::NEG_Y;
    light_direction
}
