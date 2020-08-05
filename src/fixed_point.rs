//! Fixed-point values
use crate::{fraction::Fraction, time_int::TimeInt, ConversionError};
use core::{convert::TryFrom, fmt, mem::size_of, ops, prelude::v1::*};
use num::Bounded;

/// Fixed-point value type
///
/// QX.32 where X: bit-width of `T`
pub trait FixedPoint: Sized + Copy + fmt::Display {
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

    /// Returns the integer value of the `FixedPoint`
    ///
    /// ```rust
    /// # use embedded_time::{ rate::*};
    /// #
    /// assert_eq!(Hertz(45_u32).integer(), &45_u32);
    /// ```
    fn integer(&self) -> &Self::T;

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
                    &TimeInt::checked_mul_fraction(&ticks, &Self::SCALING_FACTOR)?,
                    &fraction,
                )
            } else {
                TimeInt::checked_mul_fraction(
                    &ticks,
                    &Self::SCALING_FACTOR
                        .checked_div(&fraction)
                        .ok_or(ConversionError::Unspecified)?,
                )
            }
        } else {
            let ticks = if Self::SCALING_FACTOR > Fraction::new(1, 1) {
                TimeInt::checked_div_fraction(
                    &TimeInt::checked_mul_fraction(self.integer(), &Self::SCALING_FACTOR)?,
                    &fraction,
                )?
            } else {
                TimeInt::checked_mul_fraction(
                    self.integer(),
                    &Self::SCALING_FACTOR
                        .checked_div(&fraction)
                        .ok_or(ConversionError::Unspecified)?,
                )?
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

/// Constructs a `FixedPoint` from an integer and scaling-factor fraction
///
/// # Errors
///
/// Failure will only occur if the provided value does not fit in the selected destination type.
///
/// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
/// [`ConversionError::ConversionFailure`] : The integer conversion to that of the destination
/// type fails.
// TODO: Move back into FixedPoint?
#[doc(hidden)]
pub(crate) fn from_ticks<SourceInt: TimeInt, Dest: FixedPoint>(
    ticks: SourceInt,
    scaling_factor: Fraction,
) -> Result<Dest, ConversionError>
where
    Dest::T: TryFrom<SourceInt>,
{
    if size_of::<Dest::T>() > size_of::<SourceInt>() {
        // the dest integer is wider than the source, first promote the source integer to the dest
        // type
        let ticks = Dest::T::try_from(ticks).map_err(|_| ConversionError::ConversionFailure)?;

        let ticks = if scaling_factor > Fraction::new(1, 1) {
            // In order to preserve precision, if the source scaling factor is > 1, the source's
            // pure integer value can be calculated first followed by division by the
            // dest scaling factor.
            TimeInt::checked_div_fraction(
                &TimeInt::checked_mul_fraction(&ticks, &scaling_factor)?,
                &Dest::SCALING_FACTOR,
            )?
        } else {
            // If the source scaling factor is <= 1, the relative ratio of the scaling factors are
            // calculated first by dividing the source scaling factor by that of the
            // dest. The source integer part is then multiplied by the result.
            TimeInt::checked_mul_fraction(
                &ticks,
                &scaling_factor
                    .checked_div(&Dest::SCALING_FACTOR)
                    .ok_or(ConversionError::Unspecified)?,
            )?
        };

        Ok(Dest::new(ticks))
    } else {
        let ticks = if scaling_factor > Fraction::new(1, 1) {
            TimeInt::checked_div_fraction(
                &TimeInt::checked_mul_fraction(&ticks, &scaling_factor)?,
                &Dest::SCALING_FACTOR,
            )?
        } else if Dest::SCALING_FACTOR > Fraction::new(1, 1) {
            TimeInt::checked_mul_fraction(
                &TimeInt::checked_div_fraction(&ticks, &Dest::SCALING_FACTOR)?,
                &scaling_factor,
            )?
        } else {
            TimeInt::checked_mul_fraction(
                &ticks,
                &scaling_factor
                    .checked_div(&Dest::SCALING_FACTOR)
                    .ok_or(ConversionError::Unspecified)?,
            )?
        };

        let ticks = Dest::T::try_from(ticks).map_err(|_| ConversionError::ConversionFailure)?;

        Ok(Dest::new(ticks))
    }
}

#[doc(hidden)]
pub(crate) fn from_ticks_safe<SourceInt: TimeInt, Dest: FixedPoint>(
    ticks: SourceInt,
    scaling_factor: Fraction,
) -> Dest
where
    Dest::T: From<SourceInt>,
    Dest::T: ops::Mul<Fraction, Output = Dest::T> + ops::Div<Fraction, Output = Dest::T>,
{
    let ticks = Dest::T::from(ticks);

    let ticks = if (scaling_factor >= Fraction::new(1, 1)
        && Dest::SCALING_FACTOR <= Fraction::new(1, 1))
        || (scaling_factor <= Fraction::new(1, 1) && Dest::SCALING_FACTOR >= Fraction::new(1, 1))
    {
        // if the source's _scaling factor_ is > `1/1`, start by converting to a _scaling factor_
        // of `1/1`, then convert to destination _scaling factor_.
        (ticks * scaling_factor) / Dest::SCALING_FACTOR
    } else {
        // If the source scaling factor is <= 1, the relative ratio of the scaling factors are
        // calculated first by dividing the source scaling factor by that of the
        // dest. The source integer part is then multiplied by the result.
        ticks * (scaling_factor / Dest::SCALING_FACTOR)
    };

    Dest::new(ticks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration::*;
    use crate::fixed_point;

    #[test]
    fn from_ticks() {
        assert_eq!(
            fixed_point::from_ticks(200_u32, Fraction::new(1, 1_000)),
            Ok(Milliseconds(200_u64))
        );
        assert_eq!(
            fixed_point::from_ticks(200_u32, Fraction::new(1_000, 1)),
            Ok(Seconds(200_000_u64))
        );
    }
}
