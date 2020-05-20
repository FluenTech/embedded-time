//! Duration types/units creation and conversion.

use crate::numerical_duration::TimeRep;
use crate::Period;
use core::{convert::TryFrom, fmt, mem::size_of, ops, prelude::v1::*};
use num::{rational::Ratio, traits::WrappingSub, Bounded};

/// A duration of time with generic storage
///
/// Each implementation defines a constant fraction/ratio representing the period of the LSb
///
/// # Implementation Example
/// ```rust,no_run
/// # use embedded_time::{Duration, Period, Ratio, TimeRep};
/// # use core::{fmt, fmt::Formatter};
/// #
/// #[derive(Copy, Clone)]
/// struct Milliseconds<T: TimeRep>(pub T);
///
/// impl<T: TimeRep> Duration for Milliseconds<T> {
///     type Rep = T;   // set the storage type
///
///     fn new(value: Self::Rep) -> Self {
///         Self(value)
///     }
///
///     fn count(self) -> Self::Rep {
///         self.0
///     }
/// }
///
/// impl<T: TimeRep> Period for Milliseconds<T> {
///     const PERIOD: Ratio<i32> = Ratio::<i32>::new_raw(1, 1_000); // set LSb period to 1 millisecond
/// }
///
/// impl<T: TimeRep> fmt::Display for Milliseconds<T> {
///     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
///         unimplemented!()
///     }
///     
/// }
/// ```
pub trait Duration: Sized + Copy + fmt::Display + Period {
    type Rep: TimeRep;

    /// Not generally useful or needed as the duration can be instantiated like this:
    /// ```no_run
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// Seconds(123);
    /// 123.seconds();
    /// ```
    /// It only exists to allow Duration methods with default definitions to create a
    /// new duration
    fn new(value: Self::Rep) -> Self;

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds(123).count(), 123);
    /// ```
    fn count(self) -> Self::Rep;

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// # use num::rational::Ratio;
    /// assert_eq!(Microseconds::<i32>::from_ticks(5_i64, Ratio::<i32>::new_raw(1, 1_000)), Microseconds(5_000_i32));
    /// assert_eq!(Microseconds::<i64>::from_ticks(i32::MAX, Ratio::<i32>::new_raw(1, 1_000)), Microseconds((i32::MAX as i64) * 1_000));
    /// assert_eq!(Milliseconds::<i32>::from_ticks((i32::MAX as i64) + 1, Ratio::<i32>::new_raw(1, 1_000_000)), Milliseconds(((i32::MAX as i64) + 1) / 1_000));
    /// ```
    fn from_ticks<Rep>(ticks: Rep, period: Ratio<i32>) -> Self
    where
        Self::Rep: TimeRep + TryFrom<Rep, Error: fmt::Debug>,
        Rep: TimeRep,
    {
        if size_of::<Self::Rep>() > size_of::<Rep>() {
            let converted_ticks = Self::Rep::try_from(ticks).unwrap();

            if period > Ratio::new_raw(1, 1) {
                Some(Self::new(TimeRep::checked_div(
                    &converted_ticks.checked_mul(&period)?,
                    &Self::PERIOD,
                )?))
            } else {
                Some(Self::new(
                    converted_ticks.checked_mul(&period.checked_div(&Self::PERIOD)?)?,
                ))
            }
        } else {
            let ticks = if period > Ratio::new_raw(1, 1) {
                TimeRep::checked_div(&TimeRep::checked_mul(&ticks, &period)?, &Self::PERIOD)?
            } else {
                TimeRep::checked_mul(&ticks, &period.checked_div(&Self::PERIOD)?)?
            };

            let converted_ticks = Self::Rep::try_from(ticks).unwrap();
            Self::new(converted_ticks)
        }
    }

    /// Create an integer representation with LSb period of that provided
    ///
    /// # Errors
    /// - the conversion of periods causes an overflow
    /// - the Self integer cast to that of the provided type fails
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{prelude::*, time_units::*, Instant, Ratio};
    /// assert_eq!(Microseconds(5_000_i32).into_ticks::<i32>(Ratio::<i32>::new_raw(1, 1_000)), Ok(5_i32));
    /// assert_eq!(Microseconds(5_000_i32).into_ticks::<i32>(Ratio::<i32>::new_raw(1, 200)), Ok(1_i32));
    /// assert_eq!(Microseconds::<i32>(i32::MAX).into_ticks::<i64>(Ratio::<i32>::new_raw(1, 2_000_000)), Ok((i32::MAX as i64) * 2));
    /// assert_eq!(Microseconds::<i64>((i32::MAX as i64) + 2).into_ticks::<i32>(Ratio::new_raw(1, 500_000)), Ok(i32::MAX / 2 + 1));
    /// assert_eq!(Microseconds::<i64>(i32::MAX as i64).into_ticks::<i32>(Ratio::<i32>::new_raw(1, 500_000)), Ok(i32::MAX / 2));
    /// ```
    fn into_ticks<Rep>(self, period: Ratio<i32>) -> Result<Rep, <Rep as TryFrom<Self::Rep>>::Error>
    where
        Self::Rep: TimeRep,
        Rep: TimeRep + TryFrom<Self::Rep, Error: fmt::Debug>,
    {
        if size_of::<Rep>() > size_of::<Self::Rep>() {
            let ticks = Rep::try_from(self.count())?;

            if period > Ratio::new_raw(1, 1) {
                Some(TimeRep::checked_div(
                    &TimeRep::checked_mul(&ticks, &Self::PERIOD)?,
                    &period,
                )?)
            } else {
                Some(TimeRep::checked_mul(
                    &ticks,
                    &Self::PERIOD.checked_div(&period)?,
                )?)
            }
        } else {
            let ticks = if Self::PERIOD > Ratio::new_raw(1, 1) {
                TimeRep::checked_div(
                    &TimeRep::checked_mul(&self.count(), &Self::PERIOD)?,
                    &period,
                )?
            } else {
                TimeRep::checked_mul(&self.count(), &Self::PERIOD.checked_div(&period)?)?
            };

            Rep::try_from(ticks)
        }
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds::<i32>::min_value(), i32::MIN);
    /// ```
    #[must_use]
    fn min_value() -> Self::Rep {
        Self::Rep::min_value()
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds::<i32>::max_value(), i32::MAX);
    /// ```
    #[must_use]
    fn max_value() -> Self::Rep {
        Self::Rep::max_value()
    }

    /// Apply wrapping subtraction
    ///
    /// # Example
    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds(1).wrapping_sub(Seconds(u32::MAX as i32)), Seconds(2))
    /// ```
    fn wrapping_sub<Rhs>(self, rhs: Rhs) -> Self
    where
        Self: TryConvertFrom<Rhs, Error: fmt::Debug>,
        Self::Rep: TryFrom<Rhs::Rep, Error: fmt::Debug>,
        Rhs::Rep: TimeRep,
        Rhs: Duration,
    {
        let rhs = Self::try_convert_from(rhs).unwrap();
        Self::new(self.count().wrapping_sub(&rhs.count()))
    }
}

