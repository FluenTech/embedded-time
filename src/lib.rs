//! `embedded-time` provides a comprehensive library for implementing [`Clock`] abstractions over
//! hardware to generate [`Instant`]s and using [`Duration`]s ([`Seconds`], [`Milliseconds`], etc)
//! in embedded systems. The approach is similar to the C++ `chrono` library. A [`Duration`]
//! consists of an integer (whose type is chosen by the user to be either [`u32`] or [`u64`]) as
//! well as a `const` fraction ([`Period`]) where the integer value multiplied by the fraction is
//! the [`Duration`] in seconds. Put another way, the ratio is the precision of the LSbit of the
//! integer. This structure avoids unnecessary arithmetic. For example, if the [`Duration`] type is
//! [`Milliseconds`], a call to the [`Duration::count()`] method simply returns the stored integer
//! value directly which is the number of milliseconds being represented. Conversion arithmetic is
//! only performed when explicitly converting between time units.
//!
//! In addition frequency-type types are available including [`Hertz`] ([`u32`]) and it's reciprocal
//! [`Period`] ([`u32`]/[`u32`] seconds).
//!
//! [`Seconds`]: units::Seconds
//! [`Milliseconds`]: units::Milliseconds
//! [`Hertz`]: units::Hertz
//! [`Duration`]: trait.Duration.html
//! [`Clock`]: trait.Clock.html
//!
//! ## Definitions
//! **Clock**: Any entity that periodically counts (ie a hardware timer/counter peripheral).
//! Generally, this needs to be monotonic. A wrapping clock is considered monotonic in this context
//! as long as it fulfills the other requirements.
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
//! # use embedded_time::{traits::*, units::*, Instant, Period};
//! # #[derive(Debug)]
//! struct SomeClock;
//! impl embedded_time::Clock for SomeClock {
//!     type Rep = u64;
//!     const PERIOD: Period = <Period>::new(1, 16_000_000);
//!
//!     fn now(&mut self) -> Instant<Self> {
//!         // ...
//! #         unimplemented!()
//!     }
//! }
//!
//! let mut clock = SomeClock;
//! let instant1 = clock.now();
//! // ...
//! let instant2 = clock.now();
//! assert!(instant1 < instant2);    // instant1 is *before* instant2
//!
//! // duration is the difference between the instances
//! let duration: Result<Microseconds<u64>, _> = instant2.duration_since(&instant1);    
//!
//! assert!(duration.is_ok());
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
/// use embedded_time::traits::*;
/// ```
pub mod traits {
    // Rename traits to `_` to avoid any potential name conflicts.
    pub use crate::clock::Clock as _;
    pub use crate::duration::Duration as _;
    pub use crate::duration::TryConvertFrom as _;
    pub use crate::duration::TryConvertInto as _;
    pub use crate::time_int::NumericConstructor as _;
    pub use crate::time_int::TimeInt as _;
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
    use core::{
        convert::TryFrom,
        convert::TryInto,
        fmt::{self, Formatter},
    };
    use time::{traits::*, units::*};

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock64;

    impl time::Clock for MockClock64 {
        type Rep = u64;
        const PERIOD: time::Period = <time::Period>::new(1, 64_000_000);

        fn now(&mut self) -> time::Instant<Self> {
            time::Instant::new(128_000_000)
        }
    }

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock32;

    impl time::Clock for MockClock32 {
        type Rep = u32;
        const PERIOD: time::Period = <time::Period>::new(1, 16_000_000);

        fn now(&mut self) -> time::Instant<Self> {
            time::Instant::new(32_000_000)
        }
    }

    fn get_time<Clock: time::Clock>(clock: &mut Clock)
    where
        u32: TryFrom<Clock::Rep>,
        Clock::Rep: TryFrom<u32>,
    {
        assert_eq!(clock.now().duration_since_epoch(), Ok(Seconds(2_u32)));
    }

    #[test]
    fn common_types() {
        let then = MockClock32.now();
        let now = MockClock32.now();

        let mut clock64 = MockClock64 {};
        let mut clock32 = MockClock32 {};

        get_time(&mut clock64);
        get_time(&mut clock32);

        let then = then - Seconds(1_u32);
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
                .duration_since_epoch::<Milliseconds<u64>>()
                .map_err(|_| fmt::Error {})?;

            let hours = Hours::<u32>::try_convert_from(duration).ok_or(fmt::Error {})?;
            let minutes =
                Minutes::<u32>::try_convert_from(duration).ok_or(fmt::Error {})? % Hours(1_u32);
            let seconds =
                Seconds::<u32>::try_convert_from(duration).ok_or(fmt::Error {})? % Minutes(1_u32);
            let milliseconds = Milliseconds::<u32>::try_convert_from(duration)
                .ok_or(fmt::Error {})?
                % Seconds(1_u32);

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
