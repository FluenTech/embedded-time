//! Duration types/units creation and conversion.

use crate::{period::Period, time_int::TimeInt, Error, TimeError};
use core::{convert::TryFrom, fmt, mem::size_of, prelude::v1::*};
use num::{Bounded, CheckedDiv};

/// An unsigned duration of time
///
/// Each implementation defines a constant [`Period`] which is a fraction/ratio representing the
/// period of the count's LSbit
///
///
/// # Constructing a duration
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!(Milliseconds::<u32>::new(23), Milliseconds(23_u32));
/// assert_eq!(Milliseconds(23_u32), 23_u32.milliseconds());
/// ```
///
/// # Get the integer count
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!(Milliseconds(23_u32).count(), 23_u32);
/// ```
///
/// # Formatting
/// Just forwards the underlying integer to [`core::fmt::Display::fmt()`]
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!(format!("{}", Seconds(123_u32)), "123");
/// ```
///
/// # Getting H:M:S.MS... Components
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// let duration = 38_238_479_u32.microseconds();
/// let hours = Hours::<u32>::try_convert_from(duration).unwrap();
/// let minutes = Minutes::<u32>::try_convert_from(duration).unwrap() % Hours(1_u32);
/// let seconds = Seconds::<u32>::try_convert_from(duration).unwrap() % Minutes(1_u32);
/// let milliseconds = Milliseconds::<u32>::try_convert_from(duration).unwrap() % Seconds(1_u32);
/// // ...
/// ```
///
/// # Converting to `core` types
/// [`core::time::Duration`]
///
/// ## Examples
/// ```rust
/// # use embedded_time::traits::*;
/// # use core::convert::TryFrom;
/// #
/// let core_duration = core::time::Duration::try_from(2_569_u32.milliseconds()).unwrap();
/// assert_eq!(2, core_duration.as_secs());
/// assert_eq!(569_000_000, core_duration.subsec_nanos());
/// ```
/// ```rust
/// # use embedded_time::traits::*;
/// # use core::convert::TryInto;
/// #
/// let core_duration: core::time::Duration = 2_569_u32.milliseconds().try_into().unwrap();
/// assert_eq!(2, core_duration.as_secs());
/// assert_eq!(569_000_000, core_duration.subsec_nanos());
/// ```
///
/// # Converting from `core` types
/// [`core::time::Duration`]
///
/// ## Examples
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// # use core::convert::TryFrom;
/// #
/// let core_duration = core::time::Duration::new(5, 730023852);
/// assert_eq!(Milliseconds::<u32>::try_from(core_duration), Ok(5_730.milliseconds()));
/// ```
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// # use core::convert::TryInto;
/// #
/// let duration: Result<Milliseconds<u32>, _> = core::time::Duration::new(5, 730023852).try_into();
/// assert_eq!(duration, Ok(5_730.milliseconds()));
/// ```
///
/// ## Errors
/// The duration doesn't fit in the type specified
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// # use core::convert::{TryFrom, TryInto};
/// #
/// assert!(Milliseconds::<u32>::try_from(core::time::Duration::from_millis((u32::MAX as u64) + 1)).is_err());
///
/// let duration: Result<Milliseconds<u32>, _> = core::time::Duration::from_millis((u32::MAX as u64) + 1).try_into();
/// assert!(duration.is_err());
/// ```
///
/// # Add/Sub
///
/// ## Panics
/// Panics if the rhs duration cannot be converted into the lhs duration type
///
/// In this example, the maximum `u32` value of seconds is stored as `u32` and
/// converting that value to milliseconds (with `u32` storage type) causes an overflow.
/// ```rust,should_panic
/// # use embedded_time::{traits::*, units::*};
/// #
/// let _ = Milliseconds(24_u32) + Seconds(u32::MAX);
/// ```
///
/// This example works just fine as the seconds value is first cast to `u64`, then
/// converted to milliseconds.
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// let _ = Milliseconds(24_u64) + Seconds(u32::MAX);
/// ```
///
/// Here, there is no units conversion to worry about, but `u32::MAX + 1` cannot be
/// cast to an `u32`.
/// ```rust,should_panic
/// # use embedded_time::{traits::*, units::*};
/// #
/// let _ = Seconds(u32::MAX) - Seconds(u32::MAX as u64 + 1);
/// ```
///
/// ## Examples
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!((Milliseconds(3_234_u32) - Seconds(2_u32)), Milliseconds(1_234_u32));
/// assert_eq!((Milliseconds(3_234_u64) - Seconds(2_u32)), Milliseconds(1_234_u64));
/// assert_eq!((Seconds(u32::MAX) - Milliseconds((u32::MAX as u64) + 1)),
///     Seconds(4_290_672_328_u32));
/// ```
///
/// # Equality
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!(Seconds(123_u32), Seconds(123_u32));
/// assert_eq!(Seconds(123_u32), Milliseconds(123_000_u32));
///
/// assert_ne!(Seconds(123_u32), Milliseconds(123_001_u32));
/// assert_ne!(Milliseconds(123_001_u32), Seconds(123_u32));
/// assert_ne!(Milliseconds(123_001_u64), Seconds(123_u64));
/// assert_ne!(Seconds(123_u64), Milliseconds(123_001_u64));
/// assert_ne!(Seconds(123_u64), Milliseconds(123_001_u32));
/// ```
///
/// # Comparisons
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert!(Seconds(2_u32) < Seconds(3_u32));
/// assert!(Seconds(2_u32) < Milliseconds(2_001_u32));
/// assert!(Seconds(2_u32) == Milliseconds(2_000_u32));
/// assert!(Seconds(2_u32) > Milliseconds(1_999_u32));
/// assert!(Seconds(2_u32) < Milliseconds(2_001_u64));
/// assert!(Seconds(2_u64) < Milliseconds(2_001_u32));
/// ```
///
/// # Remainder
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
/// ```
pub trait Duration: Sized + Copy + fmt::Display {
    /// The inner type of the `Duration` representing the count of the implementation unit
    type Rep: TimeInt;

