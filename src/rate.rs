//! Representations of frequency-based values

//! Rate-based types/units

use crate::{
    duration,
    fixed_point::{self, FixedPoint},
    time_int::TimeInt,
    ConversionError, Fraction,
};
use core::{convert::TryFrom, prelude::v1::*};

/// An unsigned, fixed-point rate type
///
/// Each implementation defines an integer type and a [`Fraction`] scaling factor.
///
/// # Constructing a rate
///
/// ```rust
/// # use embedded_time::{traits::*, rate::units::*};
/// #
/// assert_eq!(45_u32.Hz(), Hertz(45_u32));
/// ```
///
/// ## From a [`Generic`] `Rate`
///
/// ### Errors
///
/// Failure will only occur if the provided integer does not fit in the
/// selected destination type.
///
/// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
///
/// #### Examples
///
/// ```rust
/// # use embedded_time::{Fraction, rate::units::*, rate::Generic, ConversionError};
/// # use core::convert::TryFrom;
/// #
/// assert_eq!(Hertz::<u32>::try_from(Generic::new(u32::MAX, Fraction::new(10,1))),
///     Err(ConversionError::Overflow));
/// ```
///
/// [`ConversionError::ConversionFailure`] : The integer cast to that of the destination
/// type fails.
///
/// #### Examples
///
/// ```rust
/// # use embedded_time::{Fraction, rate::units::*, rate::Generic, ConversionError};
/// # use core::convert::TryFrom;
/// #
/// assert_eq!(Hertz::<u32>::try_from(Generic::new(u32::MAX as u64 + 1, Fraction::new(1,1))),
///     Err(ConversionError::ConversionFailure));
/// ```
///
/// ### Examples
///
/// ```rust
/// # use embedded_time::{Fraction, rate::units::*, rate::Generic};
/// # use core::convert::{TryFrom, TryInto};
/// #
/// assert_eq!(Hertz::<u64>::try_from(Generic::new(2_000_u32, Fraction::new(1,1_000))),
///     Ok(Hertz(2_u64)));
///
/// assert_eq!(Generic::new(2_000_u64, Fraction::new(1,1_000)).try_into(),
///     Ok(Hertz(2_u64)));
/// ```
///
/// # Read the integer part
///
/// ```rust
/// # use embedded_time::{traits::*, rate::units::*};
/// #
/// assert_eq!(Hertz(45_u32).integer(), &45_u32);
/// ```
///
/// # Formatting
///
/// Just forwards the underlying integer to [`core::fmt::Display::fmt()`]
///
/// ```rust
/// # use embedded_time::{traits::*, rate::units::*};
/// #
/// assert_eq!(format!("{}", Hertz(123_u32)), "123");
/// ```
///
/// # Add/Sub
///
/// The result of the operation is the LHS type
///
/// ## Examples
///
/// ```rust
/// # use embedded_time::{traits::*, rate::units::*};
/// #
/// assert_eq!((Hertz(2u32) - Hertz(1_u32)),
///     Hertz(1_u32));
///
/// assert_eq!((Hertz(1_u32) + Hertz(1_u32)),
///     Hertz(2_u32));
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
/// # use embedded_time::{traits::*, rate::units::*};
/// #
/// let _ = Hertz(u32::MAX) + Hertz(1_u32);
/// ```
///
/// # Equality
///
/// ```rust
/// # use embedded_time::{traits::*, rate::units::*};
/// #
/// assert_eq!(Hertz(123_u32), Hertz(123_u32));
/// ```
///
/// # Comparisons
///
/// ```rust
/// # use embedded_time::{traits::*, rate::units::*};
/// #
/// assert!(Hertz(2_u32) < Hertz(3_u32));
/// assert_eq!(Hertz(2_u32), Hertz(2_u32));
/// assert!(Hertz(2_u32) > Hertz(1_u32));
/// ```
///
/// # Remainder
///
/// ```rust
/// # use embedded_time::{traits::*, rate::units::*};
/// #
/// assert_eq!(Hertz(2_037_u32) % Kilohertz(1_u32), Hertz(37_u32));
/// ```
pub trait Rate: Copy {
    /// Construct a `Generic` `Rate` from an _named_ `Rate`
    ///
    /// # Errors
    ///
    /// Failure will only occur if the provided integer does not fit in the selected destination
    /// type.
    ///
    /// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, rate::units::*, rate::{Rate, Generic}, ConversionError};
    /// # use core::convert::TryFrom;
    /// #
    /// assert_eq!(Hertz(u32::MAX).try_into_generic::<u32>(Fraction::new(1, 2)),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// [`ConversionError::ConversionFailure`] : The integer cast to that of the destination
    /// type fails.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, rate::units::*, rate::{Rate, Generic}, ConversionError};
    /// # use core::convert::TryFrom;
    /// #
    /// assert_eq!(Hertz(u32::MAX as u64 + 1).try_into_generic::<u32>(Fraction::new(1, 1)),
    ///     Err(ConversionError::ConversionFailure));
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, rate::units::*, rate::{Generic, Rate}};
    /// # use core::convert::{TryFrom, TryInto};
    /// #
    /// assert_eq!(Hertz(2_u64).try_into_generic(Fraction::new(1,2_000)),
    ///     Ok(Generic::new(4_000_u32, Fraction::new(1,2_000))));
    /// ```
    fn try_into_generic<DestInt: TimeInt>(
        self,
        scaling_factor: Fraction,
    ) -> Result<Generic<DestInt>, ConversionError>
    where
        Self: FixedPoint,
        DestInt: TryFrom<Self::Rep>,
    {
        Ok(Generic::<DestInt>::new(
            self.into_ticks(scaling_factor)?,
            scaling_factor,
        ))
    }

    /// Attempt to construct the given _rate_ type from the given _duration_ type
    ///
    /// (the rate is equal to the reciprocal of the duration)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{duration::units::*, rate::{Rate, units::*}};
    /// #
    /// assert_eq!(
    ///     Kilohertz::<u32>::try_from_duration(Microseconds(2_u32)),
    ///     Ok(Kilohertz(500_u32))
    /// );
    /// ```
    fn try_from_duration<Duration: duration::Duration>(
        duration: Duration,
    ) -> Result<Self, ConversionError>
    where
        Duration: FixedPoint,
        u32: TryFrom<Duration::Rep>,
        Self: FixedPoint,
        Self::Rep: TryFrom<Duration::Rep>,
    {
        let duration = duration.try_into_generic(Duration::SCALING_FACTOR)?;
        fixed_point::from_ticks(
            duration
                .scaling_factor()
                .checked_mul(&Self::SCALING_FACTOR)?
                .recip()
                .checked_div_integer(
                    u32::try_from(*duration.integer())
                        .map_err(|_| ConversionError::ConversionFailure)?,
                )?
                .to_integer(),
            Self::SCALING_FACTOR,
        )
    }

    // TODO: add try_into_duration
}

