//! Duration types/units creation and conversion.

use crate::{time_int::TimeInt, Period};
use core::{convert::TryFrom, fmt, mem::size_of, prelude::v1::*};
use num::Bounded;

/// A duration of time with signed, generic storage
///
/// Each implementation defines a constant [`Period`] which is a fraction/ratio representing the
/// period of the count's LSbit
///
/// # Implementation Example
/// ```rust,no_run
/// # use embedded_time::{Duration, Period, TimeInt};
/// # use core::{fmt, fmt::Formatter};
/// #
/// #[derive(Copy, Clone)]
/// struct Milliseconds<T: TimeInt>(pub T);
///
/// impl<T: TimeInt> Duration for Milliseconds<T> {
///     type Rep = T;   // set the storage type
///
///     // set LSbit period to 1 millisecond
///     const PERIOD: Period = Period::new(1, 1_000);
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
/// impl<T: TimeInt> fmt::Display for Milliseconds<T> {
///     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
///         unimplemented!()
///     }
///     
/// }
/// ```
///
/// # Constructing a duration
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// #
/// assert_eq!(Milliseconds::<i32>::new(23), Milliseconds(23_i32));
/// assert_eq!(Milliseconds(23), 23.milliseconds());
/// ```
///
/// # Get the integer count
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// #
/// assert_eq!(Milliseconds(23).count(), 23);
/// ```
///
/// # Formatting
/// Just forwards the underlying integer to [`core::fmt::Display::fmt()`]
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// #
/// assert_eq!(format!("{}", Seconds(123)), "123");
/// ```
///
/// # Getting H:M:S.MS... Components
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// #
/// let duration = 38_238_479.microseconds();
/// let hours = Hours::<i32>::try_convert_from(duration).unwrap();
/// let minutes = Minutes::<i32>::try_convert_from(duration).unwrap() % Hours(1);
/// let seconds = Seconds::<i32>::try_convert_from(duration).unwrap() % Minutes(1);
/// let milliseconds = Milliseconds::<i32>::try_convert_from(duration).unwrap() % Seconds(1);
/// // ...
/// ```
///
/// # Converting to [`core::time::Duration`]
/// ## Examples
/// ```rust
/// # use embedded_time::prelude::*;
/// # use core::convert::TryFrom;
/// #
/// let core_duration = core::time::Duration::try_from(2_569.milliseconds()).unwrap();
/// assert_eq!(2, core_duration.as_secs());
/// assert_eq!(569_000_000, core_duration.subsec_nanos());
/// ```
/// ```rust
/// # use embedded_time::prelude::*;
/// # use core::convert::TryInto;
/// #
/// let core_duration: core::time::Duration = 2_569.milliseconds().try_into().unwrap();
/// assert_eq!(2, core_duration.as_secs());
/// assert_eq!(569_000_000, core_duration.subsec_nanos());
/// ```
///
/// ## Errors
/// Attempting to convert from a _negative_ duration will fail
/// ```rust
/// # use embedded_time::prelude::*;
/// # use core::convert::{TryFrom, TryInto};
/// #
/// assert!(core::time::Duration::try_from((-2_569).milliseconds()).is_err());
///
/// let core_duration: Result<core::time::Duration, _> = (-2_569).milliseconds().try_into();
/// assert!(core_duration.is_err());
/// ```
///
/// # Converting from [`core::time::Duration`]
/// ## Examples
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// # use core::convert::TryFrom;
/// #
/// let core_duration = core::time::Duration::new(5, 730023852);
/// assert_eq!(Milliseconds::<i32>::try_from(core_duration), Ok(5_730.milliseconds()));
/// ```
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// # use core::convert::TryInto;
/// #
/// let duration: Result<Milliseconds<i32>, _> = core::time::Duration::new(5, 730023852).try_into();
/// assert_eq!(duration, Ok(5_730.milliseconds()));
/// ```
///
/// ## Errors
/// The duration doesn't fit in the type specified
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// # use core::convert::{TryFrom, TryInto};
/// #
/// assert!(Milliseconds::<i32>::try_from(core::time::Duration::from_millis((i32::MAX as u64) + 1)).is_err());
///
/// let duration: Result<Milliseconds<i32>, _> = core::time::Duration::from_millis((i32::MAX as u64) + 1).try_into();
/// assert!(duration.is_err());
/// ```
///
/// # Add/Sub
///
/// ## Panics
/// Panics if the rhs duration cannot be converted into the lhs duration type
///
/// In this example, the maximum `i32` value of seconds is stored as `i32` and
/// converting that value to milliseconds (with `i32` storage type) causes an overflow.
/// ```rust,should_panic
/// # use embedded_time::{prelude::*, units::*};
/// #
/// let _ = Milliseconds(24) + Seconds(i32::MAX);
/// ```
///
/// This example works just fine as the seconds value is first cast to `i64`, then
/// converted to milliseconds.
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// #
/// let _ = Milliseconds(24_i64) + Seconds(i32::MAX);
/// ```
///
/// Here, there is no units conversion to worry about, but `i32::MAX + 1` cannot be
/// cast to an `i32`.
/// ```rust,should_panic
/// # use embedded_time::{prelude::*, units::*};
/// #
/// let _ = Seconds(i32::MAX) - Seconds(i32::MAX as i64 + 1);
/// ```
///
/// ## Examples
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// #
/// assert_eq!((Milliseconds(3_234) - Seconds(2)), Milliseconds(1_234));
/// assert_eq!((Milliseconds(3_234_i64) - Seconds(2_i32)), Milliseconds(1_234_i64));
/// assert_eq!((Seconds(i32::MAX) - Milliseconds((i32::MAX as i64) + 1)),
///     Seconds(2_145_336_164_i32));
/// ```
///
/// # Equality
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// #
/// assert_eq!(Seconds(123), Seconds(123));
/// assert_eq!(Seconds(123), Milliseconds(123_000));
///
/// assert_ne!(Seconds(123), Milliseconds(123_001));
/// assert_ne!(Milliseconds(123_001), Seconds(123));
/// assert_ne!(Milliseconds(123_001_i64), Seconds(123_i64));
/// assert_ne!(Seconds(123_i64), Milliseconds(123_001_i64));
/// assert_ne!(Seconds(123_i64), Milliseconds(123_001_i32));
/// ```
///
/// # Comparisons
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// #
/// assert!(Seconds(2) < Seconds(3));
/// assert!(Seconds(2) < Milliseconds(2_001));
/// assert!(Seconds(2) == Milliseconds(2_000));
/// assert!(Seconds(2) > Milliseconds(1_999));
/// assert!(Seconds(2_i32) < Milliseconds(2_001_i64));
/// assert!(Seconds(2_i64) < Milliseconds(2_001_i32));
/// ```
///
/// # Remainder
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// #
/// assert_eq!(Minutes(62) % Hours(1), Minutes(2));
/// ```
pub trait Duration: Sized + Copy + fmt::Display {
    /// The inner type of the `Duration` representing the count of the implementation unit
    type Rep: TimeInt;

