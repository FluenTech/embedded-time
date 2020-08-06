# Changelog

## [Unreleased]

[unreleased]: https://github.com/FluenTech/embedded-time/compare/v0.9.0...HEAD

## [0.9.0] - 2020-08-05

### Added

- `From` implementations for infallible intra-rate and intra-duration conversions
- Implementations of common traits to types
- More frequency types: mebihertz (MiHz) and kibihertz(KiHz)
- More test coverage

### Changed

- Fallible intra-rate and intra-duration conversions use `try_from()`/`try_into()` rather than the `try_convert_` methods
- Replaced `try_into_generic()` with `to_generic()`
- Replaced `try_from_`(duration/rate) with `to_duration()` and `to_rate()`
- Return `Option`s from `checked_` methods rather than `Result`s
- Remove impl-specific `Clock` error type (`ImplError`)

### Removed

- trait module
  - duration- and/or rate-specific functionality can be imported separately by using duration::* and rate::*

[0.9.0]: https://github.com/FluenTech/embedded-time/compare/v0.8.1...v0.9.0

## [0.8.1] - 2020-07-25

### Fixed

The way the conversions from duration to rate (`Rate::try_from_duration()`) and from rate to duration (`Duration::try_from_rate()`) were causing unnecessary arithmetic overflows and type conversion failures. These functions are now optimized similar to the other similar conversion algorithms.

[0.8.1]: https://github.com/FluenTech/embedded-time/compare/v0.8.0...v0.8.1

## [0.8.0] - 2020-07-24

### Added

- A full range of rate types implemented at the same level as the duration types (as a fixed-point value)

    | Units             | Extension |
    | :---------------- | :-------- |
    | Megahertz         | MHz       |
    | Kilohertz         | kHz       |
    | Hertz             | Hz        |
    |                   |           |
    | MebibytePerSecond | MiBps     |
    | MegabytePerSecond | MBps      |
    | KibibytePerSecond | KiBps     |
    | KiloBytePerSecond | KBps      |
    | BytePerSecond     | Bps       |
    |                   |           |
    | MebibitPerSecond  | Mibps     |
    | MegabitPerSecond  | Mbps      |
    | KibibitPerSecond  | Kibps     |
    | KilobitPerSecond  | kbps      |
    | BitPerSecond      | bps       |
    |                   |           |
    | Mebibaud          | MiBd      |
    | Megabaud          | MBd       |
    | Kibibaud          | KiBd      |
    | Kilobaud          | kBd       |
    | Baud              | Bd        |

- Conversion between duration and rate types

    ```rust
    Microseconds::<u32>::try_from_rate(Kilohertz(2_u32)) -> Ok(Microseconds(500_u32))
    Kilohertz::<u32>::try_from_duration(Microseconds(2_u32)) -> Ok(Kilohertz(500_u32))
    ```

- `Generic` duration/rate types returned by `Instant` methods that return a duration/rate and convertible to/from _named_ durations/rates

    ```rust
    Seconds(2_u64).try_into_generic(Fraction::new(1, 2_000)) -> Ok(Generic::new(4_000_u32, Fraction::new(1, 2_000))))
    Seconds::<u64>::try_from(Generic::new(2_000_u32, Fraction::new(1, 1_000))) -> Ok(Seconds(2_u64))

    Hertz(2_u64).try_into_generic(Fraction::new(1,2_000)) -> Ok(Generic::new(4_000_u32, Fraction::new(1,2_000))))
    Hertz::<u64>::try_from(Generic::new(2_000_u32, Fraction::new(1,1_000))) -> Ok(Hertz(2_u64))
    ```

### Changed

- Rename `frequency` module to `rate`
- Updated crate description
- The `const PERIOD`s in duration and clock traits is now `const SCALING_FACTOR`
- The `Period` type is renamed to `Fraction` and is no longer generic (now u32)

[0.8.0]: https://github.com/FluenTech/embedded-time/compare/v0.7.0...v0.8.0

## [0.7.0] - 2020-07-13

### Added

- Error propagation for all fallible paths including conversion errors as well as `Clock` implementation-specific errors
- Construct-and-read benchmark comparison to `core::time::duration` showing `embedded-time` to be approximately 2.5 times faster

### Fixed

- Intermittent `Timer` test failures
- Derive `Copy` for `Hertz`

[0.7.0]: https://github.com/FluenTech/embedded-time/compare/v0.6.0...v0.7.0

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
