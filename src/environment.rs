//! Contains the [`Environment`] resource and its code
use std::f32::consts::PI;
use bevy::prelude::*;
use crate::conversion::*;


/// Holds the values that control the light direction
/// 
/// To control a light with a [`Sun`](crate::Sun) component, change the values in this resource
/// instead of changing its Quaternion rotation.
/// 
/// ```no_run
/// # use kj_bevy_realistic_sun::Environment;
/// // Creates a new `Environment` resource with some predefined values
/// let environment = Environment::default()
///     // Sets axial tilt to Earth's (approx 23 degrees)
///     .with_axial_tilt(Environment::AXIAL_TILT_EARTH)
///     // Sets latitude to 30 degrees north
///     .with_latitude_deg(30.0)
///     // Sets time of day to 2 hours before local solar noon (10 AM)
///     .with_hours_since_noon(-2.0); 
/// // Insert the resource into the ECS using Commands within a system
/// # use bevy::ecs::prelude::Commands;
/// # use bevy::ecs::world::CommandQueue;
/// # use bevy::prelude::World;
/// # let mut command_queue = CommandQueue::default();
/// # let world = World::default();
/// # let mut commands = Commands::new(&mut command_queue, &world);
/// commands.insert_resource(environment);
/// // Or the App directly at startup or in a plugin
/// # use bevy::app::App;
/// # let mut app = App::new();
/// app.insert_resource(environment);
/// ```
/// 
/// Sun direction is calculated each frame from these values, meaning they can be modified at
/// runtime, in any schedule. Once per frame in the [`Update`] schedule, a system will run which
/// will update all entities with a [`Sun`](crate::Sun) to face where the sun light should face
/// with the values here in the environment resource
/// 
/// **Note:** all values are stored in *radians*. All functions that manipulate the values will have
/// an equivalent in some other more common unit for that value like degrees, but if you access or
/// set the values directly they *must* be in radians.
#[derive(Clone, Copy, Debug, Default)]
#[derive(Resource)]
pub struct Environment
{
    /// Axial tilt of the planet being simulated, in radians
    pub axial_tilt: f32,
    
    /// Latitude in radians
    /// 
    /// The equator is latitude `0.0`, the north pole `PI/2.0`, and the south pole `-2.0`
    /// 
    /// **Note:** while negative latitudes (aka southern hemisphere) *are* supported,
    /// [`time_of_year`](Environment::time_of_year) is going to be opposite of how it is described
    /// in the docs. For example a `time_of_year` of `0.0` would represent the local solar summer
    /// solstice in the northern hemisphere, where the sun is at its highest, however in the
    /// southern hemisphere this will be when the sun is at its lowest.
    pub latitude: f32,
    
    /// Time of day in radians
    /// 
    /// Solar noon is `0.0`, with midnight being `PI`/`-PI`. Values outside this range are valid and
    /// will loop back around to a point until floating point precision starts causing problems, so
    /// I recommend normalizing your time of day to `-PI` to `PI` range. Positive/increasing values
    /// are forward in time, and negative/decreasing values are backward
    pub time_of_day: f32,
    
    /// Time of year in radians
    /// 
    /// The summer solstice is at `0.0`, with the winter solstice at `PI`/`-PI`. Values outside this
    /// range are valid and will loop back around to a point until floating point precision starts
    /// causing problems, so I recommend normalizing your time of year to a value from `-PI` to
    /// `PI`. Positive/increasing values are forward in time, and negative/decreasing
    /// values are backward
    pub time_of_year: f32,
}

impl Environment
{
    /// Value for setting [`axial_tilt`](Environment::axial_tilt) to Earth's
    /// 
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource
    /// // with the axial tilt set to Earth's
    /// let environment = Environment::default()
    ///     .with_axial_tilt(Environment::AXIAL_TILT_EARTH);
    /// ```
    pub const AXIAL_TILT_EARTH: f32 = 23.439281 * DEG_TO_RAD;

    /// Value for setting [`time_of_day`](Environment::time_of_day) to local solar midnight
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with
    /// // the time set to local solar midnight
    /// let environment = Environment::default()
    ///     .with_time_of_day(Environment::TIME_MIDNIGHT);
    /// ```
    pub const TIME_MIDNIGHT: f32 = -PI;

    /// Sets [`time_of_day`](Environment::time_of_day) to noon
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with
    /// // the time set to local solar noon
    /// let environment = Environment::default()
    ///     .with_time_of_day(Environment::TIME_NOON);
    /// ```
    pub const TIME_NOON: f32 = 0.0;

    /// Value to set [`latitude`](Environment::latitude) to New Jersey
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with the
    /// // latitude set to a location in New Jersey
    /// let environment = Environment::default()
    ///     .with_latitude(Environment::LATITUDE_NEW_JERSEY);
    /// ```
    pub const LATITUDE_NEW_JERSEY: f32 = 40.82706 * DEG_TO_RAD;

    /// Value t set [`latitude`](Environment::latitude) to the equator
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource
    /// // with the latitude set to the equator
    /// let environment = Environment::default()
    ///     .with_latitude(Environment::LATITUDE_EQUATOR);
    /// ```
    pub const LATITUDE_EQUATOR: f32 = 0.0;

    /// Value to set [`latitude`](Environment::latitude) to the north pole
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with
    /// // the latitude set to the north pole
    /// let environment = Environment::default()
    ///     .with_latitude(Environment::LATITUDE_NORTH_POLE);
    /// ```
    pub const LATITUDE_NORTH_POLE: f32 = PI / 2.0;

    /// Value to set [`latitude`](Environment::latitude) to the south pole
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with
    /// // the latitude set to the south pole
    /// let environment = Environment::default()
    ///     .with_latitude(Environment::LATITUDE_SOUTH_POLE);
    /// ```
    pub const LATITUDE_SOUTH_POLE: f32 = -PI / 2.0;

