//! Fixed-point values
use crate::{fraction::Fraction, time_int::TimeInt, ConversionError};
use core::{convert::TryFrom, fmt, mem::size_of, prelude::v1::*};
use num::Bounded;

/// Fixed-point value type
///
/// QX.32 where X: bit-width of `Rep`
pub trait FixedPoint: Sized + Copy + fmt::Display {
    /// The integer (magnitude) type
    type Rep: TimeInt;

    /// The fractional scaling factor
    const SCALING_FACTOR: Fraction;

    /// Not generally useful to call directly
    ///
    /// It only exists to allow FixedPoint methods with default definitions to create a
    /// new fixed-point type
    fn new(value: Self::Rep) -> Self;

    /// Returns the integer value of the `FixedPoint`
    fn count(self) -> Self::Rep;

    /// Returns the integer of the fixed-point value after converting to the _scaling factor_
    /// provided
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_time::{Fraction, traits::*};
    /// use embedded_time::rate::units::Hertz;
    /// assert_eq!(Hertz(2_u32).into_ticks(Fraction::new(1, 1_000)), Ok(2_000_u32));
    /// ```
    ///
    /// # Errors
    ///
    /// Failure will only occur if the converted integer does not fit in the selected destination
    /// type.
    ///
    /// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
    /// [`ConversionError::ConversionFailure`] : The integer type cast to that of the destination
    /// fails.
    fn into_ticks<Rep: TimeInt>(self, fraction: Fraction) -> Result<Rep, ConversionError>
    where
        Self::Rep: TimeInt,
        Rep: TryFrom<Self::Rep>,
    {
        if size_of::<Rep>() > size_of::<Self::Rep>() {
            let ticks =
                Rep::try_from(self.count()).map_err(|_| ConversionError::ConversionFailure)?;

            if fraction > Fraction::new(1, 1) {
                TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&ticks, &Self::SCALING_FACTOR)?,
                    &fraction,
                )
            } else {
                TimeInt::checked_mul_period(&ticks, &Self::SCALING_FACTOR.checked_div(&fraction)?)
            }
        } else {
            let ticks = if Self::SCALING_FACTOR > Fraction::new(1, 1) {
                TimeInt::checked_div_period(
                    &TimeInt::checked_mul_period(&self.count(), &Self::SCALING_FACTOR)?,
                    &fraction,
                )?
            } else {
                TimeInt::checked_mul_period(
                    &self.count(),
                    &Self::SCALING_FACTOR.checked_div(&fraction)?,
                )?
            };

            Rep::try_from(ticks).map_err(|_| ConversionError::ConversionFailure)
        }
    }

    /// Attempt to convert from one duration type to another
    ///
    /// The integer type and/or the scaling factor may be changed.
    ///
    /// # Errors
    ///
    /// Failure will only occur if the resulting integer does not fit in the selected destination
    /// type.
    ///
    /// [`ConversionError::Overflow`] - The conversion of the _scaling factor_ causes an overflow.
    /// [`ConversionError::ConversionFailure`] - The integer type cast to that of the destination
    /// type fails.
    fn try_convert_from<Source: FixedPoint>(source: Source) -> Result<Self, ConversionError>
    where
        Self::Rep: TryFrom<Source::Rep>,
    {
        from_ticks(source.count(), Source::SCALING_FACTOR)
    }

    /// The reciprocal of [`FixedPoint::try_convert_from()`]
    ///
    /// The integer type and/or the scaling factor may be changed.
    ///
    /// # Errors
    ///
    /// Failure will only occur if the resulting integer does not fit in the selected destination
    /// type.
    ///
    /// [`ConversionError::Overflow`] - The conversion of the _scaling factor_ causes an overflow.
    /// [`ConversionError::ConversionFailure`] - The integer type cast to that of the destination
    /// type fails.
    fn try_convert_into<Dest: FixedPoint>(self) -> Result<Dest, ConversionError>
    where
        Dest::Rep: TryFrom<Self::Rep>,
    {
        Dest::try_convert_from(self)
    }

    /// Panicky addition
    fn add<Rhs: FixedPoint>(self, rhs: Rhs) -> Self
    where
        Self::Rep: TryFrom<Rhs::Rep>,
    {
        Self::new(self.count() + Self::try_convert_from(rhs).unwrap().count())
    }

    /// Panicky subtraction
    fn sub<Rhs: FixedPoint>(self, rhs: Rhs) -> Self
    where
        Self::Rep: TryFrom<Rhs::Rep>,
    {
        Self::new(self.count() - Self::try_convert_from(rhs).unwrap().count())
    }

    /// Panicky remainder
    fn rem<Rhs: FixedPoint>(self, rhs: Rhs) -> Self
    where
        Self::Rep: TryFrom<Rhs::Rep>,
    {
        let rhs = Self::try_convert_from(rhs).unwrap().count();

        if rhs > Self::Rep::from(0) {
            Self::new(self.count() % rhs)
        } else {
            Self::new(Self::Rep::from(0))
        }
    }

    /// Panicky equality
    fn eq<Rhs: FixedPoint>(&self, rhs: &Rhs) -> bool
    where
        Self::Rep: TryFrom<Rhs::Rep>,
        Rhs::Rep: TryFrom<Self::Rep>,
    {
        if Self::SCALING_FACTOR < Rhs::SCALING_FACTOR {
            self.count() == Self::try_convert_from(*rhs).unwrap().count()
        } else {
            Rhs::try_convert_from(*self).unwrap().count() == rhs.count()
        }
    }

    /// Panicky comparison
    fn partial_cmp<Rhs: FixedPoint>(&self, rhs: &Rhs) -> Option<core::cmp::Ordering>
    where
        Self::Rep: TryFrom<Rhs::Rep>,
        Rhs::Rep: TryFrom<Self::Rep>,
    {
        if Self::SCALING_FACTOR < Rhs::SCALING_FACTOR {
            Some(
                self.count()
                    .cmp(&Self::try_convert_from(*rhs).unwrap().count()),
            )
        } else {
            Some(
                Rhs::try_convert_from(*self)
                    .unwrap()
                    .count()
                    .cmp(&rhs.count()),
            )
        }
    }

    /// Returns the minimum integer value
    #[must_use]
    fn min_value() -> Self::Rep {
        Self::Rep::min_value()
    }

    /// Returns the maximum integer value
    #[must_use]
    fn max_value() -> Self::Rep {
        Self::Rep::max_value()
    }
}

