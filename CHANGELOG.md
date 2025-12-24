# Changelog

Follows semantic versioning. Pre-release and Release-candidate versions not included directly in
the changelog

## Log

### 2025-12-23 `v0.0.2`

Fixed some things I missed for v0.0.1, all documentation related

* version badge in README is up to date
* update [`usage.md`](/docs/usage.md) for version 0.0.2, fix outdated info/links and update version number
* move license to [`LICENSE.md`](/license.md) in package root
* add changelog
* add link to changelog in readme


### 2025-12-23 `v0.0.1`

Initial release. Contains barebones tools needed to manage a realistically moving sun with this library.

* **`Environment`** resource - input the values that control the sun position
* **`Sun`** component - attach it to your directional light for the sun
* **`RealisticSunDirectionPlugin`** - sets up a default Environment and the system that updates Sun entities using the values stored in `Environment`
