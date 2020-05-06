# embedded-time

![Tests](https://github.com/PTaylor-FluenTech/embedded-time/workflows/Tests/badge.svg)

## Motivation
The handling of time on embedded systems is generally much different than that of OSs. For instance, on an OS, the time is measured against an arbitrary epoch. Embedded systems generally don't know (nor do they care) what the *real* time is, but rather how much time has passed since the system has started.

## Intention
Provide a comprehensive library for implementing `Clock`s to use with `Instant`s and using `Duration`s (`Seconds`, `Milliseconds`, etc) in embedded systems.

## Definitions
**Clock** - A trait for abstracting over the hardware clock(s)/timer(s) used.

**Instant** - A specific instant in time ("time-point") returned by calling `Clock::now()`. An `Instant` is also the result of an add/sub operation with a `Duration`.

**Duration** - The difference of two instances. 

## Notes
The `Duration` type is signed (unlike the `std::time::Duration` type). This is intentional as it eliminates the need for the sign checking necessary in the `std` implementation and also is more intuitive to work with.

Many parts of this repo were derived from various sources:
- [`time`](https://docs.rs/time/latest/time) (Specifically the [`time::NumbericalDuration`](https://docs.rs/time/latest/time/trait.NumericalDuration.html) implementations for primitive integers)

## License
This project is licensed under either of
- [Apache License, Version 2.0](https://github.com/time-rs/time/blob/master/LICENSE-Apache)
- [MIT license](https://github.com/time-rs/time/blob/master/LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in time by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
