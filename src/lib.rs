//! `embedded-time` provides a comprehensive library for implementing [`Clock`] abstractions over
//! hardware to generate [`Instant`]s and using [`Duration`]s ([`Seconds`], [`Milliseconds`], etc)
//! in embedded systems. The approach is similar to the C++ `chrono` library. A [`Duration`]
//! consists of an integer (whose type is chosen by the user to be either [`i32`] or [`i64`]) as
//! well as a `const` ratio where the integer value multiplied by the ratio is the [`Duration`] in
//! seconds. Put another way, the ratio is the precision of the LSbit of the integer. This structure
//! avoids unnecessary arithmetic. For example, if the [`Duration`] type is [`Milliseconds`], a call
//! to the [`Duration::count()`] method simply returns the stored integer value directly which is
//! the number of milliseconds being represented. Conversion arithmetic is only performed when
//! explicitly converting between time units.
//!
//! [`Clock`]: trait.Clock.html
//! [`Instant`]: instant::Instant
//! [`Seconds`]: time_units::Seconds
//! [`Milliseconds`]: time_units::Milliseconds
//!
//! ## Definitions
//! **Clock**: Any entity that periodically counts (ie a hardware timer peripheral). Generally,
//! this needs to be monotonic. A wrapping clock is considered monotonic in this context as long as
//! it fulfills the other requirements.
//!
//! **Wrapping Clock**: A clock that when at its maximum value, the next count is the minimum
//! value.
//!
//! **Instant**: A specific instant in time ("time-point") returned by calling `Clock::now()`.
//!
//! **Duration**: The difference of two instances. The duration of time elapsed from one instant
//! until another. A span of time.
//!
//! ## Notes
//! Some parts of this crate were derived from various sources:
//! - [`RTFM`](https://github.com/rtfm-rs/cortex-m-rtfm)
//! - [`time`](https://docs.rs/time/latest/time) (Specifically the [`time::NumbericalDuration`](https://docs.rs/time/latest/time/trait.NumericalDuration.html)
//!   implementations for primitive integers)
//!
//! # Example Usage
//! ```rust,no_run
//! # use embedded_time::{prelude::*, time_units::*, instant::Instant, Period};
//! # #[derive(Debug)]
//! struct SomeClock;
//! impl embedded_time::Clock for SomeClock {
//!     type Rep = i64;
//!     const PERIOD: Period = Period::new_raw(1, 16_000_000);
//!
//!     fn now() -> Instant<Self> {
//!         // ...
//! #         unimplemented!()
//!     }
//! }
//!
//! let instant1 = SomeClock::now();
//! // ...
//! let instant2 = SomeClock::now();
//! assert!(instant1 < instant2);    // instant1 is *before* instant2
//!
//! // duration is the difference between the instances
//! let duration: Option<Microseconds<i64>> = instant2.duration_since(&instant1);    
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
pub use numerical_duration::TimeRep;

pub type Period = num::rational::Ratio<i32>;

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
    use crate::{Clock, Period};

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock64;
    impl Clock for MockClock64 {
        type Rep = i64;
        const PERIOD: Period = Period::new_raw(1, 64_000_000);

        fn now() -> Instant<Self> {
            Instant::new(128_000_000)
        }
    }

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock32;
    impl Clock for MockClock32 {
        type Rep = i32;
        const PERIOD: Period = Period::new_raw(1, 16_000_000);

        fn now() -> Instant<Self> {
            Instant::new(32_000_000)
        }
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
}
