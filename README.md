# embedded-time

![Tests](https://github.com/PTaylor-FluenTech/embedded-time/workflows/Tests/badge.svg)
![Clippy](https://github.com/PTaylor-FluenTech/embedded-time/workflows/Clippy/badge.svg)

## Motivation
The handling of time on embedded systems is generally much different than that of OSs. For instance, on an OS, the time is measured against an arbitrary epoch. Embedded systems generally don't know (nor do they care) what the *real* time is, but rather how much time has passed since the system has started.

## Background
### Drawbacks of the core::Duration type
- The storage is `u64` seconds and `u32` nanoseconds.
  - This is huge overkill and adds needless complexity beyond what is required (or desired) for embedded systems.
- Any read requires arithmetic to convert to the requested units
  - This is much slower than this project's intended implementation of what may be anagolous to a tagged union of time units. For example, if your type is `Milliseconds`, a call to the `count()` method simply returns the stored value directly which is an integer representing the count of milliseconds. Conversion arithmetic is only performed when explicitly converting between time units.

### What is an Instant?
In the Rust ecosystem, it appears to be idiomatic to call a `now()` associated function from an Instant type. There is generally no concept of a "Clock". I believe that using the `Instant` in this way is a violation of the *separation of concerns* principle. What is an `Instant`? Is it a time-keeping entity from which you read the current instant in time, or is it that instant in time itself. In this case, it's both. It is much more intuitive to me to read the current instant in time from a _clock_. This allows for an implementation such that a clock can be defined as having ticks of some period, but the Instant read from it can have "ticks" (or precision) of some other period thereby breaking the Instant's dependency on the clock implementation.

## Intention
Provide a comprehensive library for implementing `Clock` abstractions over hardware to use with `Instant`s and using `Duration`s (`Seconds`, `Milliseconds`, etc) in embedded systems.

## Definitions
**Clock** - Any thing that periodically counts (ie a hardware timer peripheral). Generally, this needs to be monotonic. Here we are considering a wrapping timer as monotonic as long as it fulfills the other requirements.

**Wrapping Timer** - A timer that when at its maximum allowed value, wraps around to 0 on the next count.

**Instant** - A specific instant in time ("time-point") returned by calling `Clock::now()`. An `Instant` is also the result of an add/sub operation with a `Duration`.

**Duration** - The difference of two instances (the duration of time elapsed from one instant until another). 

## Notes

Some parts of this crate were derived from various sources:
- [`RTFM`](https://github.com/rtfm-rs/cortex-m-rtfm)
- [`time`](https://docs.rs/time/latest/time) (Specifically the [`time::NumbericalDuration`](https://docs.rs/time/latest/time/trait.NumericalDuration.html) implementations for primitive integers)

## License
This project is licensed under either of
- [Apache License, Version 2.0](https://github.com/time-rs/time/blob/master/LICENSE-Apache)
- [MIT license](https://github.com/time-rs/time/blob/master/LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in time by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