/// Constructs a `FixedPoint` from an integer and scaling-factor fraction
///
/// # Errors
///
/// Failure will only occur if the integer does not fit in the selected destination type.
///
/// [`ConversionError::Overflow`] : The conversion of the _scaling factor_ causes an overflow.
/// [`ConversionError::ConversionFailure`] : The integer cast to that of the destination
/// type fails.
pub(crate) fn from_ticks<SourceInt: TimeInt, Dest: FixedPoint>(
    ticks: SourceInt,
    scaling_factor: Fraction,
) -> Result<Dest, ConversionError>
where
    Dest::Rep: TryFrom<SourceInt>,
{
    if size_of::<Dest::Rep>() > size_of::<SourceInt>() {
        let ticks = Dest::Rep::try_from(ticks).map_err(|_| ConversionError::ConversionFailure)?;

        let ticks = if scaling_factor > Fraction::new(1, 1) {
            TimeInt::checked_div_period(
                &TimeInt::checked_mul_period(&ticks, &scaling_factor)?,
                &Dest::SCALING_FACTOR,
            )?
        } else {
            TimeInt::checked_mul_period(
                &ticks,
                &scaling_factor.checked_div(&Dest::SCALING_FACTOR)?,
            )?
        };

        Ok(Dest::new(ticks))
    } else {
        let ticks = if scaling_factor > Fraction::new(1, 1) {
            TimeInt::checked_div_period(
                &TimeInt::checked_mul_period(&ticks, &scaling_factor)?,
                &Dest::SCALING_FACTOR,
            )?
        } else if Dest::SCALING_FACTOR > Fraction::new(1, 1) {
            TimeInt::checked_mul_period(
                &TimeInt::checked_div_period(&ticks, &Dest::SCALING_FACTOR)?,
                &scaling_factor,
            )?
        } else {
            TimeInt::checked_mul_period(
                &ticks,
                &scaling_factor.checked_div(&Dest::SCALING_FACTOR)?,
            )?
        };

        let ticks = Dest::Rep::try_from(ticks).map_err(|_| ConversionError::ConversionFailure)?;

        Ok(Dest::new(ticks))
    }
}
