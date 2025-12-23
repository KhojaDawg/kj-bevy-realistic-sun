# Usage Guide

See also: [`/examples/minimum.rs`](/examples/minimum.rs)

1. add the library to your `cargo.toml`
   ```
   kj_bevy_realistic_sun = { github = "https://github.com/KhojaDawg/kj-bevy-realistic-sun.git" }
   ```

2. Add the `RealisticSunDirectionPlugin` to your app's plugins

3. Add an entity to your scene with both Bevy's
   [`DirectionalLight`](https://docs.rs/bevy/0.17.3/bevy/light/struct.DirectionalLight.html)
   component and the `Sun` component from this library

4. Add an `Environment` resource to your scene. Set the values in this resource to the
   location/time of year of your game's setting

Update any of the values in the `Environment` resource and your sun light with its
attached `Sun` component will have its orientation updated on the next frame to
match the new values.

**Note:** all values in the `Environment` resource are *rotations* not timestamps or
datetimes or hours or days or anything else like that. The `time_of_day` variable represents how far
the earth has rotated from local solar noon, not an actual time of day in realistic hours: `0.0`
will always have the sun at its highest point in the sky and `PI` and `-PI` will always have the sun
rotated 180 degrees away from that highest point. Similarly, `time_of_year` represents how the axis
of the planet is rotated with respect to the sun, not an actual date of the year. A value of `0.0`
will always have the sun follow its highest possible arc across the sky for your given latitude and
axial tilt, and `PI`/`-PI` will always have the sun follow its lowest possible arc.

This makes it easy to create simple looping values to feed into the environment, or to define your
own date and time system that can be as simple or as complex as your game needs it to be, then
normalize those values to a range between `-PI` and `PI` and plug that
into the `Environment`.
