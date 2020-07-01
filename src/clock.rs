//! The `Clock` trait can be implemented over hardware timers or other time-keeping device

use crate::{time_int::TimeInt, Duration, Error, Instant, Period};
use core::convert::TryFrom;

/// An abstraction for time-keeping items such as hardware timers
pub trait Clock: Sized {
    /// The type to hold the tick count
    type Rep: TimeInt;

    /// The duration of one clock tick in seconds, AKA the clock precision.
    const PERIOD: Period;

    /// Get the current Instant
    ///
    /// # Errors
    /// - Error: The current instant was not readable
    fn now(&mut self) -> Result<Instant<Self>, Error>;

    /// Blocking delay
    ///
    /// # Errors
    /// - Error: The current instant was not readable, actual delay (if any) is unknown
    fn delay<Dur: Duration>(&mut self, dur: Dur) -> Result<(), Error>
    where
        Self::Rep: TryFrom<Dur::Rep>,
    {
        let start = self.now()?;
        let end = start + dur;
        while self.now()? < end {}
        Ok(())
    }
}