    /// A fraction/ratio representing the period of the count's LSbit. The precision of the
    /// `Duration`.
    const PERIOD: Period;

    /// Not generally useful or needed as the duration can be constructed like this:
    /// ```no_run
    /// # use embedded_time::{traits::*, units::*};
    /// Seconds(123_u32);
    /// 123_u32.seconds();
    /// ```
    /// It only exists to allow Duration methods with default definitions to create a
    /// new duration
    fn new(value: Self::Rep) -> Self;

    /// Returns the integer value of the `Duration`
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{traits::*, units::*};
    /// assert_eq!(Seconds(123_u32).count(), 123_u32);
    /// ```
    fn count(self) -> Self::Rep;

    /// Constructs a `Duration` from a value of ticks and a period
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{traits::*, units::*, Period};
    /// assert_eq!(Microseconds::<u32>::from_ticks(5_u64, <Period>::new(1, 1_000)),
    ///     Ok(Microseconds(5_000_u32)));
    ///
    /// // the conversion arithmetic will not cause overflow
    /// assert_eq!(Milliseconds::<u32>::from_ticks((u32::MAX as u64) + 1, <Period>::new(1, 1_000_000)),
    ///     Ok(Milliseconds((((u32::MAX as u64) + 1) / 1_000) as u32)));
    /// ```
    ///
    /// # Errors
    /// the conversion of periods causes an overflow:
    /// ```rust
    /// # use embedded_time::{traits::*, units::*, Period, TimeError};
    /// assert_eq!(Milliseconds::<u32>::from_ticks(u32::MAX, <Period>::new(1, 1)),
    ///     Err(TimeError::WouldOverflow));
    /// ```
    ///
    /// the Self integer cast to that of the provided type fails
    /// ```rust
    /// # use embedded_time::{traits::*, units::*, Period, TimeError};
    /// assert_eq!(Seconds::<u32>::from_ticks(u32::MAX as u64 + 1, <Period>::new(1, 1)),
    ///     Err(TimeError::CastWouldFail));
    /// ```
    fn from_ticks<Rep: TimeInt, E: Error>(ticks: Rep, period: Period) -> Result<Self, TimeError<E>>
    where
        Self::Rep: TryFrom<Rep>,
    {
        if size_of::<Self::Rep>() > size_of::<Rep>() {
            let converted_ticks =
                Self::Rep::try_from(ticks).map_err(|_| TimeError::CastWouldFail)?;

            if period > <Period>::new(1, 1) {
                Ok(Self::new(TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&converted_ticks, &period)?,
                    &Self::PERIOD,
                )?))
            } else {
                Ok(Self::new(TimeInt::checked_mul_period(
                    &converted_ticks,
                    &<Period as num::CheckedDiv>::checked_div(&period, &Self::PERIOD)
                        .ok_or(TimeError::WouldDivByZero)?,
                )?))
            }
        } else {
            let ticks = if period > <Period>::new(1, 1) {
                TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&ticks, &period)?,
                    &Self::PERIOD,
                )?
            } else if Self::PERIOD > <Period>::new(1, 1) {
                TimeInt::checked_mul_period(
                    &TimeInt::checked_div_period(&ticks, &Self::PERIOD)?,
                    &period,
                )?
            } else {
                TimeInt::checked_mul_period(
                    &ticks,
                    &<Period as num::CheckedDiv>::checked_div(&period, &Self::PERIOD)
                        .ok_or(TimeError::WouldDivByZero)?,
                )?
            };

            let converted_ticks =
                Self::Rep::try_from(ticks).map_err(|_| TimeError::CastWouldFail)?;
            Ok(Self::new(converted_ticks))
        }
    }

    /// Create an integer representation with LSbit period of that provided
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{traits::*, units::*, Period, TimeError};
    /// assert_eq!(Microseconds(5_000_u32).into_ticks::<u32>(Period::new(1, 1_000)), Ok(5_u32));
    ///
    /// // the _into_ period can be any value
    /// assert_eq!(Microseconds(5_000_u32).into_ticks::<u32>(Period::new(1, 200)), Ok(1_u32));
    ///
    /// // as long as the result fits in the provided integer, it will succeed
    /// assert_eq!(Microseconds::<u32>(u32::MAX).into_ticks::<u64>(Period::new(1, 2_000_000)),
    ///     Ok((u32::MAX as u64) * 2));
    /// ```
    ///
    /// # Errors
    /// the conversion of periods causes an overflow:
    /// ```rust
    /// # use embedded_time::{traits::*, units::*, Period, TimeError};
    /// assert_eq!(Seconds(u32::MAX).into_ticks::<u32>(Period::new(1, 1_000)), Err(TimeError::WouldOverflow));
    /// ```
    ///
    /// the Self integer cast to that of the provided type fails
    /// ```rust
    /// # use embedded_time::{traits::*, units::*, Period, TimeError};
    /// assert_eq!(Seconds(u32::MAX as u64 + 1).into_ticks::<u32>(Period::new(1, 1)), Err(TimeError::CastWouldFail));
    /// ```
    fn into_ticks<Rep: TimeInt, E: crate::Error>(self, period: Period) -> Result<Rep, TimeError<E>>
    where
        Self::Rep: TimeInt,
        Rep: TryFrom<Self::Rep>,
    {
        if size_of::<Rep>() > size_of::<Self::Rep>() {
            let ticks = Rep::try_from(self.count()).map_err(|_| TimeError::CastWouldFail)?;

            if period > <Period>::new(1, 1) {
                TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&ticks, &Self::PERIOD)?,
                    &period,
                )
            } else {
                TimeInt::checked_mul_period(
                    &ticks,
                    &<Period as num::CheckedDiv>::checked_div(&Self::PERIOD, &period)
                        .ok_or(TimeError::WouldDivByZero)?,
                )
            }
        } else {
            let ticks = if Self::PERIOD > <Period>::new(1, 1) {
                TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&self.count(), &Self::PERIOD)?,
                    &period,
                )?
            } else {
                TimeInt::checked_mul_period(
                    &self.count(),
                    &<Period as num::CheckedDiv>::checked_div(&Self::PERIOD, &period)
                        .ok_or(TimeError::WouldDivByZero)?,
                )?
            };

            Rep::try_from(ticks).map_err(|_| TimeError::CastWouldFail)
        }
    }

    /// ```rust
    /// # use embedded_time::{traits::*, units::*};
    /// assert_eq!(Seconds::<u32>::min_value(), u32::MIN);
    /// ```
    #[must_use]
    fn min_value() -> Self::Rep {
        Self::Rep::min_value()
    }

    /// ```rust
    /// # use embedded_time::{traits::*, units::*};
    /// assert_eq!(Seconds::<u32>::max_value(), u32::MAX);
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
    fn try_convert_from<E: Error>(other: Source) -> Result<Self, TimeError<E>>;
}

