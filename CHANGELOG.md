# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2020-09-13

### Added

- @BroderickCarlin implemented [embedded-graphics][embedded-graphics] support, behind a feature
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

-

## [0.1.0] - 2018-02-01

- Initial release

[embedded-graphics]: https://github.com/jamwaffles/embedded-graphics/tree/master/embedded-graphics
[crates-io-link]: https://crates.io/crates/rpi-led-matrix
[docs-rs-link]: https://docs.rs/rpi-led-matrix/
[github-link]: https://github.com/rust-rpi-led-matrix/rust-rpi-rgb-led-matrix/