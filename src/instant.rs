use crate::duration::{Duration, Time};
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
/// `T` is a type implementing the `Duration<_>` trait
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

// impl<D1, R1, D2, R2> TryFrom<Instant<D1, R1>> for Instant<D2, R2>
// where
//     R1: TimeRep,
//     R2: TimeRep,
//     D1: Duration<R1>,
//     D2: Duration<R2>,
// {
//     type Error = ();
//
//     fn try_from(value: Instant<D1, R1>) -> Result<Self, Self::Error> {
//         Ok(Self::new(D2::try_from(value.0).unwrap()))
//     }
// }

/// ```rust
/// use embedded_time::Instant;
/// use embedded_time::time_units::*;
/// assert!(Instant::new(Seconds(2)) == Instant::new(Milliseconds(2_000)));
/// assert!(Instant::new(Seconds(1_i64)) != Instant::new(Milliseconds(1_001_i32)));
/// ```
impl<T1, R1, T2, R2> PartialEq<Instant<T2, R2>> for Instant<T1, R1>
where
    R2: TimeRep,
    R1: TimeRep,
    T1: Duration<R1>,
    T2: Duration<R2>,
    // T1: PartialEq<T2>,
    // T1: PartialEq,
    R1: PartialEq<R2>,
    // R2: PartialEq<R1>,
    R2: TryInto<R1>,
    R1: TryFrom<R2>,
    TryFromIntError: From<<R2 as TryInto<R1>>::Error>,
{
    fn eq(&self, other: &Instant<T2, R2>) -> bool {
        self.0 == other.0.map_into::<R1>().unwrap()
    }
}

/// ```rust
/// use embedded_time::Instant;
/// use embedded_time::time_units::*;
/// //assert!(Instant::new(Seconds(1)) < Instant::new(Milliseconds(1_001)));
/// ```
impl<T1, R1, T2, R2> PartialOrd<Instant<T2, R2>> for Instant<T1, R1>
where
    R2: TimeRep,
    R1: TimeRep,
    T1: Duration<R1>,
    T2: Duration<R2>,
    T1: PartialOrd,
    R1: PartialEq<R2>,
    R2: TryInto<R1>,
    R1: TryFrom<R2>,
    TryFromIntError: From<<R2 as TryInto<R1>>::Error>,
{
    fn partial_cmp(&self, other: &Instant<T2, R2>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0.try_into::<_, R1>().unwrap())
        // unimplemented!()
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