    /// A fraction/ratio representing the period of the count's LSbit. The precision of the
    /// `Duration`.
    const PERIOD: Period;

    /// Not generally useful or needed as the duration can be constructed like this:
    /// ```no_run
    /// # use embedded_time::{prelude::*, units::*};
    /// Seconds(123);
    /// 123.seconds();
    /// ```
    /// It only exists to allow Duration methods with default definitions to create a
    /// new duration
    fn new(value: Self::Rep) -> Self;

    /// Returns the integer value of the [`Duration`]
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*};
    /// assert_eq!(Seconds(123).count(), 123);
    /// ```
    fn count(self) -> Self::Rep;

    /// Constructs a [`Duration`] from a value of ticks and a period
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*, Period};
    /// assert_eq!(Microseconds::<i32>::from_ticks(5_i64, Period::new(1, 1_000)),
    ///     Some(Microseconds(5_000_i32)));
    ///
    /// // the conversion arithmetic will not cause overflow
    /// assert_eq!(Milliseconds::<i32>::from_ticks((i32::MAX as i64) + 1, Period::new(1, 1_000_000)),
    ///     Some(Milliseconds((((i32::MAX as i64) + 1) / 1_000) as i32)));
    /// ```
    ///
    /// # Errors
    /// the conversion of periods causes an overflow:
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*, Period};
    /// assert_eq!(Milliseconds::<i32>::from_ticks(i32::MAX, Period::new(1, 1)),
    ///     None);
    /// ```
    ///
    /// the Self integer cast to that of the provided type fails
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*, Period};
    /// assert_eq!(Seconds::<i32>::from_ticks(i32::MAX as i64 + 1, Period::new(1, 1)),
    ///     None);
    /// ```
    ///
    /// # Returns
    /// [`None`] if the result of the conversion does not fit in the requested integer size
    fn from_ticks<Rep>(ticks: Rep, period: Period) -> Option<Self>
    where
        Self::Rep: TimeInt + TryFrom<Rep>,
        Rep: TimeInt,
    {
        if size_of::<Self::Rep>() > size_of::<Rep>() {
            let converted_ticks = Self::Rep::try_from(ticks).ok()?;

            if period > Period::new(1, 1) {
                Some(Self::new(TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&converted_ticks, &period)?,
                    &Self::PERIOD,
                )?))
            } else {
                Some(Self::new(TimeInt::checked_mul_period(
                    &converted_ticks,
                    &<Period as num::CheckedDiv>::checked_div(&period, &Self::PERIOD)?,
                )?))
            }
        } else {
            let ticks = if period > Period::new(1, 1) {
                TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&ticks, &period)?,
                    &Self::PERIOD,
                )?
            } else if Self::PERIOD > Period::new(1, 1) {
                TimeInt::checked_mul_period(
                    &TimeInt::checked_div_period(&ticks, &Self::PERIOD)?,
                    &period,
                )?
            } else {
                TimeInt::checked_mul_period(
                    &ticks,
                    &<Period as num::CheckedDiv>::checked_div(&period, &Self::PERIOD)?,
                )?
            };

            let converted_ticks = Self::Rep::try_from(ticks).ok()?;
            Some(Self::new(converted_ticks))
        }
    }

    /// Create an integer representation with LSbit period of that provided
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*, Period};
    /// assert_eq!(Microseconds(5_000_i32).into_ticks::<i32>(Period::new(1, 1_000)), Some(5_i32));
    ///
    /// // the _into_ period can be any value
    /// assert_eq!(Microseconds(5_000_i32).into_ticks::<i32>(Period::new(1, 200)), Some(1_i32));
    ///
    /// // as long as the result fits in the provided integer, it will succeed
    /// assert_eq!(Microseconds::<i32>(i32::MAX).into_ticks::<i64>(Period::new(1, 2_000_000)),
    ///     Some((i32::MAX as i64) * 2));
    /// ```
    ///
    /// # Errors
    /// the conversion of periods causes an overflow:
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*, Period};
    /// assert_eq!(Seconds(i32::MAX).into_ticks::<i32>(Period::new(1, 1_000)), None);
    /// ```
    ///
    /// the Self integer cast to that of the provided type fails
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*, Period};
    /// assert_eq!(Seconds(i32::MAX as i64 + 1).into_ticks::<i32>(Period::new(1, 1)), None);
    /// ```
    ///
    /// # Returns
    /// [`None`] if the result of the conversion does not fit in the requested integer size
    fn into_ticks<Rep>(self, period: Period) -> Option<Rep>
    where
        Self::Rep: TimeInt,
        Rep: TimeInt + TryFrom<Self::Rep>,
    {
        if size_of::<Rep>() > size_of::<Self::Rep>() {
            let ticks = Rep::try_from(self.count()).ok()?;

            if period > Period::new(1, 1) {
                Some(TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&ticks, &Self::PERIOD)?,
                    &period,
                )?)
            } else {
                Some(TimeInt::checked_mul_period(
                    &ticks,
                    &<Period as num::CheckedDiv>::checked_div(&Self::PERIOD, &period)?,
                )?)
            }
        } else {
            let ticks = if Self::PERIOD > Period::new(1, 1) {
                TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&self.count(), &Self::PERIOD)?,
                    &period,
                )?
            } else {
                TimeInt::checked_mul_period(
                    &self.count(),
                    &<Period as num::CheckedDiv>::checked_div(&Self::PERIOD, &period)?,
                )?
            };

            Rep::try_from(ticks).ok()
        }
    }

    /// ```rust
    /// # use embedded_time::{prelude::*, units::*};
    /// assert_eq!(Seconds::<i32>::min_value(), i32::MIN);
    /// ```
    #[must_use]
    fn min_value() -> Self::Rep {
        Self::Rep::min_value()
    }

    /// ```rust
    /// # use embedded_time::{prelude::*, units::*};
    /// assert_eq!(Seconds::<i32>::max_value(), i32::MAX);
    /// ```
    #[must_use]
    fn max_value() -> Self::Rep {
        Self::Rep::max_value()
    }
}

