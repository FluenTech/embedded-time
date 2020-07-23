//! Duration types/units

use crate::{
    fixed_point::{self, FixedPoint},
    rate,
    time_int::TimeInt,
    ConversionError, Fraction,
};
use core::{convert::TryFrom, prelude::v1::*};

/// An unsigned, fixed-point duration type
///
/// Each implementation defines an _integer_ type and a [`Fraction`] _scaling factor_.
///
/// # Constructing a duration
///
/// ```rust
/// # use embedded_time::{traits::*, duration::units::*};
/// #
/// assert_eq!(23_u32.milliseconds(), Milliseconds(23_u32));
/// ```
///
/// # Getting H:M:S.MS... Components
///
/// ```rust
/// # use embedded_time::{traits::*, duration::units::*};
/// #
/// let duration = 38_238_479_u32.microseconds();
/// let hours = Hours::<u32>::try_convert_from(duration).unwrap();
/// let minutes = Minutes::<u32>::try_convert_from(duration).unwrap() % Hours(1_u32);
/// let seconds = Seconds::<u32>::try_convert_from(duration).unwrap() % Minutes(1_u32);
/// let milliseconds = Milliseconds::<u32>::try_convert_from(duration).unwrap() % Seconds(1_u32);
/// // ...
/// ```
pub trait Duration: Copy {
    /// Construct a `Generic` `Duration` from an _named_ `Duration` (eg.
    /// [`Milliseconds`](units::Milliseconds))
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, duration::units::*, duration::{Generic, Duration}};
    /// # use core::convert::{TryFrom, TryInto};
    /// #
    /// assert_eq!(Seconds(2_u64).try_into_generic(Fraction::new(1, 2_000)),
    ///     Ok(Generic::new(4_000_u32, Fraction::new(1, 2_000))));
    /// ```
    ///
    /// # Errors
    ///
    /// Failure will only occur if the provided value does not fit in the selected destination type.
    ///
    /// ---
    ///
    /// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, duration::units::*, duration::{Duration, Generic}, ConversionError};
    /// # use core::convert::TryFrom;
    /// #
    /// assert_eq!(Seconds(u32::MAX).try_into_generic::<u32>(Fraction::new(1, 2)),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// ---
    ///
    /// [`ConversionError::ConversionFailure`] : The integer conversion to that of the destination
    /// type fails.
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, duration::units::*, duration::{Duration, Generic}, ConversionError};
    /// # use core::convert::TryFrom;
    /// #
    /// assert_eq!(Seconds(u32::MAX as u64 + 1).try_into_generic::<u32>(Fraction::new(1, 1)),
    ///     Err(ConversionError::ConversionFailure));
    /// ```
    fn try_into_generic<DestInt: TimeInt>(
        self,
        scaling_factor: Fraction,
    ) -> Result<Generic<DestInt>, ConversionError>
    where
        Self: FixedPoint,
        DestInt: TryFrom<Self::T>,
    {
        Ok(Generic::<DestInt>::new(
            self.into_ticks(scaling_factor)?,
            scaling_factor,
        ))
    }

    /// Attempt to construct the given _duration_ type from the given _rate_ type
    ///
    /// (the duration is equal to the reciprocal of the rate)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{duration::{Duration, units::*}, rate::units::*};
    /// #
    /// assert_eq!(
    ///     Microseconds::<u32>::try_from_rate(Kilohertz(2_u32)),
    ///     Ok(Microseconds(500_u32))
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// Failure will only occur if the provided value does not fit in the selected destination type.
    ///
    /// ---
    ///
    /// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
    ///
    /// ```rust
    /// # use embedded_time::{duration::{Duration, units::*}, rate::units::*, ConversionError, traits::*};
    /// #
    /// assert_eq!(
    ///     Nanoseconds::<u32>::try_from_rate(u32::MAX.MHz()),
    ///     Err(ConversionError::Overflow)
    /// );
    /// ```
    ///
    /// ---
    ///
    /// [`ConversionError::ConversionFailure`] : The integer conversion to that of the destination
    /// type fails.
    ///
    /// ```rust
    /// # use embedded_time::{duration::{Duration, units::*}, rate::units::*, ConversionError, traits::*};
    /// #
    /// assert_eq!(
    ///     Seconds::<u32>::try_from_rate((u32::MAX as u64 + 1).Hz()),
    ///     Err(ConversionError::ConversionFailure)
    /// );
    /// ```
    ///
    /// ---
    ///
    /// [`ConversionError::DivByZero`] : The rate is `0`, therefore the reciprocal is undefined.
    ///
    /// ```rust
    /// # use embedded_time::{duration::{Duration, units::*}, rate::units::*, ConversionError, traits::*};
    /// #
    /// assert_eq!(
    ///     Seconds::<u32>::try_from_rate(0_u32.Hz()),
    ///     Err(ConversionError::DivByZero)
    /// );
    /// ```
    fn try_from_rate<Rate: rate::Rate>(rate: Rate) -> Result<Self, ConversionError>
    where
        Rate: FixedPoint,
        u32: TryFrom<Rate::T>,
        Self: FixedPoint,
        Self::T: TryFrom<Rate::T>,
    {
        let rate = rate.try_into_generic(Rate::SCALING_FACTOR)?;
        fixed_point::from_ticks(
            rate.scaling_factor()
                .checked_mul(&Self::SCALING_FACTOR)?
                .recip()
                .checked_div_integer(
                    u32::try_from(*rate.integer())
                        .map_err(|_| ConversionError::ConversionFailure)?,
                )?
                .to_integer(),
            Self::SCALING_FACTOR,
        )
    }

    // TODO: add try_into_rate
}

