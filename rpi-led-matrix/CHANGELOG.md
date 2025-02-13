# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Took latest clippy suggestions
- Link to pure rust rewrite of `rpi-led-matrix`
- dependabot updates of CI only crates
- Update `embedded-graphics-core` to `0.4` and `embedded-graphics` to `0.8`
- Implement Send + Sync for LedCanvas

## [0.4.0] - 2022-01-05

- Took a lot of clippy suggestions. Added `#[must_use]` where it made sense.
  Most of the reset was documentation changes and moving from `unwrap` to `expect`.
- Updated `clap` and `embedded-graphics` to the latest versions. `3` and `0.7` respectively.
- Added pass-through features to `rpi-led-matrix-sys` to enable static linking to `libstdc++`
  if requested via the feature `stdcpp-static-link`.
- Updated to 2021 edition.

## [0.3.1] - 2021-12-04

- Cosmetic README/documentation cleanups

## [0.3.0] - 2021-12-04

- Encapsulated the C++ library in a `-sys` crate

## [0.2.2] - 2020-09-25

- Version bump to (hopefully) actually fix [docs.rs][docs-rs-link] not rendering optional clap feature

## [0.2.1] - 2020-09-13

- Version bump to fix [docs.rs][docs-rs-link] not rendering optional clap feature

## [0.2.0] - 2020-09-13

### Added

- Implemented [embedded-graphics][embedded-graphics] support, behind a feature
- Command line parsing tools using clap
- Example usage of the library
- Documentation of the public API
- This CHANGELOG.md

### Changed

- Switched to new C++ API to enable full control of behavior from rust

## [0.1.5] - 2020-08-08

### Fixed

- Fixed segfault caused by improperly packed C structure

## [0.1.4] - 2020-08-08

### Changed

- Changed the [crates.io][crates-io-link] homepage to our [docs.rs][docs-rs-link] page

## [0.1.3] - 2020-08-08

### Added

- Optimizations for release builds
- GitHub Actions build CI

### Changed

- Replace `uint8_t` with `u8` per clippy lint
- Other clippy lint cleanups

## [0.1.2] - 2020-08-08

### Changed

- Redirected the [crates.io][crates-io-link] repository to its new home, [rust-rpi-led-matrix/rust-rpi-rgb-led-matrix][github-link]

## [0.1.1] - 2018-02-12

- Change LedCanvas to a struct

## [0.1.0] - 2018-02-12

- Initial release

[embedded-graphics]: https://github.com/jamwaffles/embedded-graphics/tree/master/embedded-graphics
[crates-io-link]: https://crates.io/crates/rpi-led-matrix
[docs-rs-link]: https://docs.rs/rpi-led-matrix/
[github-link]: https://github.com/rust-rpi-led-matrix/rust-rpi-rgb-led-matrix/