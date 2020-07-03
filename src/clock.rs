//! The `Clock` trait can be implemented over hardware timers or other time-keeping device

use crate::{
    duration::Duration, instant::Instant, period::Period, time_int::TimeInt, timer::param,
    timer::Timer,
};

/// Potential `Clock` errors
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum Error<E: crate::Error = ()> {
    /// specific implementation error
    Other(E),
}

/// An abstraction for time-keeping items such as hardware timers
pub trait Clock: Sized {
    /// The type to hold the tick count
    type Rep: TimeInt;

    /// The duration of one clock tick in seconds, AKA the clock precision.
    const PERIOD: Period;

    /// Implementation-specific error type
    type ImplError: crate::Error;

    /// Get the current Instant
    ///
    /// # Errors
    /// Implementation-specific error returned through [`Error::Other(ImplError)`]
    fn now(&self) -> Result<Instant<Self>, Error<Self::ImplError>>;

    /// Spawn a new, `OneShot` [`Timer`] from this clock
    fn new_timer<Dur: Duration>(&self) -> Timer<param::OneShot, param::Disarmed, Self, Dur> {
        Timer::<param::None, param::None, Self, Dur>::new(&self)
    }
}