/// Attempt to convert from one duration type to another
///
/// This is basically a specialization of the [`TryFrom`](core::convert::TryFrom) trait.
pub trait TryConvertFrom<Source>: Sized {
    /// Perform the conversion
    fn try_convert_from(other: Source) -> Option<Self>;
}

/// Attempt to convert from one duration type to another
///
/// This is basically a specialization of the [`TryInto`](core::convert::TryInto) trait.
pub trait TryConvertInto<Dest> {
    /// Perform the conversion
    fn try_convert_into(self) -> Option<Dest>;
}

impl<Source, Dest> TryConvertFrom<Source> for Dest
where
    Dest: Duration,
    Dest::Rep: TimeInt + TryFrom<Source::Rep>,
    Source: Duration,
    Source::Rep: TimeInt,
{
    /// Attempt to convert from one duration type to another
    ///
    /// Both the underlying storage type and/or the LSbit period can be converted
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*};
    /// assert_eq!(Seconds::<i32>::try_convert_from(Milliseconds(23_000_i64)), Some(Seconds(23_i32)));
    /// assert_eq!(Seconds::<i64>::try_convert_from(Milliseconds(23_000_i32)), Some(Seconds(23_i64)));
    /// assert_eq!(Seconds::<i32>::try_convert_from(Milliseconds(230_i32)), Some(Seconds(0)));
    /// ```
    ///
    /// # Errors
    /// the conversion of periods causes an overflow:
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*};
    /// assert_eq!(Milliseconds::<i32>::try_convert_from(Seconds(i32::MAX)), None);
    /// ```
    ///
    /// the Self integer cast to that of the provided type fails
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*};
    /// assert_eq!(Seconds::<i32>::try_convert_from(Seconds(i32::MAX as i64 + 1)), None);
    /// ```
    ///
    /// However, these work because the sequence of cast/conversion adapts
    /// ```rust
    /// # use embedded_time::{prelude::*, units::*};
    /// // period conversion applied first
    /// assert_eq!(Hours::<i32>::try_convert_from(Microseconds(3_600_000_000_i64)), Some(Hours(1_i32)));
    ///
    /// // cast applied first
    /// assert_eq!(Microseconds::<i64>::try_convert_from(Hours(1_i32)), Some(Microseconds(3_600_000_000_i64)));
    /// ```
    ///
    /// # Returns
    /// [`None`] if the result of the conversion does not fit in the requested integer size
    fn try_convert_from(source: Source) -> Option<Self> {
        Self::from_ticks(source.count(), Source::PERIOD)
    }
}

