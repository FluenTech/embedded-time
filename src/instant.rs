use crate::duration::{Duration, Period, Time};
use crate::numerical_duration::NumericalDuration;
use crate::IntTrait;
use core::{fmt, ops};

pub trait Clock: Sized {
    /// The type of the internal representation of time
    type Rep: IntTrait + NumericalDuration;
    const PERIOD: Period;

    /// Get the current Instant
    fn now<U: Duration<Self::Rep>>() -> Instant<U>
    where
        Self: Sized;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instant<T>(pub T);

impl<T> Instant<T> {
    pub fn duration_since_epoch(self) -> T {
        self.0
    }
}

/// ```
/// # use embedded_time::{Instant, duration::{Seconds, Milliseconds}};
/// assert_eq!(Instant(Seconds(1)) + Seconds(3), Instant(Seconds(4)));
/// assert_eq!(Instant(Seconds(1)) + Milliseconds(700), Instant(Seconds(1)));
/// ```
impl<T, U> ops::Add<U> for Instant<T>
where
    T: ops::Add<U, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: U) -> Self::Output {
        Self(self.0 + rhs)
    }
}

/// ```
/// # use embedded_time::duration::Seconds;
/// # use embedded_time::Instant;
/// assert_eq!(Instant(Seconds(5)) - Instant(Seconds(3)), Seconds(2));
/// ```
impl<T> ops::Sub for Instant<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = T;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

/// ```
/// # use embedded_time::{Instant, duration::{Seconds, Milliseconds}};
/// assert_eq!(Instant(Seconds(3)) - Seconds(2), Instant(Seconds(1)));
/// //assert_eq!(Instant(Seconds(1)) - Milliseconds(700), Instant(Seconds(1)));
/// ```
impl<T, U> ops::Sub<U> for Instant<T>
where
    T: ops::Sub<U, Output = T>,
    U: Time,
{
    type Output = Self;

    fn sub(self, rhs: U) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl<T> fmt::Display for Instant<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
