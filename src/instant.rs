use crate::duration::time_units::{TryConvertFrom, TryConvertInto};
use crate::duration::{Time, TryConvertFromError};
use crate::numerical_duration::TimeRep;
use crate::{Duration, Period, Wrapper};
use core::{cmp::Ordering, fmt, ops};

pub trait Clock: Sized + Period {
    /// The type of the internal representation of time
    type Rep: TimeRep;

    /// Get the current Instant
    fn now<Dur>() -> Instant<Dur>
    where
        Dur: Duration<Self::Rep>;
}

/// Represents an instant in time
///
/// Comparisons can be performed between different precisions but not between different representations/widths
///
/// # Examples
/// ```
/// # use embedded_time::Instant;
/// # use embedded_time::time_units::*;
/// # use embedded_time::duration::time_units::{TryConvertInto, TryConvertFrom};
/// assert!(Instant(Seconds(1)) == Instant(Milliseconds(1_000)));
/// assert!(Instant(Seconds(1)) != Instant(Milliseconds(1_001)));
/// assert!(Instant(Seconds(1)) < Instant(Milliseconds(1_001)));
/// assert!(Instant(Seconds(1)) > Instant(Milliseconds(999)));
/// assert!(Instant(Microseconds(119_900_000)) < Instant(Minutes(2)));
///
/// assert!(Instant(Seconds(1_i32)) == Instant(Milliseconds(1_000_i64)));
/// ```
#[derive(Debug, Copy, Clone, Eq, Ord)]
pub struct Instant<T: Copy>(pub T);

impl<Dur: Copy> Instant<Dur> {
    pub fn duration_since_epoch(self) -> Dur {
        self.0
    }
}

impl<Dur: Copy> Wrapper for Instant<Dur> {
    type Rep = Dur;

    fn unwrap(self) -> Self::Rep {
        self.0
    }
}

/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// # use embedded_time::Instant;
/// assert_eq!(Instant::<Seconds::<i32>>::try_convert_from(Instant(Milliseconds(23_000_i64))), Ok(Instant(Seconds(23_i32))));
/// ```
impl<Dur, Source> TryConvertFrom<Source> for Instant<Dur>
where
    Source: Wrapper,
    Dur: TryConvertFrom<Source::Rep> + Copy,
    TryConvertFromError: From<<Dur as TryConvertFrom<Source::Rep>>::Error>,
{
    type Error = TryConvertFromError;

    fn try_convert_from(other: Source) -> Result<Self, <Self as TryConvertFrom<Source>>::Error> {
        Ok(Instant(Dur::try_convert_from(other.unwrap())?))
    }
}

/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// # use embedded_time::Instant;
/// assert_eq!(Instant(Milliseconds(23_000_i64)).try_convert_into(), Ok(Instant(Seconds(23_i32))));
/// ```
impl<Dur, Dest> TryConvertInto<Dest> for Instant<Dur>
where
    Dur: Copy,
    Dest: TryConvertFrom<Self>,
{
    type Error = <Dest as TryConvertFrom<Self>>::Error;

    fn try_convert_into(self) -> Result<Dest, Self::Error> {
        Ok(Dest::try_convert_from(self)?)
    }
}

impl<Dur1: Copy, Dur2: Copy> PartialEq<Instant<Dur2>> for Instant<Dur1>
where
    Dur1: PartialEq<Dur2>,
{
    fn eq(&self, other: &Instant<Dur2>) -> bool {
        (*self).unwrap() == (*other).unwrap()
    }
}

impl<T1: Copy, T2: Copy> PartialOrd<Instant<T2>> for Instant<T1>
where
    T1: PartialOrd<T2>,
{
    fn partial_cmp(&self, other: &Instant<T2>) -> Option<Ordering> {
        self.unwrap().partial_cmp(&other.unwrap())
    }
}

/// ```
/// # use embedded_time::Instant;
/// # use embedded_time::time_units::*;
/// assert_eq!(Instant(Seconds(1)) + Seconds(3), Instant(Seconds(4)));
/// assert_eq!(Instant(Seconds(-1)) + Milliseconds(5_123), Instant(Seconds(4)));
/// assert_eq!(Instant(Seconds(1)) + Milliseconds(700), Instant(Seconds(1)));
/// ```
impl<T, U> ops::Add<U> for Instant<T>
where
    T: ops::Add<U, Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: U) -> Self::Output {
        Self(self.0 + rhs)
    }
}

/// ```
/// # use embedded_time::Instant;
/// # use embedded_time::time_units::*;
/// assert_eq!(Instant(Seconds(5)) - Instant(Seconds(3)), Seconds(2));
/// assert_eq!(Instant(Seconds(3)) - Instant(Seconds(5)), Seconds(-2));
/// ```
impl<T> ops::Sub for Instant<T>
where
    T: Copy + ops::Sub<Output = T>,
{
    type Output = T;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

/// ```
/// # use embedded_time::Instant;
/// # use embedded_time::time_units::*;
/// assert_eq!(Instant(Seconds(3)) - Seconds(2), Instant(Seconds(1)));
/// assert_eq!(Instant(Seconds(3)) - Milliseconds(5_000), Instant(Seconds(-2)));
/// assert_eq!(Instant(Seconds(1)) - Milliseconds(700), Instant(Seconds(1)));
/// ```
impl<T, U> ops::Sub<U> for Instant<T>
where
    T: Copy + ops::Sub<U, Output = T>,
    U: Time,
{
    type Output = Self;

    fn sub(self, rhs: U) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl<T> fmt::Display for Instant<T>
where
    T: fmt::Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
