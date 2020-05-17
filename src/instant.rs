use crate::duration::time_units::{TryConvertFrom, TryConvertInto};
use crate::numerical_duration::TimeRep;
use crate::{Duration, Period};
use core::{cmp::Ordering, fmt, ops};

pub trait Clock: Sized + Period {
    /// The type of the internal representation of time
    type Rep: TimeRep;

    /// Get the current Instant
    fn now<Dur>() -> Instant<Dur>
    where
        Dur: Duration,
        Dur::Rep: TimeRep;
}

/// # Example
/// ```rust
/// # use embedded_time::{prelude::*, time_units::*, Instant};
/// Instant(Milliseconds(23));
/// ```
#[derive(Debug, Copy, Clone, Eq, Ord)]
pub struct Instant<Dur: Duration>(pub Dur);

impl<Dur: Duration> Instant<Dur> {
    pub fn duration_since_epoch(self) -> Dur {
        self.0
    }
}

impl<Dur, Source> TryConvertFrom<Instant<Source>> for Instant<Dur>
where
    Source: Duration,
    Dur: Duration + TryConvertFrom<Source>,
{
    type Error = <Dur as TryConvertFrom<Source>>::Error;

    /// # Errors
    ///
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// # use embedded_time::Instant;
    /// assert_eq!(Instant::<Seconds<i32>>::try_convert_from(Instant(Milliseconds(23_000_i64))), Ok(Instant(Seconds(23_i32))));
    /// ```
    fn try_convert_from(
        other: Instant<Source>,
    ) -> Result<Self, <Self as TryConvertFrom<Instant<Source>>>::Error> {
        Ok(Instant(Dur::try_convert_from(other.0)?))
    }
}

/// # Examples
/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// # use embedded_time::Instant;
/// assert_eq!(Instant(Milliseconds(23_000_i64)).try_convert_into(), Ok(Instant(Seconds(23_i32))));
/// ```
impl<Dur, Dest> TryConvertInto<Instant<Dest>> for Instant<Dur>
where
    Dur: Duration,
    Dest: Duration,
    Instant<Dest>: TryConvertFrom<Self>,
{
    type Error = <Instant<Dest> as TryConvertFrom<Self>>::Error;

    fn try_convert_into(self) -> Result<Instant<Dest>, Self::Error> {
        Ok(Instant::<Dest>::try_convert_from(self)?)
    }
}

/// # Examples
/// ```rust
/// # use embedded_time::Instant;
/// # use embedded_time::time_units::*;
/// assert_eq!(Instant(Milliseconds(1_123_i32)), Instant(Microseconds(1_123_000_i64)));
/// assert_ne!(Instant(Milliseconds(1_123_i32)), Instant(Seconds(1_i64)));
/// ```
impl<Dur1, Dur2> PartialEq<Instant<Dur2>> for Instant<Dur1>
where
    Dur1: Duration + PartialEq<Dur2>,
    Dur2: Duration,
{
    fn eq(&self, other: &Instant<Dur2>) -> bool {
        self.0 == other.0
    }
}

/// # Examples
/// ```rust
/// # use embedded_time::Instant;
/// # use embedded_time::time_units::*;
/// assert!(Instant(Seconds(2)) <  Instant(Seconds(3)));
/// assert!(Instant(Seconds(2)) <  Instant(Milliseconds(2_001)));
/// assert!(Instant(Seconds(2)) == Instant(Milliseconds(2_000)));
/// assert!(Instant(Seconds(2)) >  Instant(Milliseconds(1_999)));
/// assert!(Instant(Seconds(2_i32)) < Instant(Milliseconds(2_001_i64)));
/// assert!(Instant(Seconds(2_i64)) < Instant(Milliseconds(2_001_i32)));
/// ```
impl<Dur, RhsDur> PartialOrd<Instant<RhsDur>> for Instant<Dur>
where
    Dur: Duration + PartialOrd<RhsDur>,
    RhsDur: Duration,
{
    fn partial_cmp(&self, other: &Instant<RhsDur>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// Add a duration to an instant resulting in a new, later instance
///
/// # Panics
/// See [`impl Sub for Duration`](duration/time_units/index.html#addsub) for details
///
/// # Examples
/// ```rust
/// # use embedded_time::Instant;
/// # use embedded_time::time_units::*;
/// assert_eq!(Instant(Seconds(1)) + Seconds(3), Instant(Seconds(4)));
/// assert_eq!(Instant(Seconds(-1)) + Milliseconds(5_123), Instant(Seconds(4)));
/// assert_eq!(Instant(Seconds(1)) + Milliseconds(700), Instant(Seconds(1)));
/// assert_eq!(Instant(Seconds(1_i32)) + Milliseconds(700_i64), Instant(Seconds(1_i32)));
/// ```
impl<Dur, AddDur> ops::Add<AddDur> for Instant<Dur>
where
    Dur: Duration + ops::Add<AddDur, Output = Dur>,
    AddDur: Duration,
{
    type Output = Self;

    fn add(self, rhs: AddDur) -> Self::Output {
        Self(self.0 + rhs)
    }
}

/// Calculates the difference between two `Instance`s resulting in a [`Duration`]
///
/// The returned [`Duration`] will be the type from the lhs `Instance`
///
/// # Panics
/// See [`impl Sub for Duration`](duration/time_units/index.html#addsub) for details
///
/// # Examples
/// ```rust
/// # use embedded_time::Instant;
/// # use embedded_time::time_units::*;
/// assert_eq!(Instant(Seconds(5)) - Instant(Seconds(3)), Seconds(2));
/// assert_eq!(Instant(Seconds(3)) - Instant(Seconds(5)), Seconds(-2));
/// assert_eq!(Instant(Milliseconds(5_000)) - Instant(Seconds(2)), Milliseconds(3_000));
/// assert_eq!(Instant(Milliseconds(5_000_i32)) - Instant(Seconds(2_i64)), Milliseconds(3_000_i32));
///
/// // wrapping examples
/// //assert_eq!(Instant(Seconds(1)) - Instant(Seconds(i32::MAX)), Seconds(2))
/// ```
impl<Dur, RhsDur> ops::Sub<Instant<RhsDur>> for Instant<Dur>
where
    Dur: Duration + ops::Sub<RhsDur, Output = Dur>,
    RhsDur: Duration,
{
    type Output = Dur;

    fn sub(self, rhs: Instant<RhsDur>) -> Self::Output {
        self.0 - rhs.0
    }
}

/// Subtract a duration from an instant resulting in a new, earlier instance
///
/// # Panics
/// See [`impl Sub for Duration`](duration/time_units/index.html#addsub) for details
///
/// # Examples
/// ```rust
/// # use embedded_time::Instant;
/// # use embedded_time::time_units::*;
/// assert_eq!(Instant(Seconds(3)) - Seconds(2), Instant(Seconds(1)));
/// assert_eq!(Instant(Seconds(3)) - Milliseconds(5_000), Instant(Seconds(-2)));
/// assert_eq!(Instant(Seconds(1)) - Milliseconds(700), Instant(Seconds(1)));
/// ```
impl<Dur, SubDur> ops::Sub<SubDur> for Instant<Dur>
where
    Dur: Duration + ops::Sub<SubDur, Output = Dur>,
    SubDur: Duration,
{
    type Output = Self;

    fn sub(self, rhs: SubDur) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl<Dur> fmt::Display for Instant<Dur>
where
    Dur: Duration,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
