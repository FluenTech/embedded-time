//! Fixed-point values
use crate::{fraction::Fraction, time_int::TimeInt, ConversionError};
use core::{convert::TryFrom, mem::size_of, ops, prelude::v1::*};
use num::Bounded;

/// Fixed-point value type
///
/// QX.32 where X: bit-width of `T`
pub trait FixedPoint: Sized + Copy {
    /// The _integer_ (magnitude) type
    type T: TimeInt;

    /// The fractional _scaling factor_
    const SCALING_FACTOR: Fraction;

    /// Not generally useful to call directly
    ///
    /// It only exists to allow FixedPoint methods with default definitions to create a
    /// new fixed-point type
    #[doc(hidden)]
    fn new(value: Self::T) -> Self;

    /// Returns the integer part of the `FixedPoint` value
    ///
    /// ```rust
    /// # use embedded_time::{ rate::*};
    /// #
    /// assert_eq!(Hertz(45_u32).integer(), &45_u32);
    /// ```
    fn integer(&self) -> &Self::T;

    /// Constructs a `FixedPoint` value from _integer_ and _scaling-factor_ ([`Fraction`]) parts
    ///
    /// # Errors
    ///
    /// Failure will only occur if the provided value does not fit in the selected destination type.
    ///
    /// - [`ConversionError::Unspecified`]
    /// - [`ConversionError::Overflow`]
    /// - [`ConversionError::ConversionFailure`]
    #[doc(hidden)]
    fn from_ticks<SourceInt: TimeInt>(
        ticks: SourceInt,
        scaling_factor: Fraction,
    ) -> Result<Self, ConversionError>
    where
        Self::T: TryFrom<SourceInt>,
    {
        if size_of::<Self::T>() > size_of::<SourceInt>() {
            // the dest integer is wider than the source, first promote the source integer to the
            // dest type
            let ticks = Self::T::try_from(ticks).map_err(|_| ConversionError::ConversionFailure)?;

            let ticks = if scaling_factor > Fraction::new(1, 1) {
                // In order to preserve precision, if the source scaling factor is > 1, the source's
                // pure integer value can be calculated first followed by division by the
                // dest scaling factor.
                TimeInt::checked_div_fraction(
                    &TimeInt::checked_mul_fraction(&ticks, &scaling_factor)
                        .ok_or(ConversionError::Unspecified)?,
                    &Self::SCALING_FACTOR,
                )
                .ok_or(ConversionError::Unspecified)?
            } else {
                // If the source scaling factor is <= 1, the relative ratio of the scaling factors
                // are calculated first by dividing the source scaling factor by
                // that of the dest. The source integer part is then multiplied by
                // the result.
                TimeInt::checked_mul_fraction(
                    &ticks,
                    &scaling_factor
                        .checked_div(&Self::SCALING_FACTOR)
                        .ok_or(ConversionError::Unspecified)?,
                )
                .ok_or(ConversionError::Unspecified)?
            };

            Ok(Self::new(ticks))
        } else {
            let ticks = if scaling_factor > Fraction::new(1, 1) {
                TimeInt::checked_div_fraction(
                    &TimeInt::checked_mul_fraction(&ticks, &scaling_factor)
                        .ok_or(ConversionError::Unspecified)?,
                    &Self::SCALING_FACTOR,
                )
                .ok_or(ConversionError::Unspecified)?
            } else if Self::SCALING_FACTOR > Fraction::new(1, 1) {
                TimeInt::checked_mul_fraction(
                    &TimeInt::checked_div_fraction(&ticks, &Self::SCALING_FACTOR)
                        .ok_or(ConversionError::Unspecified)?,
                    &scaling_factor,
                )
                .ok_or(ConversionError::Unspecified)?
            } else {
                TimeInt::checked_mul_fraction(
                    &ticks,
                    &scaling_factor
                        .checked_div(&Self::SCALING_FACTOR)
                        .ok_or(ConversionError::Unspecified)?,
                )
                .ok_or(ConversionError::Unspecified)?
            };

            let ticks = Self::T::try_from(ticks).map_err(|_| ConversionError::ConversionFailure)?;

            Ok(Self::new(ticks))
        }
    }