pub trait TryConvertFrom<Source>: Sized {
    type Error: fmt::Debug;

    fn try_convert_from(other: Source) -> Result<Self, Self::Error>;
}

pub trait TryConvertInto<Dest> {
    type Error: fmt::Debug;

    fn try_convert_into(self) -> Result<Dest, Self::Error>;
}

impl<Source, Dest> TryConvertFrom<Source> for Dest
where
    Dest: Duration,
    Dest::Rep: TimeRep + TryFrom<Source::Rep, Error: fmt::Debug>,
    Source: Duration,
    Source::Rep: TimeRep,
{
    /// Type returned upon conversion failure
    type Error = <Dest::Rep as TryFrom<Source::Rep>>::Error;

    /// Attempt to convert from one duration type to another
    ///
    /// Both the underlying storage type and the LSb period can be converted
    ///
    /// # Errors
    /// - unable to cast underlying types
    /// - LSb period conversion overflow
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// # use embedded_time::duration::TryConvertFrom;
    /// assert_eq!(Seconds::<i32>::try_convert_from(Milliseconds(23_000_i64)), Ok(Seconds(23_i32)));
    /// assert_eq!(Seconds::<i64>::try_convert_from(Milliseconds(23_000_i32)), Ok(Seconds(23_i64)));
    /// ```
    fn try_convert_from(source: Source) -> Result<Self, <Self as TryConvertFrom<Source>>::Error> {
        Ok(Self::from_ticks(source.count(), Source::PERIOD))
    }
}