/// Attempt to convert from one duration type to another
///
/// This is basically a specialization of the [`TryInto`](core::convert::TryInto) trait.
pub trait TryConvertInto<Dest> {
    /// Perform the conversion
    fn try_convert_into<E: Error>(self) -> Result<Dest, TimeError<E>>;
}

impl<Source: Duration, Dest: Duration> TryConvertFrom<Source> for Dest
where
    Dest::Rep: TryFrom<Source::Rep>,
{
    /// Attempt to convert from one duration type to another
    ///
    /// Both the underlying storage type and/or the LSbit period can be converted
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{traits::*, units::*};
    /// assert_eq!(Seconds::<u32>::try_convert_from(Milliseconds(23_000_u64)), Ok(Seconds(23_u32)));
    /// assert_eq!(Seconds::<u64>::try_convert_from(Milliseconds(23_000_u32)), Ok(Seconds(23_u64)));
    /// assert_eq!(Seconds::<u32>::try_convert_from(Milliseconds(230_u32)), Ok(Seconds(0)));
    /// ```
    ///
    /// # Errors
    /// the conversion of periods causes an overflow:
    /// ```rust
    /// # use embedded_time::{traits::*, units::*, TimeError};
    /// assert_eq!(Milliseconds::<u32>::try_convert_from(Seconds(u32::MAX)), Err(TimeError::WouldOverflow));
    /// ```
    ///
    /// the Self integer cast to that of the provided type fails
    /// ```rust
    /// # use embedded_time::{traits::*, units::*, TimeError};
    /// assert_eq!(Seconds::<u32>::try_convert_from(Seconds(u32::MAX as u64 + 1)), Err(TimeError::CastWouldFail));
    /// ```
    ///
    /// However, these work because the sequence of cast/conversion adapts
    /// ```rust
    /// # use embedded_time::{traits::*, units::*};
    /// // period conversion applied first
    /// assert_eq!(Hours::<u32>::try_convert_from(Microseconds(3_600_000_000_u64)), Ok(Hours(1_u32)));
    ///
    /// // cast applied first
    /// assert_eq!(Microseconds::<u64>::try_convert_from(Hours(1_u32)), Ok(Microseconds(3_600_000_000_u64)));
    /// ```
    ///
    /// # Returns
    /// [`None`] if the result of the conversion does not fit in the requested integer size
    fn try_convert_from<E: Error>(source: Source) -> Result<Self, TimeError<E>> {
        Self::from_ticks(source.count(), Source::PERIOD)
    }
}

