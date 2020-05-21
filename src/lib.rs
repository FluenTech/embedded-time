//! # Embedded Time
//! `embedded-time` provides a comprehensive library for implementing [`Clock`] abstractions over
//! hardware to generate [`Instant`]s and using [`Duration`]s ([`Seconds`], [`Milliseconds`], etc) in
//! embedded systems. The approach is similar to the C++ `chrono` library. A [`Duration`] consists of
//! an integer value (chosen by the user from either i32 or i64) as well as a const ratio where the
//! integer value multiplied by the ratio is the seconds represented by the [`Duration`]. Put another
//! way, the ratio is the precision of the LSbit of the integer. This structure avoids unnecessary
//! arithmetic. For example, if the [`Duration`] type is [`Milliseconds`], a call to the [`Duration::count()`]
//! method simply returns the stored integer value directly which is the number of milliseconds
//! being represented. Conversion arithmetic is only performed when explicitly converting between
//! time units.
//!
//! [`Clock`]: trait.Clock.html
//! [`Instant`]: instant::Instant
//! [`Seconds`]: time_units::Seconds
//! [`Milliseconds`]: time_units::Milliseconds
//!
//! ## Motivation
//! The handling of time on embedded systems is generally much different than that of OSs. For
//! instance, on an OS, the time is measured against an arbitrary epoch. Embedded systems generally
//! don't know (nor do they care) what the *real* time is, but rather how much time has passed since
//! the system has started.
//!
//! ## Background
//! ### Drawbacks of the core::Duration type
//! - The storage is `u64` seconds and `u32` nanoseconds.
//!   - This is huge overkill and adds needless complexity beyond what is required (or desired) for embedded systems.
//! - Any read requires arithmetic to convert to the requested units
//!   - This is much slower than this project's implementation of what is analogous to a tagged union of time units.
//!
//! ### What is an Instant?
//! In the Rust ecosystem, it appears to be idiomatic to call a `now()` associated function from an Instant type. There is generally no concept of a "Clock". I believe that using the `Instant` in this way is a violation of the *separation of concerns* principle. What is an `Instant`? Is it a time-keeping entity from which you read the current instant in time, or is it that instant in time itself. In this case, it's both.
//!
//! As an alternative, the current instant in time could be read from a **Clock**. The `Instant` read from the `Clock` has the same precision and width (integer type) as the `Clock`. Requesting the difference between two `Instant`s gives a `Duration` which can have different precision and/or width.
//!
//! ## Definitions
//! **Clock** - Any entity that periodically counts (ie a hardware timer peripheral). Generally, this needs to be monotonic. Here a wrapping timer is considered monotonic as long as it fulfills the other requirements.
//!
//! **Wrapping Timer** - A timer that when at its maximum value, the next count is the minimum value.
//!
//! **Instant** - A specific instant in time ("time-point") returned by calling `Clock::now()`.
//!
//! **Duration** - The difference of two instances (the duration of time elapsed from one instant until another).
//!
//! ## Notes
//! Some parts of this crate were derived from various sources:
//! - [`RTFM`](https://github.com/rtfm-rs/cortex-m-rtfm)
//! - [`time`](https://docs.rs/time/latest/time) (Specifically the [`time::NumbericalDuration`](https://docs.rs/time/latest/time/trait.NumericalDuration.html) implementations for primitive integers)
//!
//! # Example Usage
//! ```rust,no_run
//! # use embedded_time::{prelude::*, time_units::*, Ratio, instant::Instant};
//! # #[derive(Debug)]
//! struct SomeClock;
//! impl embedded_time::Clock for SomeClock {
//!     type Rep = i64;
//!
//!     fn now() -> Instant<Self> {
//!         // ...
//! #         unimplemented!()
//!     }
//! }
//!
//! impl embedded_time::Period for SomeClock {
//!     const PERIOD: Ratio<i32> = Ratio::<i32>::new_raw(1, 16_000_000);
//! }
//!
//! let instant1 = SomeClock::now();
//! // ...
//! let instant2 = SomeClock::now();
//! assert!(instant1 < instant2);    // instant1 is *before* instant2
//!
//! // duration is the difference between the instances
//! let duration: Option<Microseconds<i64>> = instant2.elapsed_since(&instant1);    
//!
//! assert!(duration.is_some());
//! assert_eq!(instant1 + duration.unwrap(), instant2);
//! ```

#![cfg_attr(not(test), no_std)]
#![feature(associated_type_bounds)]
#![deny(intra_doc_link_resolution_failure)]

mod clock;
pub mod duration;
pub mod instant;
mod numerical_duration;

pub use clock::Clock;
pub use duration::{time_units, Duration};
pub use num::rational::Ratio;
pub use numerical_duration::TimeRep;

pub trait Period {
    const PERIOD: Ratio<i32>;
}

/// A collection of imports that are widely useful.
///
/// Unlike the standard library, this must be explicitly imported:
///
/// ```rust,no_run
/// use embedded_time::prelude::*;
/// ```
/// The prelude may grow in minor releases. Any removals will only occur in
/// major releases.
pub mod prelude {
    // Rename traits to `_` to avoid any potential name conflicts.
    pub use crate::duration::Duration as _;
    pub use crate::duration::TryConvertFrom as _;
    pub use crate::duration::TryConvertInto as _;
    pub use crate::numerical_duration::TimeRep as _;
    pub use crate::Clock as _;
    pub use crate::Period as _;
    pub use num::Integer as _;
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::instant::Instant;
    use crate::prelude::*;
    use crate::time_units::*;
    use crate::Ratio;
    use crate::{Clock, Period};

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock64;
    impl Clock for MockClock64 {
        type Rep = i64;

        fn now() -> Instant<Self> {
            Instant::new(128_000_000)
        }
    }
    impl Period for MockClock64 {
        const PERIOD: Ratio<i32> = Ratio::new_raw(1, 64_000_000);
    }

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock32;
    impl Clock for MockClock32 {
        type Rep = i32;

        fn now() -> Instant<Self> {
            Instant::new(32_000_000)
        }
    }
    impl Period for MockClock32 {
        const PERIOD: Ratio<i32> = Ratio::new_raw(1, 16_000_000);
    }

    fn get_time<M>()
    where
        M: Clock,
    {
        assert_eq!(M::now().elapsed_since_epoch(), Some(Seconds(2)));
    }

    #[test]
    fn common_types() {
        let then = MockClock32::now();
        let now = MockClock32::now();

        get_time::<MockClock64>();
        get_time::<MockClock32>();

        let then = then - Seconds(1);
        assert_ne!(then, now);
        assert!(then < now);
    }

    #[test]
    fn brute_force() {
        let mut time = 1_i64;
        time *= 60;
        assert_eq!(Hours(1), Minutes(time));
        time *= 60;
        assert_eq!(Hours(1), Seconds(time));
        time *= 1000;
        assert_eq!(Hours(1), Milliseconds(time));
        time *= 1000;
        assert_eq!(Hours(1), Microseconds(time));
        time *= 1000;
        assert_eq!(Hours(1), Nanoseconds(time));
    }
}