/// The `Generic` `Duration` type allows arbitrary _scaling factor_s to be used without having to
/// impl FixedPoint.
///
/// The purpose of this type is to allow a simple `Duration` that can be defined at run-time. It
/// does this by replacing the `const` _scaling factor_ with a struct field.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Generic<T> {
    integer: T,
    scaling_factor: Fraction,
}

impl<T> Generic<T> {
    /// Constructs a new (ram) fixed-point `Generic` `Duration` value
    pub const fn new(integer: T, scaling_factor: Fraction) -> Self {
        Self {
            integer,
            scaling_factor,
        }
    }

    /// Returns the _integer_ value
    pub const fn integer(&self) -> &T {
        &self.integer
    }

    /// Returns the _scaling factor_ [`Fraction`] value
    pub const fn scaling_factor(&self) -> &Fraction {
        &self.scaling_factor
    }
}

impl<T: TimeInt> Duration for Generic<T> {}

/// Duration units
pub mod units {
    use super::*;
    use crate::{
        fixed_point::{self, FixedPoint},
        fraction::Fraction,
        time_int::TimeInt,
        ConversionError,
    };
    use core::{
        cmp,
        convert::{TryFrom, TryInto},
        fmt::{self, Formatter},
        ops,
    };

    macro_rules! impl_duration {
        ( $name:ident, ($numer:expr, $denom:expr) ) => {
            /// A duration unit type
            #[derive(Copy, Clone, Debug, Eq, Ord)]
            pub struct $name<T: TimeInt = u32>(pub T);

            impl<T: TimeInt> $name<T> {
                #[doc(hidden)]
                pub fn new(value: T) -> Self {
                    Self(value)
                }
            }

            impl<T: TimeInt> Duration for $name<T> {}

            impl<T: TimeInt> FixedPoint for $name<T> {
                type T = T;
                const SCALING_FACTOR: Fraction = Fraction::new($numer, $denom);

                fn new(value: Self::T) -> Self {
                    Self(value)
                }

                fn integer(&self) -> &Self::T {
                    &self.0
                }
            }

            impl<T: TimeInt> fmt::Display for $name<T> {
                /// Just forwards the underlying integer to [`core::fmt::Display::fmt()`]
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// #
                /// assert_eq!(format!("{}", Seconds(123_u32)), "123");
                /// ```
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    fmt::Display::fmt(&self.0, f)
                }
            }

            impl<T: TimeInt, Rhs: Duration> ops::Add<Rhs> for $name<T>
            where
                Rhs: FixedPoint,
                T: TryFrom<Rhs::T>,
            {
                type Output = Self;

                /// Returns the sum as the LHS type
                ///
                /// # Examples
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// #
                /// assert_eq!((Milliseconds(1_u32) + Seconds(1_u32)),
                ///     Milliseconds(1_001_u32));
                /// ```
                ///
                /// # Panics
                ///
                /// The same reason the integer operation would panic. Namely, if the result
                /// overflows the type.
                ///
                /// ```rust,should_panic
                /// # use embedded_time::{traits::*, duration::units::*};
                /// #
                /// let _ = Seconds(u32::MAX) + Seconds(1_u32);
                /// ```
                fn add(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::add(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Duration> ops::Sub<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
            {
                type Output = Self;

                /// Returns the difference as the LHS type
                ///
                /// # Examples
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// #
                /// assert_eq!((Milliseconds(2_001_u32) - Seconds(1_u32)),
                ///     Milliseconds(1_001_u32));
                /// ```
                ///
                /// # Panics
                ///
                /// The same reason the integer operation would panic. Namely, if the result
                /// overflows the type.
                ///
                /// ```rust,should_panic
                /// # use embedded_time::{traits::*, duration::units::*};
                /// #
                /// let _ = Seconds(0_u32) - Seconds(1_u32);
                /// ```
                fn sub(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::sub(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Duration> ops::Rem<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
            {
                type Output = Self;

                /// Returns the remainder as the LHS type
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// #
                /// assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
                /// ```
                fn rem(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::rem(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Duration> cmp::PartialEq<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
                Rhs::T: TryFrom<T>,
            {
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// #
                /// assert_eq!(Seconds(123_u32), Milliseconds(123_000_u64));
                /// assert_ne!(Seconds(123_u32), Milliseconds(123_001_u64));
                /// ```
                fn eq(&self, rhs: &Rhs) -> bool {
                    <Self as FixedPoint>::eq(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Duration> PartialOrd<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
                Rhs::T: TryFrom<T>,
            {
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// #
                ///
                /// assert!(Seconds(2_u64) < Milliseconds(2_001_u32));
                /// assert!(Seconds(2_u32) > Milliseconds(1_999_u32));
                /// ```
                fn partial_cmp(&self, rhs: &Rhs) -> Option<core::cmp::Ordering> {
                    <Self as FixedPoint>::partial_cmp(self, rhs)
                }
            }

            impl<SourceInt: TimeInt, DestInt: TimeInt> TryFrom<Generic<SourceInt>>
                for $name<DestInt>
            where
                DestInt: TryFrom<SourceInt>,
            {
                type Error = ConversionError;

                /// Construct a _named_ `Duration` from a `Generic` `Duration`
                ///
                /// # Examples
                ///
                /// ```rust
                /// # use embedded_time::{Fraction, duration::units::*, duration::Generic};
                /// # use core::convert::{TryFrom, TryInto};
                /// #
                /// assert_eq!(
                ///     Seconds::<u64>::try_from(Generic::new(2_000_u32, Fraction::new(1, 1_000))),
                ///     Ok(Seconds(2_u64))
                /// );
                ///
                /// // TryInto also works
                /// assert_eq!(
                ///     Generic::new(2_000_u64, Fraction::new(1, 1_000)).try_into(),
                ///     Ok(Seconds(2_u64))
                /// );
                /// ```
                ///
                /// # Errors
                ///
                /// Failure will only occur if the provided value does not fit in the selected
                /// destination type.
                ///
                /// ---
                ///
                /// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an
                /// overflow.
                ///
                /// ```rust
                /// # use embedded_time::{Fraction, duration::units::*, duration::Generic, ConversionError};
                /// # use core::convert::TryFrom;
                /// #
                /// assert_eq!(
                ///     Seconds::<u32>::try_from(Generic::new(u32::MAX, Fraction::new(10,1))),
                ///     Err(ConversionError::Overflow)
                /// );
                /// ```
                ///
                /// ---
                ///
                /// [`ConversionError::ConversionFailure`] : The _integer_ conversion to that of the
                /// destination type fails.
                ///
                /// ```rust
                /// # use embedded_time::{Fraction, duration::units::*, duration::Generic, ConversionError};
                /// # use core::convert::TryFrom;
                /// #
                /// assert_eq!(
                ///     Seconds::<u32>::try_from(Generic::new(u32::MAX as u64 + 1, Fraction::new(1,1))),
                ///     Err(ConversionError::ConversionFailure)
                /// );
                /// ```
                fn try_from(generic_duration: Generic<SourceInt>) -> Result<Self, Self::Error> {
                    fixed_point::from_ticks(
                        generic_duration.integer,
                        generic_duration.scaling_factor,
                    )
                }
            }
        };

        ( $name:ident, ($numer:expr, $denom:expr), ge_secs ) => {
            impl_duration![$name, ($numer, $denom)];

            impl<T: TimeInt> TryFrom<$name<T>> for core::time::Duration {
                type Error = ConversionError;

                /// Construct a [`core::time::Duration`] from an embedded_time::[`Duration`]
                ///
                /// ```rust
                /// # use embedded_time::traits::*;
                /// # use core::convert::TryFrom;
                /// #
                /// let core_duration = core::time::Duration::try_from(2_569_u32.milliseconds()).unwrap();
                /// assert_eq!(core_duration.as_secs(), 2);
                /// assert_eq!(core_duration.subsec_nanos(), 569_000_000);
                /// ```
                ///
                /// ```rust
                /// # use embedded_time::traits::*;
                /// # use core::convert::TryInto;
                /// #
                /// let core_duration: core::time::Duration = 2_569_u32.milliseconds().try_into().unwrap();
                /// assert_eq!(core_duration.as_secs(), 2);
                /// assert_eq!(core_duration.subsec_nanos(), 569_000_000);
                /// ```
                fn try_from(duration: $name<T>) -> Result<Self, Self::Error> {
                    let seconds = Seconds::<u64>::try_convert_from(duration)?;
                    Ok(Self::from_secs(*seconds.integer()))
                }
            }

            impl<T: TimeInt> TryFrom<core::time::Duration> for $name<T> {
                type Error = ConversionError;

                /// Construct an embedded_time::[`Duration`] from a [`core::time::Duration`]
                ///
                /// # Examples
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// # use core::convert::TryFrom;
                /// #
                /// let core_duration = core::time::Duration::new(5, 730023852);
                /// assert_eq!(Milliseconds::<u32>::try_from(core_duration), Ok(5_730.milliseconds()));
                /// ```
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// # use core::convert::TryInto;
                /// #
                /// let duration: Result<Milliseconds<u32>, _> =
                ///     core::time::Duration::new(5, 730023852).try_into();
                /// assert_eq!(duration, Ok(5_730.milliseconds()));
                /// ```
                ///
                /// # Errors
                ///
                /// [`ConversionError::ConversionFailure`] : The result doesn't fit in the specified
                /// type
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*, ConversionError};
                /// # use core::convert::{TryFrom, TryInto};
                /// #
                /// assert_eq!(
                ///     Milliseconds::<u32>::try_from(core::time::Duration::from_millis((u32::MAX as u64) + 1)),
                ///     Err(ConversionError::ConversionFailure)
                /// );
                ///
                /// let duration: Result<Milliseconds<u32>, _> =
                ///     core::time::Duration::from_millis((u32::MAX as u64) + 1).try_into();
                /// assert_eq!(duration, Err(ConversionError::ConversionFailure));
                /// ```
                fn try_from(core_duration: core::time::Duration) -> Result<Self, Self::Error> {
                    let seconds = Seconds(core_duration.as_secs());
                    Self::try_convert_from(seconds)
                }
            }
        };
        ( $name:ident, ($numer:expr, $denom:expr), $from_core_dur:ident, $as_core_dur:ident ) => {
            impl_duration![$name, ($numer, $denom)];

            impl<T: TimeInt> TryFrom<$name<T>> for core::time::Duration {
                type Error = ConversionError;

                /// Construct a [`core::time::Duration`] from an embedded_time::[`Duration`]
                ///
                /// ```rust
                /// # use embedded_time::traits::*;
                /// # use core::convert::TryFrom;
                /// #
                /// let core_duration = core::time::Duration::try_from(2_569_u32.milliseconds()).unwrap();
                /// assert_eq!(core_duration.as_secs(), 2);
                /// assert_eq!(core_duration.subsec_nanos(), 569_000_000);
                /// ```
                ///
                /// ```rust
                /// # use embedded_time::traits::*;
                /// # use core::convert::TryInto;
                /// #
                /// let core_duration: core::time::Duration = 2_569_u32.milliseconds().try_into().unwrap();
                /// assert_eq!(core_duration.as_secs(), 2);
                /// assert_eq!(core_duration.subsec_nanos(), 569_000_000);
                /// ```
                fn try_from(duration: $name<T>) -> Result<Self, Self::Error> {
                    Ok(Self::$from_core_dur((*duration.integer()).into()))
                }
            }

            impl<T: TimeInt> TryFrom<core::time::Duration> for $name<T> {
                type Error = ConversionError;

                /// Construct an embedded_time::[`Duration`] from a [`core::time::Duration`]
                ///
                /// # Examples
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// # use core::convert::TryFrom;
                /// #
                /// let core_duration = core::time::Duration::new(5, 730023852);
                /// assert_eq!(Milliseconds::<u32>::try_from(core_duration), Ok(5_730.milliseconds()));
                /// ```
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*};
                /// # use core::convert::TryInto;
                /// #
                /// let duration: Result<Milliseconds<u32>, _> =
                ///     core::time::Duration::new(5, 730023852).try_into();
                /// assert_eq!(duration, Ok(5_730.milliseconds()));
                /// ```
                ///
                /// # Errors
                ///
                /// [`ConversionError::ConversionFailure`] : The result doesn't fit in the specified
                /// type
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, duration::units::*, ConversionError};
                /// # use core::convert::{TryFrom, TryInto};
                /// #
                /// assert_eq!(
                ///     Milliseconds::<u32>::try_from(core::time::Duration::from_millis((u32::MAX as u64) + 1)),
                ///     Err(ConversionError::ConversionFailure)
                /// );
                ///
                /// let duration: Result<Milliseconds<u32>, _> =
                ///     core::time::Duration::from_millis((u32::MAX as u64) + 1).try_into();
                /// assert_eq!(duration, Err(ConversionError::ConversionFailure));
                /// ```
                fn try_from(core_duration: core::time::Duration) -> Result<Self, Self::Error> {
                    Ok(Self(
                        core_duration
                            .$as_core_dur()
                            .try_into()
                            .map_err(|_| ConversionError::ConversionFailure)?,
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

    /// Create time-based extensions from primitive numeric types.
    ///
    /// This trait is anonomously re-exported in [`traits`](crate::traits) module
    ///
    /// # Examples
    ///
    /// Basic construction of time-based values.
    ///
    /// ```rust
    /// # use embedded_time::{traits::*, duration::units::*};
    /// assert_eq!(5_u32.nanoseconds(), Nanoseconds(5_u32));
    /// assert_eq!(5_u32.microseconds(), Microseconds(5_u32));
    /// assert_eq!(5_u32.milliseconds(), Milliseconds(5_u32));
    /// assert_eq!(5_u32.seconds(), Seconds(5_u32));
    /// assert_eq!(5_u32.minutes(), Minutes(5_u32));
    /// assert_eq!(5_u32.hours(), Hours(5_u32));
    /// ```
    pub trait Extensions: TimeInt {
        /// nanoseconds
        fn nanoseconds(self) -> Nanoseconds<Self> {
            Nanoseconds::new(self)
        }
        /// microseconds
        fn microseconds(self) -> Microseconds<Self> {
            Microseconds::new(self)
        }
        /// milliseconds
        fn milliseconds(self) -> Milliseconds<Self> {
            Milliseconds::new(self)
        }
        /// seconds
        fn seconds(self) -> Seconds<Self> {
            Seconds::new(self)
        }
        /// minutes
        fn minutes(self) -> Minutes<Self> {
            Minutes::new(self)
        }
        /// hours
        fn hours(self) -> Hours<Self> {
            Hours::new(self)
        }
    }

    impl Extensions for u32 {}
    impl Extensions for u64 {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration::units::*;
    use crate::rate::units::*;
    use core::convert::TryInto;

    #[test]
    fn try_from_generic_ok() {
        assert_eq!(
            Seconds::try_from(Generic::new(246_u32, Fraction::new(1, 2))),
            Ok(Seconds(123_u32))
        );
    }

    #[test]
    fn try_into_generic_ok() {
        assert_eq!(
            Seconds(123_u32).try_into_generic(Fraction::new(1, 2)),
            Ok(Generic::new(246_u32, Fraction::new(1, 2)))
        );
    }

    #[test]
    fn try_into_generic_err() {
        assert_eq!(
            Seconds(u32::MAX).try_into_generic::<u32>(Fraction::new(1, 2)),
            Err(ConversionError::Overflow)
        );
    }

    #[test]
    fn get_generic_integer() {
        let generic = Generic::new(246_u32, Fraction::new(1, 2));
        assert_eq!(generic.integer(), &246_u32);
    }

    #[test]
    fn check_for_overflows() {
        let mut time = 1_u64;
        time *= 60;
        assert_eq!(Minutes(time), Hours(1_u32));
        time *= 60;
        assert_eq!(Seconds(time), Hours(1_u32));
        time *= 1000;
        assert_eq!(Milliseconds(time), Hours(1_u32));
        time *= 1000;
        assert_eq!(Microseconds(time), Hours(1_u32));
        time *= 1000;
        assert_eq!(Nanoseconds(time), Hours(1_u32));
    }

    #[test]
    fn remainder() {
        assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
        assert_eq!(Minutes(62_u32) % Milliseconds(1_u32), Minutes(0_u32));
        assert_eq!(Minutes(62_u32) % Minutes(60_u32), Minutes(2_u32));
    }

    #[test]
    fn convert_from_rate() {
        assert_eq!(
            Milliseconds::<u32>::try_from_rate(Hertz(2_u32)),
            Ok(Milliseconds(500_u32))
        );

        assert_eq!(
            Microseconds::<u32>::try_from_rate(Kilohertz(2_u32)),
            Ok(Microseconds(500_u32))
        );
    }

    #[test]
    fn convert_from_core_duration() {
        let core_duration = core::time::Duration::from_nanos(5_025_678_901_234);
        assert_eq!(
            core_duration.try_into(),
            Ok(Nanoseconds::<u64>(5_025_678_901_234))
        );
        assert_eq!(
            core_duration.try_into(),
            Ok(Microseconds::<u64>(5_025_678_901))
        );
        assert_eq!(core_duration.try_into(), Ok(Milliseconds::<u32>(5_025_678)));
        assert_eq!(core_duration.try_into(), Ok(Seconds::<u32>(5_025)));
        assert_eq!(core_duration.try_into(), Ok(Minutes::<u32>(83)));
        assert_eq!(core_duration.try_into(), Ok(Hours::<u32>(1)));
    }

    #[test]
    fn convert_to_core_duration() {
        assert_eq!(
            Nanoseconds(123_u32).try_into(),
            Ok(core::time::Duration::from_nanos(123))
        );
        assert_eq!(
            Microseconds(123_u32).try_into(),
            Ok(core::time::Duration::from_micros(123))
        );
        assert_eq!(
            Milliseconds(123_u32).try_into(),
            Ok(core::time::Duration::from_millis(123))
        );
        assert_eq!(
            Seconds(123_u32).try_into(),
            Ok(core::time::Duration::from_secs(123))
        );
        assert_eq!(
            Minutes(123_u32).try_into(),
            Ok(core::time::Duration::from_secs(123 * 60))
        );
        assert_eq!(
            Hours(123_u32).try_into(),
            Ok(core::time::Duration::from_secs(123 * 3600))
        );
    }

    #[test]
    fn duration_scaling() {
        assert_eq!(1_u32.nanoseconds(), 1_u32.nanoseconds());
        assert_eq!(1_u32.microseconds(), 1_000_u32.nanoseconds());
        assert_eq!(1_u32.milliseconds(), 1_000_000_u32.nanoseconds());
        assert_eq!(1_u32.seconds(), 1_000_000_000_u32.nanoseconds());
        assert_eq!(1_u32.minutes(), 60_000_000_000_u64.nanoseconds());
        assert_eq!(1_u32.hours(), 3_600_000_000_000_u64.nanoseconds());
    }
}
