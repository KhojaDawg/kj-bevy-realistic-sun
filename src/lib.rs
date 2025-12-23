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
//! Realistic Sun | Bevy
//! --------------|-----
//! 0.0 | 0.17
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
//! 2. add an [`Environment`] resource to the world
//!    ```rust,no_run
//!    # use bevy::app::App;
//!    # use kj_bevy_realistic_sun::Environment;
//!    # let mut app = App::new();
//!    let environment = Environment::default()
//!        .with_axial_tilt(Environment::AXIAL_TILT_EARTH)
//!        .with_latitude_deg(30.0)
//!        .with_hours_since_noon(-2.0)
//!        .with_date(Environment::DATE_SPRING);
//!    app.insert_resource(environment);
//!    ```
//! 
//! 3. Add an entity with both a [`DirectionalLight`](https://docs.rs/bevy/0.17.3/bevy/light/struct.DirectionalLight.html)
//!    and [`Sun`] components.
//!    ```rust,no_run
//!    # use bevy::ecs::prelude::Commands;
//!    # use bevy::ecs::world::CommandQueue;
//!    # use bevy::light::DirectionalLight;
//!    # use bevy::prelude::World;
//!    # use kj_bevy_realistic_sun::Sun;
//!    # let mut command_queue = CommandQueue::default();
//!    # let world = World::default();
//!    # let mut commands = Commands::new(&mut command_queue, &world);
//!    commands.spawn((
//!        DirectionalLight::default(),
//!        Sun,
//!    ));
//!    ```
//! 
//! Now whenever you update the variables in [`Environment`] from any schedule, the light with the
//! [`Sun`] component attached will orient itself accordingly on the next frame.
use bevy::prelude::*;

mod conversion;
mod environment;
pub use environment::Environment;


/// Adds the systems and resources needed for [`Sun`] components to update their
/// attached [`Transform`s](Transform)
/// 
/// ```no_run
/// # use bevy::app::App;
/// # use kj_bevy_realistic_sun::RealisticSunDirectionPlugin;
/// let app = App::new();
///     .add_plugins(RealisticSunDirectionPlugin);
/// ```
/// 
/// Adds an [`Environment`] resource with default values, but those values can be overridden by
/// just adding your own [`Environment`]
pub struct RealisticSunDirectionPlugin;
impl Plugin for RealisticSunDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Environment::default());
        app.add_systems(Update, update_sun_lights);
    }
}

/// Attach to a
/// [`DirectionalLight`](https://docs.rs/bevy/0.17.3/bevy/light/struct.DirectionalLight.html)
/// representing your sun
/// 
/// Any Entity with this component attached will have its [`Transform`] updated every frame to point
/// the way the sun would be pointing given the current values in the [`Environment`] resource.
/// Intended for use with a `DirectionalLight` but can work on anything with a [`Transform`]
/// 
/// ```no_run
/// # use bevy::ecs::prelude::Commands;
/// # use bevy::ecs::world::CommandQueue;
/// # use bevy::light::DirectionalLight;
/// # use bevy::prelude::World;
/// # use kj_bevy_realistic_sun::Sun;
/// # let mut command_queue = CommandQueue::default();
/// # let world = World::default();
/// # let mut commands = Commands::new(&mut command_queue, &world);
/// commands.spawn((
///     DirectionalLight::default(),
///     Sun,
/// ));
/// ```
#[derive(Clone, Copy, Debug)]
#[derive(Component)]
#[require(Transform)]
pub struct Sun;

/// Runs once per frame, updating every entity with a [`Sun`] component to face in
/// a calculated direction
/// 
/// Direction is calculated based on the values in the [`Environment` resource](Environment)
fn update_sun_lights(
    mut lights: Query<&mut Transform, With<Sun>>,
    environment: Res<Environment>,
){
    let earth_tilt_angle = -environment.time_of_year.cos() / 2.0 * environment.axial_tilt;
    let earth_tilt_rotation = Quat::from_rotation_x(earth_tilt_angle);
    let time_of_day_rotation = Quat::from_rotation_z(environment.time_of_day);
    let latitude_rotation = Quat::from_rotation_x(environment.latitude);
    let total_rotation = latitude_rotation * time_of_day_rotation * earth_tilt_rotation;
    let light_direction = total_rotation * Vec3::NEG_Y;
    for mut transform in &mut lights {
        transform.look_to(light_direction, Vec3::Y);
    }
}
