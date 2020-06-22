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
//! [`Seconds`]: units::Seconds
//! [`Milliseconds`]: units::Milliseconds
//!
//! ## Definitions
//! **Clock**: Any entity that periodically counts (ie a hardware timer peripheral). Generally,
//! this needs to be monotonic. A wrapping clock is considered monotonic in this context as long as
//! it fulfills the other requirements.
//!
//! **Wrapping Clock**: A clock that when at its maximum value, the next count is the minimum
//! value.
//!
//! **Instant**: A specific instant in time ("time-point") read from a clock.
//!
//! **Duration**: The difference of two instances. The time that has elapsed since an instant. A
//! span of time.
//!
//! ## Notes
//! Some parts of this crate were derived from various sources:
//! - [`RTIC`](https://github.com/rtic-rs/cortex-m-rtic)
//! - [`time`](https://docs.rs/time/latest/time) (Specifically the [`time::NumbericalDuration`](https://docs.rs/time/latest/time/trait.NumericalDuration.html)
//!   implementations for primitive integers)
//!
//! # Example Usage
//! ```rust,no_run
//! # use embedded_time::{prelude::*, units::*, Instant, Period};
//! # #[derive(Debug)]
//! struct SomeClock;
//! impl embedded_time::Clock for SomeClock {
//!     type Rep = i64;
//!     const PERIOD: Period = Period::new(1, 16_000_000);
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

#![deny(unsafe_code)]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
#![deny(intra_doc_link_resolution_failure)]

mod clock;
mod duration;
mod frequency;
mod instant;
mod period;
mod time_int;

pub use clock::Clock;
pub use duration::Duration;
pub use instant::Instant;
pub use period::Period;
pub use time_int::TimeInt;

/// Public _traits_
///
/// ```rust,no_run
/// use embedded_time::prelude::*;
/// ```
pub mod prelude {
    // Rename traits to `_` to avoid any potential name conflicts.
    pub use crate::duration::Duration as _;
    pub use crate::duration::TryConvertFrom as _;
    pub use crate::duration::TryConvertInto as _;
    pub use crate::time_int::TimeInt as _;
    pub use crate::Clock as _;
}

pub mod units {
    //! Time-based units of measure ([`Milliseconds`], [`Hertz`], etc)
    pub use crate::duration::units::*;
    pub use crate::frequency::units::*;
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate as time;
    use crate::prelude::*;
    use crate::units::*;
    use core::fmt::{self, Formatter};

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock64;

    impl time::Clock for MockClock64 {
        type Rep = i64;
        const PERIOD: time::Period = time::Period::new(1, 64_000_000);

        fn now() -> time::Instant<Self> {
            time::Instant::new(128_000_000)
        }
    }

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock32;

    impl time::Clock for MockClock32 {
        type Rep = i32;
        const PERIOD: time::Period = time::Period::new(1, 16_000_000);

        fn now() -> time::Instant<Self> {
            time::Instant::new(32_000_000)
        }
    }

    fn get_time<M>()
    where
        M: time::Clock,
    {
        assert_eq!(M::now().duration_since_epoch(), Some(Seconds(2)));
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

    struct Timestamp<Clock>(time::Instant<Clock>)
    where
        Clock: time::Clock;

    impl<Clock> Timestamp<Clock>
    where
        Clock: time::Clock,
    {
        pub fn new(instant: time::Instant<Clock>) -> Self {
            Timestamp(instant)
        }
    }

    impl<Clock> fmt::Display for Timestamp<Clock>
    where
        Clock: time::Clock,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let duration = self
                .0
                .duration_since_epoch::<Milliseconds<i64>>()
                .ok_or(fmt::Error {})?;

            let hours = Hours::<i32>::try_convert_from(duration).ok_or(fmt::Error {})?;
            let minutes =
                Minutes::<i32>::try_convert_from(duration).ok_or(fmt::Error {})? % Hours(1);
            let seconds =
                Seconds::<i32>::try_convert_from(duration).ok_or(fmt::Error {})? % Minutes(1);
            let milliseconds =
                Milliseconds::<i32>::try_convert_from(duration).ok_or(fmt::Error {})? % Seconds(1);

            f.write_fmt(format_args!(
                "{}:{:02}:{:02}.{:03}",
                hours, minutes, seconds, milliseconds
            ))
        }
    }

    #[test]
    fn format() {
        let timestamp = Timestamp::new(time::Instant::<MockClock64>::new(321_643_392_000));
        let formatted_timestamp = timestamp.to_string();
        assert_eq!(formatted_timestamp, "1:23:45.678");
    }
}
