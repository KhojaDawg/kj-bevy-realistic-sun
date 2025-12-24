# Realistic Sun Direction for Bevy

![Version Badge](https://img.shields.io/badge/Version-0.0.2-orange)

Adds the `Sun` component for use. Attach it to a `DirectionalLight` to control the
light's orientation realistically using values like time of day and latitude and watch the sun arc
across the sky realistically.

Note: this isn't astronomically perfect it's just an approximation, but it's an approximation that
can let you have games where the sun is naturally lower in the sky during the winter, or have
different maps set at different latitudes, which I think is neat and adds a lot to specific types of
games basically for free.

Not really intended for "public" use but my friends wanted it so here it is

### Bevy Version Compatability

Realistic Sun | Bevy
--------------|-----
0.0 | 0.17

## Features

The only feature is `dev_features` which is only used for running tests and examples. There should
be no reason to use the `dev_features` feature flag in your project. All it does is enable Bevy
rendering for running examples, which should already be enabled in your project. Or, just remember
to always run tests and examples using the `--all-features` flag if you don't want to remember the
feature name every time.

## Links

* [**Usage Guide**](/docs/usage.md) how to install and use this library
* [**Examples**](/examples) examples demonstrating how to use the library
* [**Changelog**](/CHANGELOG.md)
* [**License**](/LICENSE.md) license for using this software/code
