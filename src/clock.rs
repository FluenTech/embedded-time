use crate::timer::param::{self, Disarmed, OneShot};
use crate::{time_int::TimeInt, Duration, Instant, Period, Timer};

/// An abstraction for time-keeping items such as hardware timers
pub trait Clock: Sized {
    /// The type to hold the tick count
    type Rep: TimeInt;

    /// The duration of one clock tick in seconds, AKA the clock precision.
    const PERIOD: Period;

    /// Get the current Instant
    fn now() -> Instant<Self>;

    /// Spawn a new, `OneShot` [`Timer`] from this clock
    fn new_timer<Dur: Duration>() -> Timer<OneShot, Disarmed, Self, Dur> {
        Timer::<param::None, param::None, Self, Dur>::new()
    }
}
