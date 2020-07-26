//! Rate-based types/units

use crate::{
    duration,
    fixed_point::{self, FixedPoint},
    time_int::TimeInt,
    ConversionError, Fraction,
};
use core::{convert::TryFrom, mem::size_of, prelude::v1::*};
use num::{CheckedDiv, CheckedMul};

/// An unsigned, fixed-point rate type
///
/// Each implementation defines an _integer_ type and a [`Fraction`] _scaling factor_.
///
/// # Constructing a rate
///
/// ```rust
/// # use embedded_time::{traits::*, rate::units::*};
/// #
/// assert_eq!(45_u32.Hz(), Hertz(45_u32));
/// ```
pub trait Rate: Copy {
    /// Construct a `Generic` `Rate` from an _named_ `Rate`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, rate::units::*, rate::{Generic, Rate}};
    /// # use core::convert::{TryFrom, TryInto};
    /// #
    /// assert_eq!(Hertz(2_u64).to_generic(Fraction::new(1,2_000)),
    ///     Ok(Generic::new(4_000_u32, Fraction::new(1,2_000))));
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
    /// # use embedded_time::{Fraction, rate::units::*, rate::{Rate, Generic}, ConversionError};
    /// # use core::convert::TryFrom;
    /// #
    /// assert_eq!(Hertz(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// ---
    ///
    /// [`ConversionError::ConversionFailure`] : The integer conversion to that of the destination
    /// type fails.
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, rate::units::*, rate::{Rate, Generic}, ConversionError};
    /// # use core::convert::TryFrom;
    /// #
    /// assert_eq!(Hertz(u32::MAX as u64 + 1).to_generic::<u32>(Fraction::new(1, 1)),
    ///     Err(ConversionError::ConversionFailure));
    /// ```
    fn to_generic<DestInt: TimeInt>(
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

    /// Convert to _named_ [`Duration`](duration::Duration)
    ///
    /// (the rate is equal to the reciprocal of the duration)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{duration::units::*, rate::{Rate, units::*}};
    /// #
    /// assert_eq!(
    ///     Kilohertz(500_u32).to_duration(),
    ///     Ok(Microseconds(2_u32))
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
    /// # use embedded_time::{duration::units::*, rate::units::*, ConversionError, traits::*};
    /// #
    /// assert_eq!(
    ///     Megahertz(u32::MAX).to_duration::<Hours<u32>>(),
    ///     Err(ConversionError::Overflow)
    /// );
    /// ```
    ///
    /// ---
    ///
    /// [`ConversionError::DivByZero`] : The rate is `0`, therefore the reciprocal is undefined.
    ///
    /// ```rust
    /// # use embedded_time::{duration::units::*, rate::units::*, ConversionError, traits::*};
    /// #
    /// assert_eq!(
    ///     Hertz(0_u32).to_duration::<Seconds<u32>>(),
    ///     Err(ConversionError::DivByZero)
    /// );
    /// ```
    fn to_duration<Duration: duration::Duration>(&self) -> Result<Duration, ConversionError>
    where
        Duration: FixedPoint,
        Self: FixedPoint,
        Duration::T: TryFrom<Self::T>,
    {
        let conversion_factor = Self::SCALING_FACTOR
            .checked_mul(&Duration::SCALING_FACTOR)?
            .recip();

        if size_of::<Self::T>() >= size_of::<Duration::T>() {
            fixed_point::from_ticks(
                Self::T::from(*conversion_factor.numerator())
                    .checked_div(
                        &self
                            .integer()
                            .checked_mul(&Self::T::from(*conversion_factor.denominator()))
                            .ok_or(ConversionError::Overflow)?,
                    )
                    .ok_or(ConversionError::DivByZero)?,
                Duration::SCALING_FACTOR,
            )
        } else {
            fixed_point::from_ticks(
                Duration::T::from(*conversion_factor.numerator())
                    .checked_div(
                        &Duration::T::try_from(*self.integer())
                            .ok()
                            .unwrap()
                            .checked_mul(&Duration::T::from(*conversion_factor.denominator()))
                            .ok_or(ConversionError::Overflow)?,
                    )
                    .ok_or(ConversionError::DivByZero)?,
                Duration::SCALING_FACTOR,
            )
        }
    }
}

