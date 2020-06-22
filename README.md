# embedded-time &emsp; ![CI] [![crates.io]](https://crates.io/crates/embedded-time) [![docs.rs]](https://docs.rs/embedded-time)

[CI]: https://github.com/FluenTech/embedded-time/workflows/CI/badge.svg
[crates.io]: https://img.shields.io/crates/v/embedded-time.svg
[docs.rs]: https://docs.rs/embedded-time/badge.svg

`embedded-time` provides a comprehensive library for implementing abstractions over
hardware and work with _clocks_, _instants_, _durations_, _periods_, and _frequencies_ in a more intuitive way.
 
- `Clock` trait allowing abstraction of hardware timers for timekeeping.
- Work with time using _milliseconds_, _seconds_, _hertz_, etc. rather than _cycles_ or _ticks_.
- Includes example for the nRF52_DK board

## License
This project is licensed under either of
- [Apache License, Version 2.0](https://github.com/time-rs/time/blob/master/LICENSE-Apache)
- [MIT license](https://github.com/time-rs/time/blob/master/LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in time by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