/// The reciprocal of [`TryConvertFrom`]
///
/// # Examples
/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// # use embedded_time::duration::TryConvertInto;
/// assert_eq!(Seconds(23_000_i64).try_convert_into(), Ok(Seconds(23_000_i32)));
/// assert_eq!(Seconds(23_000_i32).try_convert_into(), Ok(Seconds(23_000_i32)));
/// assert_eq!(Ok(Seconds(23_000_i64)), (Seconds(23_000_i32).try_convert_into()));
/// assert_eq!(Milliseconds(23_000_i64).try_convert_into(), Ok(Seconds(23_i32)));
/// assert_eq!(Milliseconds(23_000_i32).try_convert_into(), Ok(Seconds(23_i64)));
/// ```
impl<Source, Dest> TryConvertInto<Dest> for Source
where
    Source: Duration,
    Dest: Duration + TryConvertFrom<Source>,
{
    type Error = <Dest as TryConvertFrom<Self>>::Error;

    fn try_convert_into(self) -> Result<Dest, <Self as TryConvertInto<Dest>>::Error> {
        Dest::try_convert_from(self)
    }
}

pub mod time_units {
    //! Implementations of the [`Duration`] trait.
    //!
    //! # Constructing a duration
    //! ```rust
    //! # use embedded_time::prelude::*;
    //! # use embedded_time::time_units::*;
    //! assert_eq!(Milliseconds::<i32>::new(23), Milliseconds(23_i32));
    //! assert_eq!(Milliseconds(23), 23.milliseconds());
    //! ```
    //!
    //! # Get the integer count
    //! ```rust
    //! # use embedded_time::prelude::*;
    //! # use embedded_time::time_units::*;
    //! assert_eq!(Milliseconds(23).count(), 23);
    //! ```
    //!
    //! # Formatting
    //! Just forwards the underlying integer to [`core::fmt::Display::fmt()`]
    //! ```rust
    //! # use embedded_time::prelude::*;
    //! # use embedded_time::time_units::*;
    //! assert_eq!(format!("{}", Seconds(123)), "123");
    //! ```
    //!
    //!
    //! # Add/Sub
    //!
    //! ## Panics
    //! Panics if the rhs duration cannot be converted into the lhs duration type
    //!
    //! In this example, the maximum `i32` value of seconds is stored as `i32` and
    //! converting that value to milliseconds (with `i32` storage type) causes an overflow.
    //! ```rust,should_panic
    //! # use embedded_time::prelude::*;
    //! # use embedded_time::time_units::*;
    //! let _ = Milliseconds(24) + Seconds(i32::MAX);
    //! ```
    //!
    //! This example works just fine as the seconds value is first cast to `i64`, then
    //! converted to milliseconds.
    //! ```rust
    //! # use embedded_time::prelude::*;
    //! # use embedded_time::time_units::*;
    //! let _ = Milliseconds(24_i64) + Seconds(i32::MAX);
    //! ```
    //!
    //! Here, there is no units conversion to worry about, but `i32::MAX + 1` cannot be
    //! cast to an `i32`.
    //! ```rust,should_panic
    //! # use embedded_time::prelude::*;
    //! # use embedded_time::time_units::*;
    //! let _ = Seconds(i32::MAX) + Seconds(i32::MAX as i64 + 1);
    //! # //todo: perhaps initially convert types to largest storage, do the op, then convert to lhs type
    //! ```
    //!
    //! ## Examples
    //! ```rust
    //! # use embedded_time::prelude::*;
    //! # use embedded_time::time_units::*;
    //! assert_eq!((Milliseconds(3_234) - Seconds(2)), Milliseconds(1_234));
    //! assert_eq!((Milliseconds(3_234_i64) - Seconds(2_i32)), Milliseconds(1_234_i64));
    //! ```
    //!
    //! # Equality
    //! ```rust
    //! # use embedded_time::prelude::*;
    //! # use embedded_time::time_units::*;
    //! assert_eq!(Seconds(123), Seconds(123));
    //! assert_eq!(Seconds(123), Milliseconds(123_000));
    //! assert_ne!(Seconds(123), Milliseconds(123_001));
    //! assert_ne!(Milliseconds(123_001), Seconds(123));
    //! assert_ne!(Milliseconds(123_001_i64), Seconds(123_i64));
    //! assert_ne!(Seconds(123_i64), Milliseconds(123_001_i64));
    //! assert_ne!(Seconds(123_i64), Milliseconds(123_001_i32));
    //! ```
    //!
    //! # Comparisons
    //! ```rust
    //! # use embedded_time::prelude::*;
    //! # use embedded_time::time_units::*;
    //! assert!(Seconds(2) < Seconds(3));
    //! assert!(Seconds(2) < Milliseconds(2_001));
    //! assert!(Seconds(2) == Milliseconds(2_000));
    //! assert!(Seconds(2) > Milliseconds(1_999));
    //! assert!(Seconds(2_i32) < Milliseconds(2_001_i64));
    //! assert!(Seconds(2_i64) < Milliseconds(2_001_i32));
    //! ```