/// The reciprocal of [`TryConvertFrom`]
///
/// # Examples
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// assert_eq!(Seconds(23_i64).try_convert_into(), Some(Seconds(23_i32)));
/// assert_eq!(Some(Seconds(23_i64)), (Seconds(23_i32).try_convert_into()));
/// assert_eq!(Milliseconds(23_000_i64).try_convert_into(), Some(Seconds(23_i32)));
/// ```
///
/// # Errors
/// the conversion of periods causes an overflow:
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// assert_eq!(Seconds(i32::MAX).try_convert_into(), None::<Milliseconds<i32>>);
/// ```
///
/// the Self integer cast to that of the destination type fails
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// assert_eq!(Seconds(i32::MAX as i64 + 1).try_convert_into(), None::<Seconds<i32>>);
/// ```
///
/// However, these work because the sequence of cast/conversion adapts
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// // period conversion applied first
/// assert_eq!(Microseconds(3_600_000_000_i64).try_convert_into(), Some(Hours(1_i32)));
///
/// // cast applied first
/// assert_eq!(Hours(1_i32).try_convert_into(), Some(Microseconds(3_600_000_000_i64)));
/// ```
///
/// # Returns
/// [`None`] if the result of the conversion does not fit in the requested [`Duration`] type
impl<Source, Dest> TryConvertInto<Dest> for Source
where
    Source: Duration,
    Dest: Duration + TryConvertFrom<Source>,
{
    fn try_convert_into(self) -> Option<Dest> {
        Dest::try_convert_from(self)
    }
}

