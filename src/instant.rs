use crate::duration::time_units::{TryMapFrom, TryMapInto, *};
use crate::duration::{Duration, Time, TryMapFromError};
use crate::integer::IntTrait;
use crate::numerical_duration::TimeRep;
use crate::Period;
use core::convert::TryInto;
use core::marker::PhantomData;
use core::num::TryFromIntError;
use core::{cmp::Ordering, convert::TryFrom, fmt, ops};

pub trait Clock: Sized {
    /// The type of the internal representation of time
    type Rep: TimeRep;
    const PERIOD: Period;

    /// Get the current Instant
    fn now<U: Duration<Self::Rep>>() -> Instant<U, Self::Rep>
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
/// # use embedded_time::duration::time_units::{TryMapInto, TryMapFrom};
/// assert!(Instant::new(Seconds(1)) == Instant::new(Milliseconds(1_000)));
/// assert!(Instant::new(Seconds(1)) != Instant::new(Milliseconds(1_001)));
/// assert!(Instant::new(Seconds(1)) < Instant::new(Milliseconds(1_001)));
/// assert!(Instant::new(Seconds(1)) > Instant::new(Milliseconds(999)));
///
/// assert!(Instant::new(Microseconds(119_900_000)) < Instant::new(Minutes(2)));
///
/// // assert!(Instant::new(Seconds(1_i32)) < Instant::new(Milliseconds(1_001_i64))); <- doesn't compile
/// assert!(Instant::new(Seconds(1_i32)) < Instant::new(Milliseconds(1_001_i32)));
/// let time = Instant::new(Seconds(1_i32));
/// let time = i64::try_map_from(time);
/// assert_eq!(time, Ok(Instant::new(Seconds(1_i64))));
/// let time = time.unwrap().try_map_into();
/// assert_eq!(time, Ok(Instant::new(Seconds(1_i32))));
/// ```
#[derive(Debug, Copy, Clone, Eq, Ord)]
pub struct Instant<T, R>(pub T, PhantomData<R>)
where
    T: Duration<R>,
    R: TimeRep;

impl<T, R> Instant<T, R>
where
    T: Duration<R>,
    R: TimeRep,
{
    pub fn new(duration: T) -> Self
    where
        T: Duration<R>,
    {
        Self(duration, PhantomData)
    }

    pub fn duration_since_epoch(self) -> T {
        self.0
    }
}

/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// # use embedded_time::Instant;
/// let time = Instant::new(Seconds(23_000_i64));
/// let time = i32::try_map_from(time);
/// assert_eq!(time, Ok(Instant::new(Seconds(23_000_i32))));
/// let time = i64::try_map_from(time.unwrap());
/// assert_eq!(time, Ok(Instant::new(Seconds(23_000_i64))));
/// let time = i64::try_map_from(time.unwrap());
/// assert_eq!(time, Ok(Instant::new(Seconds(23_000_i64))));
///
/// //assert_eq!(i32::try_map_from(Milliseconds(23_000_i64)), Ok(Milliseconds(23_000_i32)));
/// ```
impl<FromRep, ToRep> TryMapFrom<Instant<Seconds<FromRep>, FromRep>, FromRep> for ToRep
where
    FromRep: TimeRep,
    ToRep: TimeRep + TryFrom<FromRep>,
    TryMapFromError: From<<ToRep as TryFrom<FromRep>>::Error>,
{
    type Error = TryMapFromError;
    type Output = Instant<Seconds<Self>, Self>;

    fn try_map_from(
        other: Instant<Seconds<FromRep>, FromRep>,
    ) -> Result<Instant<Seconds<Self>, Self>, TryMapFromError> {
        Ok(Instant::new(Self::try_map_from(other.0)?))
    }
}

/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// # use embedded_time::Instant;
/// let time = Instant::new(Seconds(23_000_i64));
/// let time = time.try_map_into();
/// assert_eq!(time, Ok(Instant::new(Seconds(23_000_i32))));
/// //let time = i64::try_map_from(time);
/// //assert_eq!(time, Ok(Seconds(23_000_i64)));
/// //let time = i64::try_map_from(time);
/// //assert_eq!(time, Ok(Seconds(23_000_i64)));
///
/// //assert_eq!(Milliseconds(23_000_i64).try_map_into(), Ok(Milliseconds(23_000_i32)));
/// ```
impl<FromRep, ToRep> TryMapInto<Instant<Seconds<ToRep>, ToRep>, ToRep>
    for Instant<Seconds<FromRep>, FromRep>
where
    FromRep: TimeRep,
    ToRep: TimeRep,
    ToRep: TryMapFrom<
        Instant<Seconds<FromRep>, FromRep>,
        FromRep,
        Output = Instant<Seconds<ToRep>, ToRep>,
    >,
{
    type Error = <ToRep as TryMapFrom<Instant<Seconds<FromRep>, FromRep>, FromRep>>::Error;

    fn try_map_into(self) -> Result<Instant<Seconds<ToRep>, ToRep>, Self::Error> {
        Ok(ToRep::try_map_from(self)?)
    }
}
impl<T1, R1, T2> PartialEq<Instant<T2, R1>> for Instant<T1, R1>
where
    T1: Duration<R1>,
    T1: PartialEq<T2>,
    R1: TimeRep,
    T2: Duration<R1>,
{
    fn eq(&self, other: &Instant<T2, R1>) -> bool {
        self.0 == other.0
    }
}

impl<T1, R1, T2> PartialOrd<Instant<T2, R1>> for Instant<T1, R1>
where
    R1: TimeRep,
    T1: Duration<R1>,
    T2: Duration<R1>,
    T1: PartialOrd<T2>,
{
    fn partial_cmp(&self, other: &Instant<T2, R1>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
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
