# Changelog

## [Unreleased]

### Added

- Error propagation to all fallible paths including conversion errors as well as `Clock` implementation-specific errors
- Construct-and-read benchmark comparison to `core::time::duration` showing `embedded-time` to be approximately 2.5 times faster

### Fixed

- Intermittent `Timer` test failures

[unreleased]: https://github.com/FluenTech/embedded-time/compare/v0.6.0...HEAD

## [0.6.0] - 2020-07-03

### Added

- A `Timer` type supporting one-shot and periodic software timers utilizing a `Clock` implementation
- Fallibility and error handling for `Clock` methods
- `Instant::duration_until()` with order checking
- Order checking to `Instant::duration_since()`
- Bounds checking on `Instant` impls of Add/Sub
- Changelog back to v0.5.0 release
- [`crossbeam-utils`](https://crates.io/crates/crossbeam-utils) dev-dependency for scoped threads in tests

### Changed

- Add `&self` to `Clock` functions (make stateful, or at least allow stateful implementations)
- All time-type inner types from signed to unsigned
- `Instant::duration_since()` return type to `Result`
- Refactor `examples/nrf52_dk`

[0.6.0]: https://github.com/FluenTech/embedded-time/compare/v0.5.2...v0.6.0

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