    /// Value to set [`time_of_year`](Environment::time_of_year) to the winter solstice, when the
    /// sun is going to be lowest in the sky
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with
    /// // the date set to the winter solstice
    /// let environment = Environment::default()
    ///     .with_date(Environment::DATE_WINTER);
    /// ```
    /// 
    /// **Note:** while latitudes in the southern hemisphere *are* supported, the time of year in
    /// the southern hemisphere is going to be opposite from the northern hemisphere.
    pub const DATE_WINTER: f32 = -PI;

    /// Value to set [`time_of_year`](Environment::time_of_year) halfway between the winter
    /// and summer solstice
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with the
    /// // date halfway between winter and summer
    /// let environment = Environment::default()
    ///     .with_date(Environment::DATE_SPRING);
    /// ```
    /// 
    /// **Note:** while latitudes in the southern hemisphere *are* supported, the time of year in
    /// the southern hemisphere is going to be opposite from the northern hemisphere.
    pub const DATE_SPRING: f32 = -PI / 2.0;

    /// Value to set [`time_of_year`](Environment::time_of_year) to the summer solstice, when the
    /// sun is going to be highest in the sky
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with
    /// // the date set to the summer solstice
    /// let environment = Environment::default()
    ///     .with_date(Environment::DATE_SUMMER);
    /// ```
    /// 
    /// **Note:** while latitudes in the southern hemisphere *are* supported, the time of year in
    /// the southern hemisphere is going to be opposite from the northern hemisphere.
    pub const DATE_SUMMER: f32 = 0.0;

    /// Value to set [`time_of_year`](Environment::time_of_year) halfway between
    /// the summer and winter solstices
    ///
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with the
    /// // date set halfway between summer and winter
    /// let environment = Environment::default()
    ///     .with_date(Environment::DATE_AUTUMN);
    /// ```
    /// 
    /// **Note:** while latitudes in the southern hemisphere *are* supported, the time of year in
    /// the southern hemisphere is going to be opposite from the northern hemisphere.
    pub const DATE_AUTUMN: f32 = PI / 2.0;

    /// Sets the axial tilt of the environment planet in radians
    /// 
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource
    /// // with the same axial tilt as earth's
    /// let environment = Environment::default()
    ///     .with_axial_tilt(Environment::AXIAL_TILT_EARTH);
    /// ```
    /// 
    /// To set the axial tilt in degrees, use [`with_axial_tilt_deg`](Environment::with_axial_tilt_deg)
    pub const fn with_axial_tilt(mut self, axial_tilt: f32) -> Self {
        self.axial_tilt = axial_tilt;
        self
    }

    /// Sets the axial tilt of the environment planet in degrees
    /// 
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with the
    /// // axial tilt defined manually in degrees
    /// let environment = Environment::default()
    ///     .with_axial_tilt_deg(23.439281);
    /// ```
    /// 
    /// **Note:** this function is not to be used with the constants provided with the
    /// [`Environment`] resource, which are in radians, not degrees. To use those constants, use the
    /// [`with_axial_tilt`](Environment::with_axial_tilt) function instead
    pub const fn with_axial_tilt_deg(self, axial_tilt: f32) -> Self {
        self.with_axial_tilt(axial_tilt * DEG_TO_RAD)
    }

    /// Sets the time of year of the enviroment in radians
    /// 
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource
    /// // with the date set to Spring
    /// let environment = Environment::default()
    ///     .with_date(Environment::DATE_SPRING);
    /// ```
    pub const fn with_date(mut self, date: f32) -> Self {
        self.time_of_year = date;
        self
    }

    /// Sets the environment latitude in Radians
    /// 
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource
    /// // with the latitude set to New Jersey
    /// let environment = Environment::default()
    ///     .with_latitude(Environment::LATITUDE_NEW_JERSEY);
    /// ```
    /// 
    /// To set latitude in degrees, see [`with_latitude_deg`](Environment::with_latitude_deg)
    pub const fn with_latitude(mut self, latitude: f32) -> Self {
        self.latitude = latitude;
        self
    }

    /// Sets the environment latitude in degrees
    /// 
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with
    /// // the latitude set manually to 30 degrees
    /// let environment = Environment::default()
    ///     .with_latitude_deg(30.0);
    /// ```
    /// 
    /// **Note:** this function does not work with any of the latitude constants, which are in
    /// radians, not degrees. To set latitude in radians or using a builtin constant, see
    /// [`with_latitude`](Environment::with_latitude)
    pub const fn with_latitude_deg(self, latitude: f32) -> Self {
        self.with_latitude(latitude * DEG_TO_RAD)
    }

    /// Sets the current solar time of day in radians. `0.0` is local solar noon, with `PI`/`-PI` at
    /// midnight. Positive values are forwards in time and negative values are backwards.
    /// 
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with
    /// // the time set to local solar noon
    /// let environment = Environment::default()
    ///     .with_time_of_day(Environment::TIME_NOON);
    /// ```
    pub const fn with_time_of_day(mut self, time_of_day: f32) -> Self {
        self.time_of_day = time_of_day;
        self
    }

    /// Sets the current solar time of day in hours. `0.0` is local solar noon, with positive values
    /// in the future and negative values in the past.
    /// 
    /// ```no_run
    /// # use kj_bevy_realistic_sun::Environment;
    /// // Creates a new `Environment` resource with the time
    /// // set to 2 hours before local solar noon (10 AM)
    /// let environment = Environment::default()
    ///     .with_hours_since_noon(-2.0);
    /// ```
    pub const fn with_hours_since_noon(self, time_of_day: f32) -> Self {
        self.with_time_of_day(time_of_day * HOURS_TO_RAD)
    }
}
