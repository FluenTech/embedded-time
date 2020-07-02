//! The `Clock` trait can be implemented over hardware timers or other time-keeping device

use crate::{time_int::TimeInt, Duration, Instant, Period};
use core::convert::TryFrom;

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

    /// Blocking delay
    ///
    /// # Errors
    /// Implementation-specific error returned through [`Error::Other(ImplError)`]
    fn delay<Dur: Duration>(&self, dur: Dur) -> Result<(), Error<Self::ImplError>>
    where
        Self::Rep: TryFrom<Dur::Rep>,
    {
        let start = self.now()?;
        let end = start + dur;
        while self.now()? < end {}
        Ok(())
    }
}
