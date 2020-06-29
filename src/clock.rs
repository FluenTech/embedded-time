use crate::{
    time_int::TimeInt,
    timer::{param, TimerBuilder},
    Duration, Instant, Period,
};

/// An abstraction for time-keeping items such as hardware timers
pub trait Clock: Sized {
    /// The type to hold the tick count
    type Rep: TimeInt;

    /// The duration of one clock tick in seconds, AKA the clock precision.
    const PERIOD: Period;

    /// Get the current Instant
    fn now() -> Instant<Self>;

    /// Construct a new [`TimerBuilder`] based on this `Clock`
    ///
    /// The new [`TimerBuilder`]'s type is `OneShot`, but can be changed to
    /// `Periodic` with the [`TimerBuilder::into_periodic()`] method
    fn new_timer<Dur: Duration>() -> TimerBuilder<param::OneShot, param::Disarmed, Self, Dur> {
        TimerBuilder::<param::None, param::None, Self, Dur>::new()
    }
}