/// The reciprocal of [`TryConvertFrom`]
///
/// # Examples
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// assert_eq!(Seconds(23_u64).try_convert_into(), Ok(Seconds(23_u32)));
/// assert_eq!(Ok(Seconds(23_u64)), (Seconds(23_u32).try_convert_into()));
/// assert_eq!(Milliseconds(23_000_u64).try_convert_into(), Ok(Seconds(23_u32)));
/// ```
///
/// # Errors
/// the conversion of periods causes an overflow:
/// ```rust
/// # use embedded_time::{traits::*, units::*, TimeError};
/// assert_eq!(Seconds(u32::MAX).try_convert_into(), Err(TimeError::WouldOverflow));
/// ```
///
/// the Self integer cast to that of the destination type fails
/// ```rust
/// # use embedded_time::{traits::*, units::*, TimeError};
/// assert_eq!(Seconds(u32::MAX as u64 + 1).try_convert_into(), Err(TimeError::CastWouldFail));
/// ```
///
/// However, these work because the sequence of cast/conversion adapts
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// // period conversion applied first
/// assert_eq!(Microseconds(3_600_000_000_u64).try_convert_into(), Ok(Hours(1_u32)));
///
/// // cast applied first
/// assert_eq!(Hours(1_u32).try_convert_into(), Ok(Microseconds(3_600_000_000_u64)));
/// ```
///
/// # Returns
/// [`None`] if the result of the conversion does not fit in the requested [`Duration`] type
impl<Source, Dest> TryConvertInto<Dest> for Source
where
    Source: Duration,
    Dest: Duration + TryConvertFrom<Source>,
{
    fn try_convert_into<E: Error>(self) -> Result<Dest, TimeError<E>> {
        Dest::try_convert_from(self)
    }
}