/// The `Generic` `Rate` type allows arbitrary _scaling factor_s to be used without having to impl
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
        ( $name:ident, ($numer:expr, $denom:expr), $desc:literal ) => {
            #[doc = $desc]
            #[derive(Copy, Clone, Debug, Eq, Ord)]
            pub struct $name<T: TimeInt = u32>(pub T);

            impl<T: TimeInt> $name<T> {
                #[doc(hidden)]
                pub fn new(value: T) -> Self {
                    Self(value)
                }
            }

            impl<T: TimeInt> Rate for $name<T> {}

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
                /// # use embedded_time::{traits::*, rate::units::*};
                /// #
                /// assert_eq!(format!("{}", Hertz(123_u32)), "123");
                /// ```
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    fmt::Display::fmt(&self.0, f)
                }
            }

            impl<T: TimeInt, Rhs: Rate> ops::Add<Rhs> for $name<T>
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
                /// # use embedded_time::{traits::*, rate::units::*};
                /// #
                /// assert_eq!((Hertz(1_u32) + Hertz(1_u32)), Hertz(2_u32));
                /// ```
                ///
                /// # Panics
                ///
                /// The same reason the integer operation would panic. Namely, if the
                /// result overflows the type.
                ///
                /// ```rust,should_panic
                /// # use embedded_time::{traits::*, rate::units::*};
                /// #
                /// let _ = Hertz(u32::MAX) + Hertz(1_u32);
                /// ```
                fn add(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::add(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Rate> ops::Sub<Rhs> for $name<T>
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
                /// # use embedded_time::{traits::*, rate::units::*};
                /// #
                /// assert_eq!((Hertz(2u32) - Hertz(1_u32)),
                ///     Hertz(1_u32));
                /// ```
                ///
                /// # Panics
                ///
                /// The same reason the integer operation would panic. Namely, if the
                /// result overflows the type.
                ///
                /// ```rust,should_panic
                /// # use embedded_time::{traits::*, rate::units::*};
                /// #
                /// let _ = Hertz(0_u32) - Hertz(1_u32);
                /// ```
                fn sub(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::sub(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Rate> ops::Rem<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
            {
                type Output = Self;

                /// Returns the remainder as the LHS type
                ///
                /// ```rust
                /// # use embedded_time::{traits::*, rate::units::*};
                /// #
                /// assert_eq!(Hertz(2_037_u32) % Kilohertz(1_u32), Hertz(37_u32));
                /// ```
                fn rem(self, rhs: Rhs) -> Self::Output {
                    <Self as FixedPoint>::rem(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Rate> PartialEq<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
                Rhs::T: TryFrom<T>,
            {
                /// ```rust
                /// # use embedded_time::{traits::*, rate::units::*};
                /// #
                /// assert_eq!(Hertz(123_000_u32), Kilohertz(123_u32));
                /// assert_ne!(Hertz(123_001_u32), Kilohertz(123_u32));
                /// ```
                fn eq(&self, rhs: &Rhs) -> bool {
                    <Self as FixedPoint>::eq(self, rhs)
                }
            }

            impl<T: TimeInt, Rhs: Rate> PartialOrd<Rhs> for $name<T>
            where
                T: TryFrom<Rhs::T>,
                Rhs: FixedPoint,
                Rhs::T: TryFrom<T>,
            {
                /// ```rust
                /// # use embedded_time::{traits::*, rate::units::*};
                /// #
                /// assert!(Hertz(2_001_u32) > Kilohertz(2_u32));
                /// assert!(Hertz(1_999_u32) < Kilohertz(2_u32));
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

                /// Construct a _named_ `Rate` from a `Generic` `Rate`
                ///
                /// # Examples
                ///
                /// ```rust
                /// # use embedded_time::{Fraction, rate::units::*, rate::Generic};
                /// # use core::convert::{TryFrom, TryInto};
                /// #
                /// assert_eq!(
                ///     Hertz::<u64>::try_from(Generic::new(2_000_u32, Fraction::new(1,1_000))),
                ///     Ok(Hertz(2_u64))
                /// );
                ///
                /// assert_eq!(
                ///     Generic::new(2_000_u64, Fraction::new(1,1_000)).try_into(),
                ///     Ok(Hertz(2_u64))
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
                /// # use embedded_time::{Fraction, rate::units::*, rate::Generic, ConversionError};
                /// # use core::convert::TryFrom;
                /// #
                /// assert_eq!(Hertz::<u32>::try_from(Generic::new(u32::MAX, Fraction::new(10,1))),
                ///     Err(ConversionError::Overflow));
                /// ```
                ///
                /// ---
                ///
                /// [`ConversionError::ConversionFailure`] : The _integer_ conversion to that of the
                /// destination type fails.
                ///
                /// ```rust
                /// # use embedded_time::{Fraction, rate::units::*, rate::Generic, ConversionError};
                /// # use core::convert::TryFrom;
                /// #
                /// assert_eq!(Hertz::<u32>::try_from(Generic::new(u32::MAX as u64 + 1, Fraction::new(1,1))),
                ///     Err(ConversionError::ConversionFailure));
                /// ```
                fn try_from(generic_rate: Generic<SourceInt>) -> Result<Self, Self::Error> {
                    fixed_point::from_ticks(generic_rate.integer, generic_rate.scaling_factor)
                }
            }

            impl<T: TimeInt> From<$name<T>> for Generic<T> {
                // TODO: documentation
                fn from(rate: $name<T>) -> Self {
                    Self::new(*rate.integer(), $name::<T>::SCALING_FACTOR)
                }
            }
        };
    }
    impl_rate![Megahertz, (1_000_000, 1), "Hertz × 1,000,000"];
    impl_rate![Kilohertz, (1_000, 1), "Hertz × 1,000"];
    impl_rate![Hertz, (1, 1), "Hertz"];

    impl_rate![MebibytesPerSecond, (1_048_576, 1), "Bytes/s × 1,048,576"];
    impl_rate![MegabytesPerSecond, (1_000_000, 1), "Bytes/s × 1,000,000"];
    impl_rate![KibibytesPerSecond, (1_024, 1), "Bytes/s × 1,024"];
    impl_rate![KiloBytesPerSecond, (1_000, 1), "Bytes/s × 1,000"];
    impl_rate![BytesPerSecond, (1, 1), "Bytes/s"];

    impl_rate![MebibitsPerSecond, (1_048_576, 1), "Bits/s × 1,048,576"];
    impl_rate![MegabitsPerSecond, (1_000_000, 1), "Bits/s × 1,000,000"];
    impl_rate![KibibitsPerSecond, (1_024, 1), "Bits/s × 1,024"];
    impl_rate![KilobitsPerSecond, (1_000, 1), "Bits/s × 1,000"];
    impl_rate![BitsPerSecond, (1, 1), "Bits/s"];

    impl_rate![Mebibaud, (1_048_576, 1), "Baud × 1,048,576"];
    impl_rate![Megabaud, (1_000_000, 1), "Baud × 1,000,000"];
    impl_rate![Kibibaud, (1_024, 1), "Baud × 1,024"];
    impl_rate![Kilobaud, (1_000, 1), "Baud × 1,000"];
    impl_rate![Baud, (1, 1), "Baud"];

    /// Create time-based values from primitive and core numeric types.
    ///
    /// This trait is anonomously re-exported in [`traits`](crate::traits)
    ///
    /// # Examples
    /// Basic construction of time-based values.
    /// ```rust
    /// # use embedded_time::{traits::*, rate::units::*};
    /// assert_eq!(5_u32.MHz(), Megahertz(5_u32));
    /// assert_eq!(5_u32.kHz(), Kilohertz(5_u32));
    /// assert_eq!(5_u32.Hz(), Hertz(5_u32));
    /// assert_eq!(5_u32.MiBps(), MebibytesPerSecond(5_u32));
    /// assert_eq!(5_u32.MBps(), MegabytesPerSecond(5_u32));
    /// assert_eq!(5_u32.KiBps(), KibibytesPerSecond(5_u32));
    /// assert_eq!(5_u32.kBps(), KiloBytesPerSecond(5_u32));
    /// assert_eq!(5_u32.Bps(), BytesPerSecond(5_u32));
    /// assert_eq!(5_u32.Mibps(), MebibitsPerSecond(5_u32));
    /// assert_eq!(5_u32.Mbps(), MegabitsPerSecond(5_u32));
    /// assert_eq!(5_u32.Kibps(), KibibitsPerSecond(5_u32));
    /// assert_eq!(5_u32.kbps(), KilobitsPerSecond(5_u32));
    /// assert_eq!(5_u32.bps(), BitsPerSecond(5_u32));
    /// assert_eq!(5_u32.MiBd(), Mebibaud(5_u32));
    /// assert_eq!(5_u32.MBd(), Megabaud(5_u32));
    /// assert_eq!(5_u32.KiBd(), Kibibaud(5_u32));
    /// assert_eq!(5_u32.kBd(), Kilobaud(5_u32));
    /// assert_eq!(5_u32.Bd(), Baud(5_u32));
    /// ```
    #[allow(non_snake_case)]
    pub trait Extensions: TimeInt {
        /// megahertz
        fn MHz(self) -> Megahertz<Self> {
            Megahertz::new(self)
        }

        /// kilohertz
        fn kHz(self) -> Kilohertz<Self> {
            Kilohertz::new(self)
        }

        /// hertz
        fn Hz(self) -> Hertz<Self> {
            Hertz::new(self)
        }

        /// mebibytes per second
        fn MiBps(self) -> MebibytesPerSecond<Self> {
            MebibytesPerSecond::new(self)
        }

        /// megabytes per second
        fn MBps(self) -> MegabytesPerSecond<Self> {
            MegabytesPerSecond::new(self)
        }

        /// kibibytes per second
        fn KiBps(self) -> KibibytesPerSecond<Self> {
            KibibytesPerSecond::new(self)
        }

        /// kiloBytes per second
        fn kBps(self) -> KiloBytesPerSecond<Self> {
            KiloBytesPerSecond::new(self)
        }

        /// bytes per second
        fn Bps(self) -> BytesPerSecond<Self> {
            BytesPerSecond::new(self)
        }

        /// mebibits per second
        fn Mibps(self) -> MebibitsPerSecond<Self> {
            MebibitsPerSecond::new(self)
        }

        /// megabits per second
        fn Mbps(self) -> MegabitsPerSecond<Self> {
            MegabitsPerSecond::new(self)
        }

        /// kibibits per second
        fn Kibps(self) -> KibibitsPerSecond<Self> {
            KibibitsPerSecond::new(self)
        }

        /// kilobits per second
        fn kbps(self) -> KilobitsPerSecond<Self> {
            KilobitsPerSecond::new(self)
        }

        /// bits per second
        fn bps(self) -> BitsPerSecond<Self> {
            BitsPerSecond::new(self)
        }

        /// mebibaud
        fn MiBd(self) -> Mebibaud<Self> {
            Mebibaud::new(self)
        }

        /// megabaud
        fn MBd(self) -> Megabaud<Self> {
            Megabaud::new(self)
        }

        /// kibibaud
        fn KiBd(self) -> Kibibaud<Self> {
            Kibibaud::new(self)
        }

        /// kilobaud
        fn kBd(self) -> Kilobaud<Self> {
            Kilobaud::new(self)
        }

        /// baud
        fn Bd(self) -> Baud<Self> {
            Baud::new(self)
        }
    }

    impl Extensions for u32 {}
    impl Extensions for u64 {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{duration::units::*, rate::units::*, traits::*};

    #[test]
    fn try_from_generic() {
        assert_eq!(
            Hertz::try_from(Generic::new(246_u32, Fraction::new(1, 2))),
            Ok(Hertz(123_u32))
        );
    }

    #[test]
    fn to_generic() {
        assert_eq!(
            Hertz(123_u32).to_generic(Fraction::new(1, 2)),
            Ok(Generic::new(246_u32, Fraction::new(1, 2)))
        );
    }

    #[test]
    fn try_into_generic_err() {
        assert_eq!(
            Hertz(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
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
        let mut time = 1_u32;
        time *= 60;
        assert_eq!(Hertz(time), Hertz(60_u32));
    }

    #[test]
    fn remainder() {
        assert_eq!(Hertz(456_u32) % Hertz(100_u32), Hertz(56_u32));
        assert_eq!(Hertz(2_003_u32) % Kilohertz(1_u32), Hertz(3_u32));
        assert_eq!(Kilohertz(40_u32) % Hertz(100_u32), Kilohertz(0_u32));
    }

    #[test]
    fn convert_to_duration() {
        assert_eq!(Hertz(500_u32).to_duration(), Ok(Milliseconds(2_u32)));

        assert_eq!(Kilohertz(500_u32).to_duration(), Ok(Microseconds(2_u32)));
    }

    #[test]
    fn frequency_scaling() {
        assert_eq!(1_u32.Hz(), 1_u32.Hz());
        assert_eq!(1_u32.kHz(), 1_000_u32.Hz());
        assert_eq!(1_u32.MHz(), 1_000_000_u32.Hz());
    }

    #[test]
    fn bytes_per_second_scaling() {
        assert_eq!(1_u32.Bps(), 1_u32.Bps());
        assert_eq!(1_u32.kBps(), 1_000_u32.Bps());
        assert_eq!(1_u32.KiBps(), 1_024_u32.Bps());
        assert_eq!(1_u32.MBps(), 1_000_000_u32.Bps());
        assert_eq!(1_u32.MiBps(), 1_048_576_u32.Bps());
    }

    #[test]
    fn bits_per_second_scaling() {
        assert_eq!(1_u32.bps(), 1_u32.bps());
        assert_eq!(1_u32.kbps(), 1_000_u32.bps());
        assert_eq!(1_u32.Kibps(), 1_024_u32.bps());
        assert_eq!(1_u32.Mbps(), 1_000_000_u32.bps());
        assert_eq!(1_u32.Mibps(), 1_048_576_u32.bps());
    }

    #[test]
    fn baud_scaling() {
        assert_eq!(1_u32.Bd(), 1_u32.Bd());
        assert_eq!(1_u32.kBd(), 1_000_u32.Bd());
        assert_eq!(1_u32.KiBd(), 1_024_u32.Bd());
        assert_eq!(1_u32.MBd(), 1_000_000_u32.Bd());
        assert_eq!(1_u32.MiBd(), 1_048_576_u32.Bd());
    }
}
