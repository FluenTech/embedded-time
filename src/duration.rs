//! Duration types/units.

use crate::fixed_point::FixedPoint;

/// An unsigned duration of time
///
/// Each implementation defines a constant `Fraction` which represents the
/// period of the count's LSbit
///
///
/// # Constructing a duration
///
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!(Milliseconds::<u32>::new(23), Milliseconds(23_u32));
/// assert_eq!(23_u32.milliseconds(), Milliseconds(23_u32));
/// ```
///
/// # Get the integer count
///
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!(Milliseconds(23_u32).count(), 23_u32);
/// ```
///
/// # Formatting
///
/// Just forwards the underlying integer to [`core::fmt::Display::fmt()`]
///
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!(format!("{}", Seconds(123_u32)), "123");
/// ```
///
/// # Getting H:M:S.MS... Components
///
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
///
/// [`core::time::Duration`]
///
/// ## Examples
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
///
/// # Converting from `core` types
///
/// [`core::time::Duration`]
///
/// ## Examples
///
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// # use core::convert::TryFrom;
/// #
/// let core_duration = core::time::Duration::new(5, 730023852);
/// assert_eq!(Milliseconds::<u32>::try_from(core_duration), Ok(5_730.milliseconds()));
/// ```
///
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// # use core::convert::TryInto;
/// #
/// let duration: Result<Milliseconds<u32>, _> = core::time::Duration::new(5, 730023852).try_into();
/// assert_eq!(duration, Ok(5_730.milliseconds()));
/// ```
///
/// ## Errors
///
/// `ConversionError::ConversionFailure` : The duration doesn't fit in the type specified
///
/// ```rust
/// # use embedded_time::{traits::*, units::*, ConversionError};
/// # use core::convert::{TryFrom, TryInto};
/// #
/// assert_eq!(
///     Milliseconds::<u32>::try_from(
///     core::time::Duration::from_millis((u32::MAX as u64) + 1)), Err(ConversionError::ConversionFailure));
///
/// let duration: Result<Milliseconds<u32>, _> =
///     core::time::Duration::from_millis((u32::MAX as u64) + 1).try_into();
/// assert_eq!(duration, Err(ConversionError::ConversionFailure));
/// ```
///
/// # Add/Sub
///
/// The result of the operation is the LHS type
///
/// ## Examples
///
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!((Milliseconds(2_001_u32) - Seconds(1_u32)),
///     Milliseconds(1_001_u32));
///
/// assert_eq!((Milliseconds(1_u32) + Seconds(1_u32)),
///     Milliseconds(1_001_u32));
/// ```
///
/// ## Panics
///
/// The same reason the integer operation would panic. Namely, if the
/// result overflows the type.
///
/// ### Examples
///
/// ```rust,should_panic
/// # use embedded_time::{traits::*, units::*};
/// #
/// let _ = Seconds(u32::MAX) + Seconds(1_u32);
/// ```
///
/// # Equality
///
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
///
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
///
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// #
/// assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
/// ```
pub trait Duration: FixedPoint {}

#[doc(hidden)]
pub mod units {
    use crate::{
        duration::Duration, fixed_point::FixedPoint, fraction::Fraction, time_int::TimeInt,
        ConversionError,
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

            impl<Rep: TimeInt> Duration for $name<Rep> {}

            impl<Rep: TimeInt> FixedPoint for $name<Rep> {
                type Rep = Rep;
                const SCALING_FACTOR: Fraction = Fraction::new($numer, $denom);

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
                    let rhs = Self::try_convert_from(rhs).unwrap().count();

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
                    if Self::SCALING_FACTOR < OtherDur::SCALING_FACTOR {
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
                    if Self::SCALING_FACTOR < OtherDur::SCALING_FACTOR {
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
                type Error = ConversionError;

                /// Convert an embedded_time::[`Duration`] into a [`core::time::Duration`]
                fn try_from(duration: $name<Rep>) -> Result<Self, Self::Error> {
                    let seconds = Seconds::<u64>::try_convert_from(duration)?;
                    Ok(Self::from_secs(seconds.count()))
                }
            }

            impl<Rep: TimeInt> convert::TryFrom<core::time::Duration> for $name<Rep> {
                type Error = ConversionError;

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
                type Error = ConversionError;

                /// Convert an embedded_time::[`Duration`] into a [`core::time::Duration`]
                fn try_from(duration: $name<Rep>) -> Result<Self, Self::Error> {
                    Ok(Self::$from_core_dur(duration.count().into()))
                }
            }

            impl<Rep: TimeInt> convert::TryFrom<core::time::Duration> for $name<Rep> {
                type Error = ConversionError;

                /// Convert a [`core::time::Duration`] into an embedded_time::[`Duration`]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryInto;
    use units::*;

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
}