/// The `Generic` `Rate` type allows arbitrary scaling factors to be used without having to impl
/// FixedPoint.
///
/// The purpose of this type is to allow a simple `Rate` that can be defined at run-time. It does
/// this by replacing the `const` _scaling factor_ with a field.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Generic<T> {
    integer: T,
    scaling_factor: Fraction,
}

impl<T> Generic<T> {
    /// Constructs a new (ram) fixed-point `Generic` `Rate` value
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

impl<T: TimeInt> Rate for Generic<T> {}

/// Rate-type units
pub mod units {
    use super::*;
    use crate::{
        fixed_point::{self, FixedPoint},
        fraction::Fraction,
        time_int::TimeInt,
        ConversionError,
    };
    use core::{
        convert::TryFrom,
        fmt::{self, Formatter},
        ops,
    };

    macro_rules! impl_rate {
        ( $name:ident, ($numer:expr, $denom:expr) ) => {
            /// A duration unit type
            #[derive(Copy, Clone, Debug, Eq, Ord)]
            pub struct $name<T: TimeInt = u32>(pub T);

            impl<Rep: TimeInt> Rate for $name<Rep> {}

            impl<Rep: TimeInt> FixedPoint for $name<Rep> {
                type Rep = Rep;
                const SCALING_FACTOR: Fraction = Fraction::new($numer, $denom);

                fn new(value: Self::Rep) -> Self {
                    Self(value)
                }

                fn integer(&self) -> &Self::Rep {
                    &self.0
                }
            }

            impl<T: TimeInt> fmt::Display for $name<T> {
                /// See module-level documentation for details about this type
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    fmt::Display::fmt(&self.0, f)
                }
            }

            impl<Rep: TimeInt, Rhs: Rate> ops::Add<Rhs> for $name<Rep>
            where
                Rep: TryFrom<Rhs::Rep>,
                Rhs: FixedPoint,
            {
                type Output = Self;

                /// See module-level documentation for details about this type
                fn add(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::add(self, rhs)
                }
            }

            impl<Rep: TimeInt, Rhs: Rate> ops::Sub<Rhs> for $name<Rep>
            where
                Rep: TryFrom<Rhs::Rep>,
                Rhs: FixedPoint,
            {
                type Output = Self;

                /// See module-level documentation for details about this type
                fn sub(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::sub(self, rhs)
                }
            }

            impl<Rep: TimeInt, Rhs: Rate> ops::Rem<Rhs> for $name<Rep>
            where
                Rep: TryFrom<Rhs::Rep>,
                Rhs: FixedPoint,
            {
                type Output = Self;

                fn rem(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::rem(self, rhs)
                }
            }

            impl<Rep: TimeInt, Rhs: Rate> PartialEq<Rhs> for $name<Rep>
            where
                Rep: TryFrom<Rhs::Rep>,
                Rhs: FixedPoint,
                Rhs::Rep: TryFrom<Rep>,
            {
                /// See module-level documentation for details about this type
                fn eq(&self, rhs: &Rhs) -> bool {
                    <Self as FixedPoint>::eq(self, rhs)
                }
            }

            impl<Rep: TimeInt, Rhs: Rate> PartialOrd<Rhs> for $name<Rep>
            where
                Rep: TryFrom<Rhs::Rep>,
                Rhs: FixedPoint,
                Rhs::Rep: TryFrom<Rep>,
            {
                /// See module-level documentation for details about this type
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

                fn try_from(generic_rate: Generic<SourceInt>) -> Result<Self, Self::Error> {
                    fixed_point::from_ticks(generic_rate.integer, generic_rate.scaling_factor)
                }
            }

            impl<T: TimeInt> From<$name<T>> for Generic<T> {
                fn from(rate: $name<T>) -> Self {
                    Self::new(*rate.integer(), $name::<T>::SCALING_FACTOR)
                }
            }
        };
    }
    impl_rate![Megahertz, (1_000_000, 1)];
    impl_rate![Kilohertz, (1_000, 1)];
    impl_rate![Hertz, (1, 1)];

