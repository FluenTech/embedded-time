use crate::{time_int::TimeInt, Duration, Instant, Period};
use core::convert::TryFrom;

/// An abstraction for time-keeping items such as hardware timers
pub trait Clock: Sized {
    /// The type to hold the tick count
    type Rep: TimeInt;

    /// The duration of one clock tick in seconds, AKA the clock precision.
    const PERIOD: Period;

    /// Get the current Instant
    fn now(&mut self) -> Instant<Self>;

    /// Blocking delay
    fn delay<Dur: Duration>(&mut self, dur: Dur)
    where
        Self::Rep: TryFrom<Dur::Rep>,
    {
        let start = self.now();
        let end = start + dur;
        while self.now() < end {}
    }
}
