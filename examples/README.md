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
`minimal` | Demonstrates the bare minimum setup needed for the library to work. Renders some primitives on a flat plane with a fast-moving sun controlled by the library to show that it works.