pub mod units {
    use crate::{
        duration::{Duration, TryConvertFrom},
        time_int::TimeInt,
        Period,
    };
    use core::{
        cmp,
        convert::{self, TryInto as _},
        fmt::{self, Formatter},
        ops,
    };

    macro_rules! impl_duration {
        ( $name:ident, ($numer:expr, $denom:expr) ) => {
            /// A duration unit type
            #[derive(Copy, Clone, Debug, Eq, Ord)]
            pub struct $name<T: TimeInt>(pub T);

            impl<Rep: TimeInt> Duration for $name<Rep> {
                type Rep = Rep;
                const PERIOD: Period = Period::new($numer, $denom);

                fn new(value: Self::Rep) -> Self {
                    Self(value)
                }

                fn count(self) -> Self::Rep {
                    self.0
                }
            }

            impl<T: TimeInt> fmt::Display for $name<T> {
                /// See module-level documentation for details about this type
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    fmt::Display::fmt(&self.0, f)
                }
            }

            impl<Rep, RhsDur> ops::Add<RhsDur> for $name<Rep>
            where
                RhsDur: Duration,
                RhsDur::Rep: TimeInt,
                Rep: TimeInt + convert::TryFrom<RhsDur::Rep>,
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
                Rep: TimeInt + convert::TryFrom<RhsDur::Rep>,
                RhsDur: Duration,
            {
                type Output = Self;

                /// See module-level documentation for details about this type
                #[inline]
                fn sub(self, rhs: RhsDur) -> Self::Output {
                    Self(self.count() - Self::try_convert_from(rhs).unwrap().count())
                }
            }

            impl<Rep, Dur> ops::Rem<Dur> for $name<Rep>
            where
                Rep: TimeInt + convert::TryFrom<Dur::Rep>,
                Dur: Duration,
            {
                type Output = Self;

                fn rem(self, rhs: Dur) -> Self::Output {
                    let rhs = <Self as TryConvertFrom<Dur>>::try_convert_from(rhs)
                        .unwrap()
                        .count();

                    if rhs > Rep::from(0) {
                        Self(self.count() % rhs)
                    } else {
                        Self(Rep::from(0))
                    }
                }
            }

            impl<Rep, OtherDur> cmp::PartialEq<OtherDur> for $name<Rep>
            where
                Rep: TimeInt + convert::TryFrom<OtherDur::Rep>,
                OtherDur: Duration,
                OtherDur::Rep: convert::TryFrom<Rep>,
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
                Rep: TimeInt + convert::TryFrom<OtherDur::Rep>,
                OtherDur: Duration,
                OtherDur::Rep: convert::TryFrom<Rep>,
            {
                /// See module-level documentation for details about this type
                fn partial_cmp(&self, other: &OtherDur) -> Option<core::cmp::Ordering> {
                    if Self::PERIOD < OtherDur::PERIOD {
                        Some(
                            self.count()
                                .cmp(&Self::try_convert_from(*other).unwrap().count()),
                        )
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
        };
        ( $name:ident, ($numer:expr, $denom:expr), ge_secs ) => {
            impl_duration![$name, ($numer, $denom)];

            impl<Rep: TimeInt> convert::TryFrom<$name<Rep>> for core::time::Duration {
                type Error = ();

                /// Convert an embedded_time::[`Duration`] into a [`core::time::Duration`]
                fn try_from(duration: $name<Rep>) -> Result<Self, Self::Error> {
                    let seconds = Seconds::<i64>::try_convert_from(duration).ok_or(())?;
                    Ok(Self::from_secs(seconds.count().try_into().or(Err(()))?))
                }
            }

            impl<Rep: TimeInt> convert::TryFrom<core::time::Duration> for $name<Rep> {
                type Error = ();

                /// Convert a [`core::time::Duration`] into an embedded_time::[`Duration`]
                fn try_from(core_duration: core::time::Duration) -> Result<Self, Self::Error> {
                    let seconds = Seconds::<i64>(core_duration.as_secs().try_into().or(Err(()))?);
                    Ok(Self::try_convert_from(seconds).ok_or(())?)
                }
            }
        };
        ( $name:ident, ($numer:expr, $denom:expr), $from_core_dur:ident, $as_core_dur:ident ) => {
            impl_duration![$name, ($numer, $denom)];

            impl<Rep: TimeInt> convert::TryFrom<$name<Rep>> for core::time::Duration {
                type Error = <Rep as convert::TryInto<u64>>::Error;

                /// Convert an embedded_time::[`Duration`] into a [`core::time::Duration`]
                fn try_from(duration: $name<Rep>) -> Result<Self, Self::Error> {
                    Ok(Self::$from_core_dur(duration.count().try_into()?))
                }
            }

            impl<Rep: TimeInt> convert::TryFrom<core::time::Duration> for $name<Rep> {
                type Error = <Rep as convert::TryFrom<u128>>::Error;

                /// Convert a [`core::time::Duration`] into an embedded_time::[`Duration`]
                fn try_from(core_duration: core::time::Duration) -> Result<Self, Self::Error> {
                    Ok(Self(core_duration.$as_core_dur().try_into()?))
                }
            }
        };
    }
    impl_duration![Hours, (3600, 1), ge_secs];
    impl_duration![Minutes, (60, 1), ge_secs];
    impl_duration![Seconds, (1, 1), ge_secs];
    impl_duration![Milliseconds, (1, 1_000), from_millis, as_millis];
    impl_duration![Microseconds, (1, 1_000_000), from_micros, as_micros];
    impl_duration![Nanoseconds, (1, 1_000_000_000), from_nanos, as_nanos];
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryInto;
    use units::*;

    #[test]
    fn check_for_overflows() {
        let mut time = 1_i64;
        time *= 60;
        assert_eq!(Hours(1), Minutes(time));
        time *= 60;
        assert_eq!(Hours(1), Seconds(time));
        time *= 1000;
        assert_eq!(Hours(1), Milliseconds(time));
        time *= 1000;
        assert_eq!(Hours(1), Microseconds(time));
        time *= 1000;
        assert_eq!(Hours(1), Nanoseconds(time));
    }

    #[test]
    fn remainder() {
        assert_eq!(Minutes(62) % Hours(1), Minutes(2));
        assert_eq!(Minutes(62) % Milliseconds(1), Minutes(0));
        assert_eq!(Minutes(62) % Minutes(60), Minutes(2));
    }

    #[test]
    fn convert_from_core_duration() {
        let core_duration = core::time::Duration::from_nanos(5_025_678_901_234);
        assert_eq!(
            core_duration.try_into(),
            Ok(Nanoseconds::<i64>(5_025_678_901_234))
        );
        assert_eq!(
            core_duration.try_into(),
            Ok(Microseconds::<i64>(5_025_678_901))
        );
        assert_eq!(core_duration.try_into(), Ok(Milliseconds::<i32>(5_025_678)));
        assert_eq!(core_duration.try_into(), Ok(Seconds::<i32>(5_025)));
        assert_eq!(core_duration.try_into(), Ok(Minutes::<i32>(83)));
        assert_eq!(core_duration.try_into(), Ok(Hours::<i32>(1)));
    }

    #[test]
    fn convert_to_core_duration() {
        assert_eq!(
            Nanoseconds(123).try_into(),
            Ok(core::time::Duration::from_nanos(123))
        );
        assert_eq!(
            Microseconds(123).try_into(),
            Ok(core::time::Duration::from_micros(123))
        );
        assert_eq!(
            Milliseconds(123).try_into(),
            Ok(core::time::Duration::from_millis(123))
        );
        assert_eq!(
            Seconds(123).try_into(),
            Ok(core::time::Duration::from_secs(123))
        );
        assert_eq!(
            Minutes(123).try_into(),
            Ok(core::time::Duration::from_secs(123 * 60))
        );
        assert_eq!(
            Hours(123).try_into(),
            Ok(core::time::Duration::from_secs(123 * 3600))
        );
    }
}
