//! Contains the [`Environment`] resource and its code
use std::f32::consts::PI;
use bevy::prelude::*;
use crate::conversion::*;


/// These values control the sun lights in the environment
/// 
/// All values are stored in RADIANS. All functions that manipulate the values will have a version
/// in radians and an equivalent in some other more common unit for that value, but if you access or
/// set the values directly they must be in radians.
/// 
/// Sun direction is calculated each frame from these values, meaning they can be modified at
/// runtime, in any schedule. Once per frame in the [`Update`] schedule, a system will run which
/// will update all entities with a [`Sun`](crate::Sun) to face where the sun light should face
/// with the values here in the environment resource
#[derive(Clone, Copy, Debug, Default)]
#[derive(Resource)]
pub struct Environment {
    /// Axial tilt of the planet being simulated, in radians
    pub axial_tilt: f32,
    /// Latitude in radians
    /// 
    /// The equator is latitude `0.0`, the north pole `PI/2.0`, and the south pole `-2.0`
    pub latitude: f32,
    /// Time of day in radians
    /// 
    /// Solar noon is `0.0`, with midnight being `PI`/`-PI`. Values outside this range are valid and
    /// will loop back around (to a point until floating point precision starts causing problems).
    /// Positive/increasing values are forward in time, and negative/decreasing values are backward
    pub time_of_day: f32,
    /// Time of year in radians
    /// 
    /// The summer solstice is at `0.0`, with the winter solstice at `PI`/`-PI`. Values outside this
    /// range are valid and will loop back around (to a point until floating point precision starts
    /// causing problems). Positive/increasing values are forward in time, and negative/decreasing
    /// values are backward
    pub time_of_year: f32,
}
impl Environment
{
    /// Value for setting [`axial_tilt`](Environment::axial_tilt) to Earth's
    /// 
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Sets the environment's axial tilt to Earth's
    /// let environment = Environment::default()
    ///     .with_axial_tilt(Environment::AXIAL_TILT_EARTH);
    /// ```
    pub const AXIAL_TILT_EARTH: f32 = 23.439281 * DEG_TO_RAD;

    /// Value for setting [`time_of_day`](Environment::time_of_day) to midnight
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Sets the environment's time of day to local solar midnight
    /// let environment = Environment::default()
    ///     .with_time_of_day(Environment::TIME_MIDNIGHT);
    /// ```
    pub const TIME_MIDNIGHT: f32 = 0.0;

    /// Sets [`time_of_day`](Environment::time_of_day) to noon
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Sets the Environment's time of day to local solar noon
    /// let environment = Environment::default()
    ///     .with_time_of_day(Environment::TIME_NOON);
    /// ```
    pub const TIME_NOON: f32 = PI;

    /// Value to set [`latitude`](Environment::latitude) to New Jersey
    pub const LATITUDE_NEW_JERSEY: f32 = 40.17523 * DEG_TO_RAD;

    /// Value t set [`latitude`](Environment::latitude) to the equator
    pub const LATITUDE_EQUATOR: f32 = 0.0;

    /// Value to set [`latitude`](Environment::latitude) to the north pole
    pub const LATITUDE_NORTH_POLE: f32 = PI / 2.0;

    /// Value to set [`latitude`](Environment::latitude) to the south pole
    pub const LATITUDE_SOUTH_POLE: f32 = -PI / 2.0;

    /// Value to set [`time_of_year`](Environment::time_of_year) to the winter solstice, when the sun is going to be lowest in the sky
    pub const DATE_WINTER: f32 = -PI;

    /// Value to set [`time_of_year`](Environment::time_of_year) halfway between the winter and summer solstice
    pub const DATE_SPRING: f32 = -PI / 2.0;

    /// Value to set [`time_of_year`](Environment::time_of_year) to the summer solstice, when the sun is going to be highest in the sky
    pub const DATE_SUMMER: f32 = 0.0;

    /// Value to set [`time_of_year`](Environment::time_of_year) halfway between the summer and winter solstices
    pub const DATE_AUTUMN: f32 = PI / 2.0;

    /// Sets the axial tilt of the environment planet in radians
    /// 
    /// To set the axial tilt in degrees, use [`with_axial_tilt_deg`](Environment::with_axial_tilt_deg)
    pub const fn with_axial_tilt(mut self, axial_tilt: f32) -> Self {
        self.axial_tilt = axial_tilt;
        self
    }
    /// Sets the axial tilt of the environment planet in degrees
    pub const fn with_axial_tilt_deg(self, axial_tilt: f32) -> Self {
        self.with_axial_tilt(axial_tilt * DEG_TO_RAD)
    }
    /// Sets the time of year of the enviroment in radians
    pub const fn with_date(mut self, date: f32) -> Self {
        self.time_of_year = date;
        self
    }
    /// Sets the environment latitude in Radians
    /// 
    /// To set latitude in degrees, see [`with_latitude_deg`](Environment::with_latitude_deg)
    pub const fn with_latitude(mut self, latitude: f32) -> Self {
        self.latitude = latitude;
        self
    }
    /// Sets the environment latitude in degrees
    pub const fn with_latitude_deg(self, latitude: f32) -> Self {
        self.with_latitude(latitude * DEG_TO_RAD)
    }
    /// Sets the current solar time of day in radians
    pub const fn with_time_of_day(mut self, time_of_day: f32) -> Self {
        self.time_of_day = time_of_day;
        self
    }
    /// Sets the current solar time of day in hours
    pub const fn with_hours_since_noon(self, time_of_day: f32) -> Self {
        self.with_time_of_day(time_of_day * HOURS_TO_RAD)
    }
}
