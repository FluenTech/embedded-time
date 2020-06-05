# embedded-time

`embedded-time` provides a comprehensive library for implementing abstractions over
hardware and work with _instants_ and _durations_ in an intuitive way.
 
- `Clock` trait allowing abstraction of hardware timers/counters for timekeeping.
- Work with time using _milliseconds_, _seconds_, etc. rather than _cycles_ or _ticks_.
- Includes examples for the nRF52_DK development kit as bare-metal as well as using [`rtfm`](https://github.com/rtfm-rs/cortex-m-rtfm) (with patches)

## Example Usage
```rust
struct SomeClock;

impl Clock for SomeClock {
    type Rep = i64;

    // this clock is counting at 16 MHz
    const PERIOD: Period = Period::new_raw(1, 16_000_000);

    fn now() -> Instant<Self> {
        // read the count of the clock
        // ...
        Instant::new(count as Self::Rep)
    }
}

fn main() {
    // read from a Clock
    let instant1 = SomeClock::now();
    
    // ... some time passes
    
    let instant2 = SomeClock::now();
    assert!(instant1 < instant2);    // instant1 is *before* instant2
    
    // duration is the difference between the instances
    let duration: Option<Microseconds<i64>> = instant2.duration_since(&instant1);    
    
    // add some duration to an instant
    let future_instant = instant2 + Milliseconds(23);
    // or
    let future_instant = instant2 + 23.milliseconds();
    
    assert(future_instant > instant2);
}
```

## Motivation
The handling of time on embedded systems is generally much different than that of OSs. For instance, on an OS, the time is measured against an arbitrary epoch. Embedded systems generally don't know (nor do they care) what the *real* time is, but rather how much time has passed since the system has started.
 
### Drawbacks of the standard library types
#### Duration
- The storage is `u64` seconds and `u32` nanoseconds.
  - This is huge overkill and adds needless complexity beyond what is required (or desired) for embedded systems.
- Any read requires arithmetic to convert to the requested units
  - This is much slower than this project's implementation of what is analogous to a tagged union of time units.
#### Instant
- The `Instant` type requires `std`.

### Drawbacks of the [`time`](https://crates.io/crates/time) crate
The `time` crate is a remarkable library but isn't geared for embedded systems (although it does support a subset of features in `no_std` contexts). It suffers from some of the same drawbacks as the core::Duration type (namely the storage format) and the `Instant` struct dependency on `std`. It also adds a lot of functionally that would seldom be useful in an embedded context. For instance it has a comprehensive date/time formatting, timezone, and calendar support.

## Background
### What is an Instant?
In the Rust ecosystem, it appears to be idiomatic to call a `now()` associated function from an Instant type. There is generally no concept of a "Clock". I believe that using the `Instant` in this way is a violation of the *separation of concerns* principle. What is an `Instant`? Is it a time-keeping entity from which you read the current instant in time, or is it that instant in time itself. In this case, it's both.

As an alternative, the current instant in time could be read from a **Clock**. The `Instant` read from the `Clock` has the same precision and width (integer type) as the `Clock`. Requesting the difference between two `Instant`s gives a `Duration` which can have different precision and/or width.

## License
This project is licensed under either of
- [Apache License, Version 2.0](https://github.com/time-rs/time/blob/master/LICENSE-Apache)
- [MIT license](https://github.com/time-rs/time/blob/master/LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in time by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