    /// Constructs a `FixedPoint` value from _integer_ and _scaling-factor_ ([`Fraction`]) parts
    ///
    #[doc(hidden)]
    fn from_ticks_safe<SourceInt: TimeInt>(ticks: SourceInt, scaling_factor: Fraction) -> Self
    where
        Self::T: From<SourceInt>,
        Self::T: ops::Mul<Fraction, Output = Self::T> + ops::Div<Fraction, Output = Self::T>,
    {
        let ticks = Self::T::from(ticks);

        let ticks = if (scaling_factor >= Fraction::new(1, 1)
            && Self::SCALING_FACTOR <= Fraction::new(1, 1))
            || (scaling_factor <= Fraction::new(1, 1)
                && Self::SCALING_FACTOR >= Fraction::new(1, 1))
        {
            // if the source's _scaling factor_ is > `1/1`, start by converting to a _scaling
            // factor_ of `1/1`, then convert to destination _scaling factor_.
            (ticks * scaling_factor) / Self::SCALING_FACTOR
        } else {
            // If the source scaling factor is <= 1, the relative ratio of the scaling factors are
            // calculated first by dividing the source scaling factor by that of the
            // dest. The source integer part is then multiplied by the result.
            ticks * (scaling_factor / Self::SCALING_FACTOR)
        };

        Self::new(ticks)
    }

    /// Returns the _integer_ of the fixed-point value after converting to the _scaling factor_
    /// provided
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{fraction::Fraction,  rate::*};
    /// #
    /// assert_eq!(Hertz(2_u32).into_ticks(Fraction::new(1, 1_000)), Ok(2_000_u32));
    /// ```
    ///
    /// # Errors
    ///
    /// Failure will only occur if the provided value does not fit in the selected destination type.
    ///
    /// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
    /// [`ConversionError::ConversionFailure`] : The _integer_ type cast to that of the destination
    /// fails.
    #[doc(hidden)]
    fn into_ticks<T: TimeInt>(self, fraction: Fraction) -> Result<T, ConversionError>
    where
        Self::T: TimeInt,
        T: TryFrom<Self::T>,
    {
        if size_of::<T>() > size_of::<Self::T>() {
            let ticks =
                T::try_from(*self.integer()).map_err(|_| ConversionError::ConversionFailure)?;

            if fraction > Fraction::new(1, 1) {
                TimeInt::checked_div_fraction(
                    &TimeInt::checked_mul_fraction(&ticks, &Self::SCALING_FACTOR)
                        .ok_or(ConversionError::Unspecified)?,
                    &fraction,
                )
                .ok_or(ConversionError::Unspecified)
            } else {
                TimeInt::checked_mul_fraction(
                    &ticks,
                    &Self::SCALING_FACTOR
                        .checked_div(&fraction)
                        .ok_or(ConversionError::Unspecified)?,
                )
                .ok_or(ConversionError::Unspecified)
            }
        } else {
            let ticks = if Self::SCALING_FACTOR > Fraction::new(1, 1) {
                TimeInt::checked_div_fraction(
                    &TimeInt::checked_mul_fraction(self.integer(), &Self::SCALING_FACTOR)
                        .ok_or(ConversionError::Unspecified)?,
                    &fraction,
                )
                .ok_or(ConversionError::Unspecified)?
            } else {
                TimeInt::checked_mul_fraction(
                    self.integer(),
                    &Self::SCALING_FACTOR
                        .checked_div(&fraction)
                        .ok_or(ConversionError::Unspecified)?,
                )
                .ok_or(ConversionError::Unspecified)?
            };

            T::try_from(ticks).map_err(|_| ConversionError::ConversionFailure)
        }
    }

    /// Panicky addition
    #[doc(hidden)]
    fn add<Rhs: FixedPoint>(self, rhs: Rhs) -> Self
    where
        Self: TryFrom<Rhs>,
    {
        Self::new(*self.integer() + *Self::try_from(rhs).ok().unwrap().integer())
    }

    /// Panicky subtraction
    #[doc(hidden)]
    fn sub<Rhs: FixedPoint>(self, rhs: Rhs) -> Self
    where
        Self: TryFrom<Rhs>,
    {
        Self::new(*self.integer() - *Self::try_from(rhs).ok().unwrap().integer())
    }

    /// Panicky remainder
    #[doc(hidden)]
    fn rem<Rhs: FixedPoint>(self, rhs: Rhs) -> Self
    where
        Self: TryFrom<Rhs>,
    {
        match Self::try_from(rhs) {
            Ok(rhs) => {
                if *rhs.integer() > Self::T::from(0) {
                    Self::new(*self.integer() % *rhs.integer())
                } else {
                    Self::new(Self::T::from(0))
                }
            }
            Err(_) => self,
        }
    }

    /// Returns the minimum integer value
    fn min_value() -> Self::T {
        Self::T::min_value()
    }

    /// Returns the maximum integer value
    fn max_value() -> Self::T {
        Self::T::max_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration::*;
    use crate::fixed_point;

    #[test]
    fn from_ticks() {
        assert_eq!(
            fixed_point::FixedPoint::from_ticks(200_u32, Fraction::new(1, 1_000)),
            Ok(Milliseconds(200_u64))
        );
        assert_eq!(
            fixed_point::FixedPoint::from_ticks(200_u32, Fraction::new(1_000, 1)),
            Ok(Seconds(200_000_u64))
        );
    }
}