    use super::Period;
    use crate::duration::{Duration, TryConvertFrom};
    use crate::numerical_duration::TimeRep;
    use core::{
        cmp,
        convert::TryFrom,
        fmt::{self, Formatter},
        ops,
    };
    use num::rational::Ratio;

    macro_rules! durations {
        ( $( $name:ident, ($numer:expr, $denom:expr) );+ ) => {
            $(
                /// See module-level documentation for details about this type
                #[derive(Copy, Clone, Debug, Eq, Ord)]
                pub struct $name<T: TimeRep>(pub T);

                /// See module-level documentation for details about this type
                impl<T: TimeRep> Period for $name<T> {
                    const PERIOD: Ratio<i32> = Ratio::<i32>::new_raw($numer, $denom);
                }

                impl<Rep: TimeRep> Duration for $name<Rep> {
                    type Rep = Rep;

                    fn new(value: Self::Rep) -> Self {
                        Self(value)
                    }

                    fn count(self) -> Self::Rep {
                        self.0
                    }
                }

                impl<T: TimeRep> fmt::Display for $name<T> {
                    /// See module-level documentation for details about this type
                    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                        fmt::Display::fmt(&self.0, f)
                    }
                }

                impl<Rep, RhsDur> ops::Add<RhsDur> for $name<Rep>
                where
                    RhsDur: Duration,
                    RhsDur::Rep: TimeRep,
                    Rep: TimeRep + TryFrom<RhsDur::Rep, Error: fmt::Debug>,
                {
                    type Output = Self;

                    /// See module-level documentation for details about this type
                    #[inline]
                    fn add(self, rhs: RhsDur) -> Self::Output {
                        Self(self.count() + Self::try_convert_from(rhs).unwrap().count())
                    }
                }

                impl<Rep, RhsDur> ops::Sub<RhsDur> for $name<Rep>
                where
                    Rep: TimeRep + TryFrom<RhsDur::Rep, Error: fmt::Debug>,
                    RhsDur: Duration,
                {
                    type Output = Self;

                    /// See module-level documentation for details about this type
                    #[inline]
                    fn sub(self, rhs: RhsDur) -> Self::Output {
                        Self(self.count() - Self::try_convert_from(rhs).unwrap().count())
                    }
                }

                impl<Rep, OtherDur> cmp::PartialEq<OtherDur> for $name<Rep>
                where
                    Rep: TimeRep + TryFrom<OtherDur::Rep, Error: fmt::Debug>,
                    OtherDur: Duration,
                    OtherDur::Rep: TryFrom<Rep, Error: fmt::Debug>,
                {
                    /// See module-level documentation for details about this type
                    fn eq(&self, other: &OtherDur) -> bool {
                        if Self::PERIOD < OtherDur::PERIOD {
                            self.count() == Self::try_convert_from(*other).unwrap().count()
                        } else {
                            OtherDur::try_convert_from(*self).unwrap().count() == other.count()
                        }
                    }
                }

                impl<Rep, OtherDur> PartialOrd<OtherDur> for $name<Rep>
                where
                    Rep: TimeRep + TryFrom<OtherDur::Rep, Error: fmt::Debug>,
                    OtherDur: Duration,
                    OtherDur::Rep: TryFrom<Rep, Error: fmt::Debug>,
                {
                    /// See module-level documentation for details about this type
                    fn partial_cmp(&self, other: &OtherDur) -> Option<core::cmp::Ordering> {
                        if Self::PERIOD < OtherDur::PERIOD {
                            Some(self.count().cmp(&Self::try_convert_from(*other).unwrap().count()))
                        } else {
                            Some(
                                OtherDur::try_convert_from(*self)
                                    .unwrap()
                                    .count()
                                    .cmp(&other.count()),
                            )
                        }
                    }
                }

             )+
         };
    }

    durations![
        Hours,     (3600, 1);
        Minutes,     (60, 1);
        Seconds,      (1, 1);
        Milliseconds, (1, 1_000);
        Microseconds, (1, 1_000_000);
        Nanoseconds,  (1, 1_000_000_000)
    ];
}

impl<T> ops::Mul<Ratio<i32>> for Integer<T>
where
    T: IntTrait,
{
    type Output = Self;

    fn mul(self, rhs: Ratio<i32>) -> Self::Output {
        Self(self.0 * (*rhs.numer()).into() / (*rhs.denom()).into())
    }
}

impl<T> ops::Div<Ratio<i32>> for Integer<T>
where
    T: IntTrait,
{
    type Output = Self;

    fn div(self, rhs: Ratio<i32>) -> Self::Output {
        Self(self.0 / (*rhs.numer()).into() * (*rhs.denom()).into())
    }
}