#[doc(hidden)]
pub mod units {
    use crate::{
        duration::{Duration, TryConvertFrom},
        period::Period,
        time_int::TimeInt,
        TimeError,
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
            pub struct $name<T: TimeInt = u32>(pub T);

            impl<Rep: TimeInt> Duration for $name<Rep> {
                type Rep = Rep;
                const PERIOD: Period = <Period>::new($numer, $denom);

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

            // impl<Rep, RhsDur> ops::Add<RhsDur> for $name<Rep>
            // where
            //     RhsDur: Duration,
            //     RhsDur::Rep: TimeInt,
            //     Rep: TimeInt + convert::TryFrom<RhsDur::Rep>,
            // {
            //     type Output = Result<Self, Error>;
            //
            //     /// See module-level documentation for details about this type
            //     #[inline]
            //     fn add(self, rhs: RhsDur) -> Self::Output {
            //         Self(self.count() + Self::try_convert_from(rhs)?.count())
            //     }
            // }

            // impl<Rep, RhsDur> ops::Sub<RhsDur> for $name<Rep>
            // where
            //     Rep: TimeInt + convert::TryFrom<RhsDur::Rep>,
            //     RhsDur: Duration,
            // {
            //     type Output = Self;
            //
            //     /// See module-level documentation for details about this type
            //     #[inline]
            //     fn sub(self, rhs: RhsDur) -> Self::Output {
            //         Self(self.count() - Self::try_convert_from(rhs).unwrap().count())
            //     }
            // }

            // impl<Rep, Dur> ops::Rem<Dur> for $name<Rep>
            // where
            //     Rep: TimeInt + convert::TryFrom<Dur::Rep>,
            //     Dur: Duration,
            // {
            //     type Output = Self;
            //
            //     fn rem(self, rhs: Dur) -> Self::Output {
            //         let rhs = <Self as TryConvertFrom<Dur>>::try_convert_from(rhs)
            //             .unwrap()
            //             .count();
            //
            //         if rhs > Rep::from(0) {
            //             Self(self.count() % rhs)
            //         } else {
            //             Self(Rep::from(0))
            //         }
            //     }
            // }

            impl<Rep, OtherDur> cmp::PartialEq<OtherDur> for $name<Rep>
            where
                Rep: TimeInt + convert::TryFrom<OtherDur::Rep>,
                OtherDur: Duration,
                OtherDur::Rep: convert::TryFrom<Rep>,
            {
                /// See module-level documentation for details about this type
                fn eq(&self, other: &OtherDur) -> bool {
                    if Self::PERIOD < OtherDur::PERIOD {
                        self.count() == Self::try_convert_from::<()>(*other).unwrap().count()
                    } else {
                        OtherDur::try_convert_from::<()>(*self).unwrap().count() == other.count()
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
                                .cmp(&Self::try_convert_from::<()>(*other).unwrap().count()),
                        )
                    } else {
                        Some(
                            OtherDur::try_convert_from::<()>(*self)
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
                type Error = TimeError;

                /// Convert an embedded_time::[`Duration`] into a [`core::time::Duration`]
                fn try_from(duration: $name<Rep>) -> Result<Self, Self::Error> {
                    let seconds = Seconds::<u64>::try_convert_from(duration)?;
                    Ok(Self::from_secs(seconds.count()))
                }
            }

            impl<Rep: TimeInt> convert::TryFrom<core::time::Duration> for $name<Rep> {
                type Error = TimeError;

                /// Convert a [`core::time::Duration`] into an embedded_time::[`Duration`]
                fn try_from(core_duration: core::time::Duration) -> Result<Self, Self::Error> {
                    let seconds = Seconds(core_duration.as_secs());
                    Self::try_convert_from(seconds)
                }
            }
        };
        ( $name:ident, ($numer:expr, $denom:expr), $from_core_dur:ident, $as_core_dur:ident ) => {
            impl_duration![$name, ($numer, $denom)];

            impl<Rep: TimeInt> convert::TryFrom<$name<Rep>> for core::time::Duration {
                type Error = TimeError;

                /// Convert an embedded_time::[`Duration`] into a [`core::time::Duration`]
                fn try_from(duration: $name<Rep>) -> Result<Self, Self::Error> {
                    Ok(Self::$from_core_dur(duration.count().into()))
                }
            }

            impl<Rep: TimeInt> convert::TryFrom<core::time::Duration> for $name<Rep> {
                type Error = TimeError;

                /// Convert a [`core::time::Duration`] into an embedded_time::[`Duration`]
                fn try_from(core_duration: core::time::Duration) -> Result<Self, Self::Error> {
                    Ok(Self(
                        core_duration
                            .$as_core_dur()
                            .try_into()
                            .map_err(|_| TimeError::CastWouldFail)?,
                    ))
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

    // #[test]
    // fn check_for_overflows() {
    //     let mut time = 1_u64;
    //     time *= 60;
    //     assert_eq!(Hours(1_u32), Minutes(time));
    //     time *= 60;
    //     assert_eq!(Hours(1_u32), Seconds(time));
    //     time *= 1000;
    //     assert_eq!(Hours(1_u32), Milliseconds(time));
    //     time *= 1000;
    //     assert_eq!(Hours(1_u32), Microseconds(time));
    //     time *= 1000;
    //     assert_eq!(Hours(1_u32), Nanoseconds(time));
    // }
    //
    // #[test]
    // fn remainder() {
    //     assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
    //     assert_eq!(Minutes(62_u32) % Milliseconds(1_u32), Minutes(0_u32));
    //     assert_eq!(Minutes(62_u32) % Minutes(60_u32), Minutes(2_u32));
    // }
    //
    // #[test]
    // fn convert_from_core_duration() {
    //     let core_duration = core::time::Duration::from_nanos(5_025_678_901_234);
    //     assert_eq!(
    //         core_duration.try_into(),
    //         Ok(Nanoseconds::<u64>(5_025_678_901_234))
    //     );
    //     assert_eq!(
    //         core_duration.try_into(),
    //         Ok(Microseconds::<u64>(5_025_678_901))
    //     );
    //     assert_eq!(core_duration.try_into(), Ok(Milliseconds::<u32>(5_025_678)));
    //     assert_eq!(core_duration.try_into(), Ok(Seconds::<u32>(5_025)));
    //     assert_eq!(core_duration.try_into(), Ok(Minutes::<u32>(83)));
    //     assert_eq!(core_duration.try_into(), Ok(Hours::<u32>(1)));
    // }
    //
    // #[test]
    // fn convert_to_core_duration() {
    //     assert_eq!(
    //         Nanoseconds(123_u32).try_into(),
    //         Ok(core::time::Duration::from_nanos(123))
    //     );
    //     assert_eq!(
    //         Microseconds(123_u32).try_into(),
    //         Ok(core::time::Duration::from_micros(123))
    //     );
    //     assert_eq!(
    //         Milliseconds(123_u32).try_into(),
    //         Ok(core::time::Duration::from_millis(123))
    //     );
    //     assert_eq!(
    //         Seconds(123_u32).try_into(),
    //         Ok(core::time::Duration::from_secs(123))
    //     );
    //     assert_eq!(
    //         Minutes(123_u32).try_into(),
    //         Ok(core::time::Duration::from_secs(123 * 60))
    //     );
    //     assert_eq!(
    //         Hours(123_u32).try_into(),
    //         Ok(core::time::Duration::from_secs(123 * 3600))
    //     );
    // }
}
