use crate::{numerical_duration::TimeRep, Duration, Instant, Period};
use core::convert::TryFrom;

pub trait Clock: Sized {
    /// The type of the internal representation of time
    type Rep: TimeRep;

    /// The duration of one clock tick, AKA the clock precision.
    const PERIOD: Period;

    /// Get the current Instant
    fn now() -> Instant<Self>;

    /// Blocking delay
    fn delay<Dur>(dur: Dur)
    where
        Dur: Duration,
        Dur::Rep: TimeRep,
        Self::Rep: TryFrom<Dur::Rep>,
    {
        let start = Self::now();
        let end = start + dur;
        while Self::now() < end {}
    }
}
