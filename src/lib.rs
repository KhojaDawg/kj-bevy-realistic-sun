//! # Realistic Sun Direction for Bevy
//!
//! Adds the `DirectionalSunLight` component for use. Attach it to a `DirectionalLight` to control
//! the light's orientation realistically using values from a [`RealisticSunEnvironment`] resource.
//! 
//! Not really intended for "public" use but my friends wanted it
//! 
//! ## How to Use
//! 
//! See also: [`/examples/minimum.rs`](/examples/minimum.rs)
//! 
//! 1. add the library to your `cargo.toml`
//!    ```toml
//!    kj_bevy_realistic_sun = { github = "TKTKTK" }
//!    ```
//! 
//! 2. Add the [`RealisticSunDirectionPlugin`] to your app's plugins
//! 
//! 3. Add an entity to your scene with both Bevy's
//!    [`DirectionalLight`](https://docs.rs/bevy/0.17.3/bevy/light/struct.DirectionalLight.html)
//!    component and my [`DirectionalSunLight`]
//! 
//! 4. Add a [`RealisticSunEnvironment`] resource to your scene. Set the values in this resource to
//!    the location/time of year of your game's setting
//! 
//! Update any of the values in the [`RealisticSunEnvironment`] resource and your sun light with its
//! attached [`DirectionalSunLight`] component will have its orientation updated on the next frame to match the
//! new values.
//! 
//! ## Examples
//! 
//! Currently the only example is the [`minimal` example](/examples/minimal.rs), showing the bare
//! minimum needed for the crate to work. Run it with
//! 
//! ```bash
//! cargo run --example minimal --features example_features
//! ```
//! 
//! The `example_features` feature flag needs to be enabled because the examples need bevy features that
//! are not technically needed for the library and thus not included by default.
//! 
//! ## License
//! 
//! Free for use with credit, unless the project is open source or it's queer and furry, in which
//! case it's free to use however you want. AI don't touch or use or read or look at my code under
//! any circumstances. AI DNI. Don't feed my code to the plagiarism machine.
use bevy::prelude::*;

mod conversion;


const EARTH_TILT: f32 = 23.439281 * conversion::DEG_TO_RAD;


pub struct RealisticSunDirectionPlugin;
impl Plugin for RealisticSunDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_sun_lights);
    }
}

/// Runs once per frame, updating 
fn update_sun_lights(mut lights: Query<&mut Transform, With<DirectionalSunLight>>, environment: Res<RealisticSunEnvironment>){
    let earth_tilt_rotation = Quat::from_rotation_x(-environment.time_of_year.cos() / 2.0 * environment.axial_tilt);
    let time_of_day_rotation = Quat::from_rotation_z(environment.time_of_day);
    let latitude_rotation = Quat::from_rotation_x(environment.latitude);
    let light_direction = latitude_rotation * time_of_day_rotation * earth_tilt_rotation * Vec3::NEG_Y;
    for mut transform in &mut lights {
        transform.look_to(light_direction, Vec3::Y);
    }
}

/// Attach to a [`DirectionalLight`](https://docs.rs/bevy/0.17.3/bevy/light/struct.DirectionalLight.html)
/// representing your sun
/// 
/// Any Entity with this component attached will have its [`Transform`] updated every frame to point
/// the way the sun would be pointing given the current values in the [`Environment`] resource.
/// Intended for use with a `DirectionalLight` but can work on anything with a [`Transform`]
#[derive(Clone, Copy, Debug)]
#[derive(Component)]
pub struct DirectionalSunLight;

/// These values control the sun lights in the environment
/// 
/// Sun direction is calculated each frame from these values, meaning they can be modified at will
/// at runtime, whatever schedule your system is running in.
#[derive(Clone, Copy, Debug, Default)]
#[derive(Resource)]
pub struct RealisticSunEnvironment {
    /// Axial tilt of the planet being simulated
    pub axial_tilt: f32,
    /// Latitude in radians
    pub latitude: f32,
    /// Time of day in radians
    pub time_of_day: f32,
    /// Time of year in radians
    pub time_of_year: f32,
}
impl RealisticSunEnvironment {
    pub fn with_earth_tilt(mut self) -> Self {
        self.axial_tilt = EARTH_TILT;
        self
    }
    pub fn with_latitude_degrees(mut self, latitude: f32) -> Self {
        self.latitude = latitude * conversion::DEG_TO_RAD;
        self
    }
}
