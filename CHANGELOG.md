# Changelog

## [Unreleased]

### Added

- Add fallibility to `Clock` methods
- Changelog back to v0.5.0 release

### Changed

- Add `&mut self` to `Clock` functions (make stateful, or at least allow stateful implementations)
- Refactor `examples/nrf52_dk`

[unreleased]: https://github.com/FluenTech/embedded-time/compare/v0.5.2...HEAD


## [0.5.2] - 2020-06-21

### Added

- Ability to convert to/from [`core::time::Duration`](https://doc.rust-lang.org/stable/core/time/struct.Duration.html)
- Missing documentation

### Changed

- Moved majority of `Duration`-related documentation to `Duration` trait
- Minor refactoring

[0.5.2]: https://github.com/FluenTech/embedded-time/compare/v0.5.1...v0.5.2


## [0.5.1] - 2020-06-21

### Changed

- Repository location

### Removed

- `Period` from `prelude` mod as it is no longer a trait

[0.5.1]: https://github.com/FluenTech/embedded-time/compare/v0.5.0...v0.5.1


## [0.5.0] - 2020-06-17

### Added

- `cargo doc` CI test
- Frequency-based type (`Hertz`) with conversion to/from `Period`
- CI tests for `stable`

### Changed

- Rename `duration::time_units` to `duration::units` (`units` is also re-exported)
- Rename `TimeRep` to `TimeInt`
- Update `num` to v0.3.x
- Make `Period` a struct that wraps a `Ratio`, rather than a trait 

### Removed

- `associated_type_bounds` feature flag to allow `stable` build
- Re-export of the `duration` module (wasn't useful)

[0.5.0]: https://github.com/FluenTech/embedded-time/compare/v0.4.0...v0.5.0