    impl_rate![MebibytePerSecond, (1_048_576, 1)];
    impl_rate![MegabytePerSecond, (1_000_000, 1)];
    impl_rate![KibibytePerSecond, (1_024, 1)];
    impl_rate![KiloBytePerSecond, (1_000, 1)];
    impl_rate![BytePerSecond, (1, 1)];

    impl_rate![MebibitPerSecond, (1_048_576, 1)];
    impl_rate![MegabitPerSecond, (1_000_000, 1)];
    impl_rate![KibibitPerSecond, (1_024, 1)];
    impl_rate![KilobitPerSecond, (1_000, 1)];
    impl_rate![BitPerSecond, (1, 1)];

    impl_rate![Megabaud, (1, 1)];
    impl_rate![Kilobaud, (1, 1)];
    impl_rate![Baud, (1, 1)];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{duration::units::*, rate::units::*};

    #[test]
    fn try_from_generic_ok() {
        assert_eq!(
            Hertz::try_from(Generic::new(246_u32, Fraction::new(1, 2))),
            Ok(Hertz(123_u32))
        );
    }

    #[test]
    fn try_into_generic_ok() {
        assert_eq!(
            Hertz(123_u32).try_into_generic(Fraction::new(1, 2)),
            Ok(Generic::new(246_u32, Fraction::new(1, 2)))
        );
    }

    #[test]
    fn try_into_generic_err() {
        assert_eq!(
            Hertz(u32::MAX).try_into_generic::<u32>(Fraction::new(1, 2)),
            Err(ConversionError::Overflow)
        );
    }

    #[test]
    fn get_generic_count() {
        let generic = Generic::new(246_u32, Fraction::new(1, 2));
        assert_eq!(generic.integer(), &246_u32);
    }

    #[test]
    fn check_for_overflows() {
        let mut time = 1_u32;
        time *= 60;
        assert_eq!(Hertz(time), Hertz(60_u32));
    }

    #[test]
    fn remainder() {
        assert_eq!(Hertz(62_u32) % Hertz(60_u32), Hertz(2_u32));
        assert_eq!(Hertz(2_003_u32) % Kilohertz(1_u32), Hertz(3_u32));
        assert_eq!(Kilohertz(40_u32) % Hertz(100_u32), Kilohertz(0_u32));
    }

    #[test]
    fn convert_from_duration() {
        assert_eq!(
            Hertz::<u32>::try_from_duration(Milliseconds(2_u32)),
            Ok(Hertz(500_u32))
        );

        assert_eq!(
            Kilohertz::<u32>::try_from_duration(Microseconds(2_u32)),
            Ok(Kilohertz(500_u32))
        );
    }
}
