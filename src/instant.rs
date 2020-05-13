use crate::duration::time_units::{TryConvertFrom, TryConvertInto, *};
use crate::duration::{Duration, Time, TryMapFromError};
use crate::integer::IntTrait;
use crate::numerical_duration::TimeRep;
use crate::{Period, Wrapper};
use core::convert::TryInto;
use core::marker::PhantomData;
use core::num::TryFromIntError;
use core::{cmp::Ordering, convert::TryFrom, fmt, ops};
use num::rational::Ratio;

pub trait Clock: Sized {
    /// The type of the internal representation of time
    type Rep: TimeRep;
    const PERIOD: Ratio<i32>;

    /// Get the current Instant
    fn now<Dur: Copy>() -> Instant<Dur>
    where
        Self: Sized;
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
/// //assert!(Instant(Seconds(1_i32)) == Instant(Seconds(1_i64)));  <- won't compile
///
/// let time = Instant(Seconds(1_i32));
/// let time = i64::try_convert_from(time);
/// assert_eq!(time, Ok(Instant(Seconds(1_i64))));
/// let time = time.unwrap().try_convert_into();
/// assert_eq!(time, Ok(Instant(Seconds(1_i32))));
/// ```
#[derive(Debug, Copy, Clone, Eq, Ord)]
pub struct Instant<T: Copy>(pub T);

impl<T: Copy> Instant<T> {
    pub fn duration_since_epoch(self) -> T {
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
/// let time = Instant(Seconds(23_000_i64));
/// let time = i32::try_convert_from(time);
/// assert_eq!(time, Ok(Instant(Seconds(23_000_i32))));
/// let time = i64::try_convert_from(time.unwrap());
/// assert_eq!(time, Ok(Instant(Seconds(23_000_i64))));
/// let time = i64::try_convert_from(time.unwrap());
/// assert_eq!(time, Ok(Instant(Seconds(23_000_i64))));
///
/// //assert_eq!(i32::try_convert_from(Milliseconds(23_000_i64)), Ok(Milliseconds(23_000_i32)));
/// ```
impl<FromRep, ToRep> TryConvertFrom<Instant<Seconds<FromRep>>, FromRep> for ToRep
where
    FromRep: TimeRep,
    ToRep: TimeRep + TryFrom<FromRep>,
    TryMapFromError: From<<ToRep as TryFrom<FromRep>>::Error>,
{
    type Error = TryMapFromError;
    type Output = Instant<Seconds<Self>>;

    fn try_convert_from(
        other: Instant<Seconds<FromRep>>,
    ) -> Result<Instant<Seconds<Self>>, TryMapFromError> {
        Ok(Instant(Self::try_convert_from(other.0)?))
    }
}

/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// # use embedded_time::Instant;
/// let time = Instant(Seconds(23_000_i64));
/// let time = time.try_convert_into();
/// assert_eq!(time, Ok(Instant(Seconds(23_000_i32))));
/// let time = i64::try_convert_from(time.unwrap());
/// assert_eq!(time, Ok(Instant(Seconds(23_000_i64))));
/// let time = i64::try_convert_from(time.unwrap());
/// assert_eq!(time, Ok(Instant(Seconds(23_000_i64))));
///
/// assert_eq!(Milliseconds(23_000_i64).try_convert_into(), Ok(Milliseconds(23_000_i32)));
/// ```
impl<FromRep, ToRep> TryConvertInto<Instant<Seconds<ToRep>>, ToRep> for Instant<Seconds<FromRep>>
where
    FromRep: TimeRep,
    ToRep: TimeRep,
    ToRep: TryConvertFrom<Instant<Seconds<FromRep>>, FromRep, Output = Instant<Seconds<ToRep>>>,
{
    type Error = <ToRep as TryConvertFrom<Instant<Seconds<FromRep>>, FromRep>>::Error;

    fn try_convert_into(self) -> Result<Instant<Seconds<ToRep>>, Self::Error> {
        Ok(ToRep::try_convert_from(self)?)
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

// /// ```
// /// # use embedded_time::Instant;
// /// # use embedded_time::time_units::*;
// /// assert_eq!(Instant(Seconds(1)) + Seconds(3), Instant(Seconds(4)));
// /// assert_eq!(Instant(Seconds(1)) + Milliseconds(700), Instant(Seconds(1)));
// /// ```
// impl<T, U> ops::Add<U> for Instant<T>
// where
//     T: ops::Add<U, Output = T>,
// {
//     type Output = Self;
//
//     fn add(self, rhs: U) -> Self::Output {
//         Self(self.0 + rhs)
//     }
// }
//
// /// ```
// /// # use embedded_time::Instant;
// /// # use embedded_time::time_units::*;
// /// assert_eq!(Instant(Seconds(5)) - Instant(Seconds(3)), Seconds(2));
// /// ```
// impl<T> ops::Sub for Instant<T>
// where
//     T: ops::Sub<Output = T>,
// {
//     type Output = T;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         self.0 - rhs.0
//     }
// }
//
// /// ```
// /// # use embedded_time::Instant;
// /// # use embedded_time::time_units::*;
// /// assert_eq!(Instant(Seconds(3)) - Seconds(2), Instant(Seconds(1)));
// /// //assert_eq!(Instant(Seconds(1)) - Milliseconds(700), Instant(Seconds(1)));
// /// ```
// impl<T, U> ops::Sub<U> for Instant<T>
// where
//     T: ops::Sub<U, Output = T>,
//     U: Time,
// {
//     type Output = Self;
//
//     fn sub(self, rhs: U) -> Self::Output {
//         Self(self.0 - rhs)
//     }
// }
//
// impl<T> fmt::Display for Instant<T>
// where
//     T: fmt::Display,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.0.fmt(f)
//     }
// }
