use crate::{instant::Instant, numerical_duration::TimeRep, Period};

pub trait Clock: Sized {
    /// The type of the internal representation of time
    type Rep: TimeRep;

    /// The duration of one clock tick, AKA the clock precision.
    const PERIOD: Period;

    /// Get the current Instant
    fn now() -> Instant<Self>;
}
