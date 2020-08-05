//! `embedded-time` provides a comprehensive library of [`Duration`] and [`Rate`] types as well as
//! a [`Clock`] abstractions for hardware timers/clocks and the associated [`Instant`] type for
//! in embedded systems.
//!
//! Additionally, an implementation of software timers is provided that work seemlessly with all
//! the types in this crate.
//!
//! # Imports
//!
//! The suggested use statements are as follows depending on what is needed:
//!
//! ```rust
//! use embedded_time::duration::*;    // imports all duration-related types and traits
//! use embedded_time::rate::*;        // imports all rate-related types and traits
//! use embedded_time::clock;
//! use embedded_time::Instant;
//! use embedded_time::Timer;
//! ```
//!
//! # Details
//!
//! The approach taken is similar to the C++ `chrono` library. [`Duration`]s and [`Rate`]s are
//! fixed-point values as in they are comprised of _integer_ and _scaling factor_ values.
//! The _scaling factor_ is a `const` [`Fraction`](fraction::Fraction). One benefit of this
//! structure is that it avoids unnecessary arithmetic. For example, if the [`Duration`] type is
//! [`Milliseconds`], a call to the [`Duration::integer()`] method simply returns the _integer_
//! part directly which in the case is the number of milliseconds represented by the [`Duration`].
//! Conversion arithmetic is only performed when explicitly converting between time units (eg.
//! [`Milliseconds`] --> [`Seconds`]).
//!
//! In addition, a wide range of rate-type types are available including [`Hertz`],
//! [`BitsPerSecond`], [`KibibytesPerSecond`], [`Baud`], etc.
//!
//! A [`Duration`] type can be converted to a [`Rate`] type and vica-versa.
//!
//! [`Seconds`]: duration::units::Seconds
//! [`Milliseconds`]: duration::units::Milliseconds
//! [`Rate`]: rate::Rate
//! [`Hertz`]: rate::units::Hertz
//! [`BitsPerSecond`]: rate::units::BitsPerSecond
//! [`KibibytesPerSecond`]: rate::units::KibibytesPerSecond
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

#![deny(unsafe_code)]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
#![deny(intra_doc_link_resolution_failure)]

pub mod clock;
pub mod duration;
pub mod fixed_point;
pub mod fraction;
mod instant;
pub mod rate;
mod time_int;
mod timer;

pub use clock::Clock;
pub use instant::Instant;
pub use timer::Timer;

/// Crate errors
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum TimeError {
    /// Exact cause of failure is unknown
    Unspecified,
    /// Attempted type conversion failed
    ConversionFailure,
    /// Result is outside of those valid for this type
    Overflow,
    /// Attempted to divide by zero
    DivByZero,
    /// Resulting [`Duration`](duration/trait.Duration.html) is negative (not allowed)
    NegDuration,
    /// [`Clock`]-implementation-specific error
    Clock(clock::Error),
}

impl From<clock::Error> for TimeError {
    fn from(clock_error: clock::Error) -> Self {
        TimeError::Clock(clock_error)
    }
}

/// Conversion errors
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ConversionError {
    /// Exact cause of failure is unknown
    Unspecified,
    /// Attempted type conversion failed
    ConversionFailure,
    /// Result is outside of those valid for this type
    Overflow,
    /// Attempted to divide by zero
    DivByZero,
    /// Resulting [`Duration`](duration/trait.Duration.html) is negative (not allowed)
    NegDuration,
}

impl From<ConversionError> for TimeError {
    fn from(error: ConversionError) -> Self {
        match error {
            ConversionError::Unspecified => TimeError::Unspecified,
            ConversionError::ConversionFailure => TimeError::ConversionFailure,
            ConversionError::Overflow => TimeError::Overflow,
            ConversionError::DivByZero => TimeError::DivByZero,
            ConversionError::NegDuration => TimeError::NegDuration,
        }
    }
}

#[cfg(test)]
mod tests {}
