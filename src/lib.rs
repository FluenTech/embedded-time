//! `embedded-time` provides a comprehensive library of [`Duration`] and [`Rate`] types as well as
//! a [`Clock`] abstractions for hardware timers/clocks and the associated [`Instant`] type for
//! in embedded systems.
//!
//! Additionally, an implementation of software timers is provided that work seemlessly with all
//! the types in this crate.
//!
//! The approach taken is similar to the C++ `chrono` library. [`Duration`]s and [`Rate`]s are
//! fixed-point values as in they are comprised of _integer_ and _scaling factor_ values.
//! The _scaling factor_ is a `const` [`Fraction`]. One benefit of this structure is that it avoids
//! unnecessary arithmetic. For example, if the [`Duration`] type is
//! [`Milliseconds`], a call to the [`Duration::integer()`] method simply returns the _integer_
//! part directly which in the case is the number of milliseconds represented by the [`Duration`].
//! Conversion arithmetic is only performed when explicitly converting between time units (eg.
//! [`Milliseconds`] --> [`Seconds`]).
//!
//! In addition, a wide range of rate-type types are available including [`Hertz`],
//! [`BitsPerSecond`], [`Baud`], etc.
//!
//! A [`Duration`] type can be converted to a [`Rate`] type and vica-versa.
//!
//! [`Seconds`]: duration::units::Seconds
//! [`Milliseconds`]: duration::units::Milliseconds
//! [`Rate`]: rate::Rate
//! [`Hertz`]: rate::units::Hertz
//! [`BitsPerSecond`]: rate::units::BitsPerSecond
//! [`Baud`]: rate::units::Baud
//! [`Duration`]: duration::Duration
//! [`Duration::integer()`]: duration/trait.Duration.html#tymethod.integer
//!
//! ## Definitions
//!
//! **Clock**: Any entity that periodically counts (ie an external or peripheral hardware
//! timer/counter). Generally, this needs to be monotonic. A wrapping clock is considered monotonic
//! in this context as long as it fulfills the other requirements.
//!
//! **Wrapping Clock**: A clock that when at its maximum value, the next count is the minimum
//! value.
//!
//! **Timer**: An entity that counts toward an expiration.
//!
//! **Instant**: A specific instant in time ("time-point") read from a clock.
//!
//! **Duration**: The difference of two instants. The time that has elapsed since an instant. A
//! span of time.
//!
//! **Rate**: A measure of events per time such as frequency, data-rate, etc.
//!
//! ## Notes
//! Some parts of this crate were derived from various sources:
//! - [`RTIC`](https://github.com/rtic-rs/cortex-m-rtic)
//! - [`time`](https://docs.rs/time/latest/time) (Specifically the [`time::NumbericalDuration`](https://docs.rs/time/latest/time/trait.NumericalDuration.html)
//!   implementations for primitive integers)
//!
//! # Example Usage
//! ```rust,no_run
//! # use embedded_time::{prelude::*, duration::units::*, rate::units::*, Instant, Fraction};
//! # use core::convert::TryFrom;
//! # #[derive(Debug)]
//! struct SomeClock;
//! impl embedded_time::Clock for SomeClock {
//!     type T = u64;
//!     type ImplError = ();
//!     const SCALING_FACTOR: Fraction = Fraction::new(1, 16_000_000);
//!
//!     fn try_now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {
//!         // ...
//! #         unimplemented!()
//!     }
//! }
//!
//! let mut clock = SomeClock;
//! let instant1 = clock.try_now().unwrap();
//! // ...
//! let instant2 = clock.try_now().unwrap();
//! assert!(instant1 < instant2);    // instant1 is *before* instant2
//!
//! // duration is the difference between the instants
//! let duration = instant2.duration_since(&instant1);
//! assert!(duration.is_ok());
//!
//! // convert to _named_ duration
//! let duration = Microseconds::<u64>::try_from(duration.unwrap());
//! assert_eq!(instant1 + duration.unwrap(), instant2);
//! ```

#![deny(unsafe_code)]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
#![deny(intra_doc_link_resolution_failure)]

pub mod clock;
pub mod duration;
mod fixed_point;
mod fraction;
mod instant;
pub mod rate;
mod time_int;
mod timer;

pub use clock::Clock;
pub use duration::Duration;
pub use fixed_point::FixedPoint;
pub use fraction::Fraction;
pub use instant::Instant;
pub use rate::Rate;
pub use time_int::TimeInt;
pub use timer::Timer;

