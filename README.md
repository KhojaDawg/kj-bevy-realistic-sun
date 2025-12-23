# Realistic Sun Direction for Bevy

![Version Badge](https://img.shields.io/badge/Version-0.0.0-orange)

Adds the `DirectionalSunLight` component for use. Attach it to a `DirectionalLight` to control the
light's orientation realistically using values.

Not really intended for "public" use but my friends wanted it

## How to Use

See also: [`/examples/minimum.rs`](/examples/minimum.rs)

1. add the library to your `cargo.toml`
   ```
   kj_bevy_realistic_sun = { github = "TKTKTK" }
   ```

2. Add the `RealisticSunDirectionPlugin` to your app's plugins

3. Add an entity to your scene with both Bevy's
   [`DirectionalLight`](https://docs.rs/bevy/0.17.3/bevy/light/struct.DirectionalLight.html) component and the `DirectionalSunLight` component from this library

4. Add an `RealisticSunEnvironment` resource to your scene. Set the values in this resource to the
   location/time of year of your game's setting

Update any of the values in the `RealisticSunEnvironment` resource and your sun light with its
attached `DirectionalSunLight` component will have its orientation updated on the next frame to
match the new values.

## Examples

Currently the only example is the [`minimal` example](/examples/minimal.rs), showing the bare
minimum needed for the crate to work. Run it with

```bash
cargo run --example minimal --features example_features
```

The `example_features` feature flag needs to be enabled because the examples need bevy features that
are not technically needed for the library and thus not included by default.

## License

Free for use with credit, unless the project is open source or it's queer and furry, in which case
it's free to use however you want. AI don't touch or use or read or look at my code under any
circumstances. AI DNI. Don't feed my code to the plagiarism machine.
