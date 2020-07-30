//! Fixed-point values
use crate::{fraction::Fraction, time_int::TimeInt, ConversionError};
use core::{convert::TryFrom, fmt, mem::size_of, ops, prelude::v1::*};
use num::Bounded;

/// Fixed-point value type
///
/// QX.32 where X: bit-width of `T`
pub trait FixedPoint: Sized + Copy + fmt::Display {
    /// The _integer_ (magnitude) type
    #[doc(hidden)]
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
    /// # use embedded_time::{Fraction,  rate::*};
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
                TimeInt::checked_mul_fraction(&ticks, &Self::SCALING_FACTOR.checked_div(&fraction)?)
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
                    &Self::SCALING_FACTOR.checked_div(&fraction)?,
                )?
            };

            T::try_from(ticks).map_err(|_| ConversionError::ConversionFailure)
        }
    }

    /// Attempt to convert from one duration type to another
    ///
    /// The integer type and/or the _scaling factor_ may be changed.
    ///
    /// # Errors
    ///
    /// Failure will only occur if the provided value does not fit in the selected destination type.
    ///
    /// [`ConversionError::Overflow`] - The conversion of the _scaling factor_ causes an overflow.
    /// [`ConversionError::ConversionFailure`] - The integer type cast to that of the destination
    /// type fails.
    fn try_convert_from<Source: FixedPoint>(source: Source) -> Result<Self, ConversionError>
    where
        Self::T: TryFrom<Source::T>,
    {
        from_ticks(*source.integer(), Source::SCALING_FACTOR)
    }

    /// The reciprocal of [`FixedPoint::try_convert_from()`]
    ///
    /// The _integer_ type and/or the _scaling factor_ may be changed.
    ///
    /// # Errors
    ///
    /// Failure will only occur if the provided value does not fit in the selected destination type.
    ///
    /// [`ConversionError::Overflow`] - The conversion of the _scaling factor_ causes an overflow.
    /// [`ConversionError::ConversionFailure`] - The integer type cast to that of the destination
    /// type fails.
    fn try_convert_into<Dest: FixedPoint>(self) -> Result<Dest, ConversionError>
    where
        Dest::T: TryFrom<Self::T>,
    {
        Dest::try_convert_from(self)
    }

    /// Panicky addition
    #[doc(hidden)]
    fn add<Rhs: FixedPoint>(self, rhs: Rhs) -> Self
    where
        Self::T: TryFrom<Rhs::T>,
    {
        Self::new(*self.integer() + *Self::try_convert_from(rhs).unwrap().integer())
    }

    /// Panicky subtraction
    #[doc(hidden)]
    fn sub<Rhs: FixedPoint>(self, rhs: Rhs) -> Self
    where
        Self::T: TryFrom<Rhs::T>,
    {
        Self::new(*self.integer() - *Self::try_convert_from(rhs).unwrap().integer())
    }

    /// Panicky remainder
    #[doc(hidden)]
    fn rem<Rhs: FixedPoint>(self, rhs: Rhs) -> Self
    where
        Self::T: TryFrom<Rhs::T>,
    {
        let rhs = *Self::try_convert_from(rhs).unwrap().integer();

        if rhs > Self::T::from(0) {
            Self::new(*self.integer() % rhs)
        } else {
            Self::new(Self::T::from(0))
        }
    }

    /// Panicky equality
    #[doc(hidden)]
    fn eq<Rhs: FixedPoint>(&self, rhs: &Rhs) -> bool
    where
        Self::T: TryFrom<Rhs::T>,
        Rhs::T: TryFrom<Self::T>,
    {
        if Self::SCALING_FACTOR < Rhs::SCALING_FACTOR {
            self.integer() == Self::try_convert_from(*rhs).unwrap().integer()
        } else {
            Rhs::try_convert_from(*self).unwrap().integer() == rhs.integer()
        }
    }

    /// Panicky comparison
    #[doc(hidden)]
    fn partial_cmp<Rhs: FixedPoint>(&self, rhs: &Rhs) -> Option<core::cmp::Ordering>
    where
        Self::T: TryFrom<Rhs::T>,
        Rhs::T: TryFrom<Self::T>,
    {
        if Self::SCALING_FACTOR < Rhs::SCALING_FACTOR {
            Some(
                self.integer()
                    .cmp(&Self::try_convert_from(*rhs).unwrap().integer()),
            )
        } else {
            Some(
                Rhs::try_convert_from(*self)
                    .unwrap()
                    .integer()
                    .cmp(&rhs.integer()),
            )
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
                &scaling_factor.checked_div(&Dest::SCALING_FACTOR)?,
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
                &scaling_factor.checked_div(&Dest::SCALING_FACTOR)?,
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
}