/// Public _traits_
///
/// ```rust,no_run
/// use embedded_time::prelude::*;
/// ```
pub mod prelude {
    // Rename traits to `_` to avoid any potential name conflicts.
    pub use crate::clock::Clock as _;
    pub use crate::duration::units::Extensions as _;
    pub use crate::duration::Duration as _;
    pub use crate::fixed_point::FixedPoint as _;
    pub use crate::rate::units::Extensions as _;
    pub use crate::rate::Rate as _;
    pub use crate::time_int::TimeInt as _;
}

/// Crate errors
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum TimeError<E> {
    /// Attempted type conversion failed
    ConversionFailure,
    /// Result is outside of those valid for this type
    Overflow,
    /// Attempted to divide by zero
    DivByZero,
    /// Resulting [`Duration`](duration/trait.Duration.html) is negative (not allowed)
    NegDuration,
    /// [`Clock`]-implementation-specific error
    Clock(clock::Error<E>),
}

impl<E> From<clock::Error<E>> for TimeError<E> {
    fn from(clock_error: clock::Error<E>) -> Self {
        TimeError::<E>::Clock(clock_error)
    }
}

/// Conversion errors
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum ConversionError {
    /// Attempted type conversion failed
    ConversionFailure,
    /// Result is outside of those valid for this type
    Overflow,
    /// Attempted to divide by zero
    DivByZero,
    /// Resulting [`Duration`](duration/trait.Duration.html) is negative (not allowed)
    NegDuration,
}

impl<E> From<ConversionError> for TimeError<E> {
    fn from(error: ConversionError) -> Self {
        match error {
            ConversionError::ConversionFailure => TimeError::ConversionFailure,
            ConversionError::Overflow => TimeError::Overflow,
            ConversionError::DivByZero => TimeError::DivByZero,
            ConversionError::NegDuration => TimeError::NegDuration,
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::{self as time, clock, duration::units::*, prelude::*, rate::units::*};
    use core::{
        convert::{Infallible, TryFrom, TryInto},
        fmt::{self, Formatter},
    };

    struct MockClock64;
    impl time::Clock for MockClock64 {
        type T = u64;
        type ImplError = Infallible;
        const SCALING_FACTOR: time::Fraction = <time::Fraction>::new(1, 64_000_000);

        fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error<Self::ImplError>> {
            Ok(time::Instant::new(128_000_000))
        }
    }

    #[derive(Debug)]
    struct MockClock32;

    impl time::Clock for MockClock32 {
        type T = u32;
        type ImplError = Infallible;
        const SCALING_FACTOR: time::Fraction = <time::Fraction>::new(1, 16_000_000);

        fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error<Self::ImplError>> {
            Ok(time::Instant::new(32_000_000))
        }
    }

    #[non_exhaustive]
    #[derive(Debug, Eq, PartialEq)]
    pub enum ClockImplError {
        NotStarted,
    }

    #[derive(Debug)]
    struct BadClock;

    impl time::Clock for BadClock {
        type T = u32;
        type ImplError = ClockImplError;
        const SCALING_FACTOR: time::Fraction = <time::Fraction>::new(1, 16_000_000);

        fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error<Self::ImplError>> {
            Err(time::clock::Error::Other(ClockImplError::NotStarted))
        }
    }

    fn get_time<Clock: time::Clock>(clock: &Clock)
    where
        u32: TryFrom<Clock::T>,
        Clock::T: TryFrom<u32>,
    {
        assert_eq!(
            clock
                .try_now()
                .ok()
                .unwrap()
                .duration_since_epoch()
                .try_into(),
            Ok(Seconds(2_u32))
        );
    }

    #[test]
    fn common_types() {
        let then = MockClock32.try_now().unwrap();
        let now = MockClock32.try_now().unwrap();

        let clock64 = MockClock64 {};
        let clock32 = MockClock32 {};

        get_time(&clock64);
        get_time(&clock32);

        let then = then - Seconds(1_u32);
        assert_ne!(then, now);
        assert!(then < now);
    }

    #[test]
    fn clock_error() {
        assert_eq!(
            BadClock.try_now(),
            Err(time::clock::Error::Other(ClockImplError::NotStarted))
        );
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
            let duration = Milliseconds::<u64>::try_from(self.0.duration_since_epoch())
                .map_err(|_| fmt::Error {})?;

            let hours = Hours::<u32>::try_convert_from(duration).map_err(|_| fmt::Error {})?;
            let minutes = Minutes::<u32>::try_convert_from(duration).map_err(|_| fmt::Error {})?
                % Hours(1_u32);
            let seconds = Seconds::<u32>::try_convert_from(duration).map_err(|_| fmt::Error {})?
                % Minutes(1_u32);
            let milliseconds = Milliseconds::<u32>::try_convert_from(duration)
                .map_err(|_| fmt::Error {})?
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
