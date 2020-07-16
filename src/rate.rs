//! Representations of frequency-based values

pub(crate) mod units {
    use crate::{ConversionError, Fraction, TimeInt};
    use core::{convert, ops};
    use num::Zero;

    /// A frequency unit type
    ///
    /// Convertible to/from [`Fraction`].
    #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    pub struct Hertz<T: TimeInt = u32>(pub T);

    impl<T: TimeInt, Rhs: TimeInt> ops::Mul<Rhs> for Hertz<T>
    where
        T: From<Rhs>,
    {
        type Output = Self;

        /// ```rust
        /// # use embedded_time::units::*;
        /// assert_eq!(Hertz(100_u32) * 3_u32, Hertz(300_u32));
        /// ```
        fn mul(self, rhs: Rhs) -> Self::Output {
            Self(self.0 * <T as convert::From<Rhs>>::from(rhs))
        }
    }

    impl<T: TimeInt, Rhs: TimeInt> ops::Div<Rhs> for Hertz<T>
    where
        T: From<Rhs>,
    {
        type Output = Self;

        /// ```rust
        /// # use embedded_time::units::*;
        /// assert_eq!(Hertz(300_u32) / 3_u32, Hertz(100_u32));
        /// ```
        fn div(self, rhs: Rhs) -> Self::Output {
            Self(self.0 / <T as convert::From<Rhs>>::from(rhs))
        }
    }

    impl<T: TimeInt> convert::TryFrom<Fraction> for Hertz<T> {
        type Error = ConversionError;

        /// Create a new `Frequency` from a [`Fraction`] in seconds
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use embedded_time::{Fraction, units::*, ConversionError};
        /// # use core::{convert::{TryFrom, TryInto}};
        /// #
        /// assert_eq!(<Hertz>::try_from(Fraction::new(1, 1_000)),
        ///     Ok(Hertz(1_000_u32)));
        ///
        /// assert_eq!(<Hertz>::try_from(Fraction::new(0, 1_000)),
        ///     Err(ConversionError::DivByZero));
        ///
        /// assert_eq!(<Fraction>::new(1, 1_000).try_into(),
        ///     Ok(Hertz(1_000_u32)));
        /// ```
        ///
        /// # Errors
        ///
        /// [`ConversionError::DivByZero`]
        fn try_from(fraction: Fraction) -> Result<Self, Self::Error> {
            if !fraction.numerator().is_zero() {
                Ok(Hertz(T::from(fraction.recip().to_integer())))
            } else {
                Err(ConversionError::DivByZero)
            }
        }
    }

    impl convert::From<u32> for Hertz {
        /// ```rust
        /// # use embedded_time::units::Hertz;
        /// #
        /// assert_eq!(Hertz::from(23), Hertz(23_u32));
        /// assert_eq!(<Hertz as Into<u32>>::into(Hertz(23_u32)), 23_u32);
        /// ```
        fn from(value: u32) -> Self {
            Self(value)
        }
    }

    impl convert::From<Hertz> for u32 {
        /// ```rust
        /// # use embedded_time::units::Hertz;
        /// #
        /// assert_eq!(u32::from(Hertz(23)), 23_u32);
        /// assert_eq!(<u32 as Into<Hertz>>::into(23), Hertz(23_u32));
        /// ```
        fn from(hertz: Hertz) -> Self {
            hertz.0
        }
    }
}
