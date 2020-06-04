# embedded-time &emsp; ![CI] [![crates.io]](https://crates.io/crates/embedded-time) [![docs.rs]](https://docs.rs/embedded-time)

[CI]: https://github.com/PTaylor-FluenTech/embedded-time/workflows/CI/badge.svg
[crates.io]: https://img.shields.io/crates/v/embedded-time.svg
[docs.rs]: https://docs.rs/embeded-time/badge.svg

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

## License
This project is licensed under either of
- [Apache License, Version 2.0](https://github.com/time-rs/time/blob/master/LICENSE-Apache)
- [MIT license](https://github.com/time-rs/time/blob/master/LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in time by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
