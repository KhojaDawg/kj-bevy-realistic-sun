# Usage Guide

This crate controls the sun light direction in Bevy using realistic parameters instead of just XYZ
rotation. This allows for day night cycles where the sun arcs across the sky realistically based on
your game settings's latitude, time of year, and even the axial tilt of the planet you're on.
Sacrifices a small amount of direct creative control over light direction for a little more
immersion, or use it to make your game's procedural day/night cycle feel more natural.

**Note:** this is done in a simplified way that is not perfectly astronomically precise, it just
allows control of the sun direction using parameters that make its motion feel more real and allow
for cool effects like the sun not setting during the summer solstice at high enough latitudes.

### Bevy Version Compatability

Realistic Sun | Bevy
--------------|-----
0.0 | 0.17

### Basic Usage

1. Add the [`RealisticSunDirectionPlugin`] to your game's plugins
   ```rust
   app.add_plugins(RealisticSunDirectionPlugin);
   ```

2. add an [`Environment`] resource to the world
   ```rust
   let environment = Environment::default()
       .with_axial_tilt(Environment::AXIAL_TILT_EARTH)
       .with_latitude_deg(30.0)
       .with_hours_since_noon(-2.0)
       .with_date(Environment::DATE_SPRING);
   app.insert_resource(environment);
   ```

3. Add an entity with both a [`DirectionalLight`](https://docs.rs/bevy/0.17.3/bevy/light/struct.DirectionalLight.html)
   and [`Sun`] components.
   ```rust
   commands.spawn((
       DirectionalLight::default(),
       Sun,
   ));
   ```

Now whenever you update the variables in `Environment` from any schedule, the light with the `Sun`
component attached will orient itself accordingly on the next frame.
