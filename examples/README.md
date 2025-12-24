## Examples

Run the following command with the example name

```bash
cargo run --example <example_name> --all-features
```

For example: to run the `minimal` example ([`examples/minimal.rs`](/examples/minimal.rs)):

```bash
cargo run --example minimal --all-features
```

The `dev_features` feature flag *must* be enabled in order for examples to run, so either
`--all-features` or `--features "dev_features"` *must* be included when running examples. 

### Examples List

name | description
-----|------------
[`minimal`](minimal.rs) | Demonstrates the bare minimum setup needed for the library to work. Renders some primitives on a flat plane with a fast-moving sun controlled by the library to show that it works.
[`control`](control.rs) | Complex example with direct control over the `Environment` parameters and a display showing their current values, so you can see how the light behavior changes with different values. Try setting the latitude very close to maximum or minimum and seeing how the sun moves through the sky at different times of year
[`cage`](cage.rs) | Similar to `control` example but instead of seeing the effect the light has on objects, you can see an interactive graph of how the sun will move through the sky with the current settings
