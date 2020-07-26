use crate::{ConversionError, Fraction};
use core::{convert::TryFrom, convert::TryInto, fmt};

/// The core inner-type trait for time-related types
pub trait TimeInt:
    Copy
    + num::Integer
    + num::Bounded
    + num::traits::WrappingAdd
    + num::traits::WrappingSub
    + num::CheckedAdd
    + num::CheckedSub
    + num::CheckedMul
    + num::CheckedDiv
    + From<u32>
    + TryInto<u32>
    + TryFrom<u64>
    + Into<u64>
    + TryFrom<u128>
    + fmt::Display
    + fmt::Debug
{
    /// Checked integer × [`Fraction`] = integer
    ///
    /// Returns truncated integer
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, prelude::*};
    /// #
    /// assert_eq!(8_u32.checked_mul_fraction(&Fraction::new(1,2)), Ok(4_u32));
    ///
    /// // the result is not rounded, but truncated (8×(1/3)=2.66)
    /// assert_eq!(8_u32.checked_mul_fraction(&Fraction::new(1,3)), Ok(2_u32));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
    // TODO: add example
    /// [`ConversionError::DivByZero`]
    // TODO: add example
    fn checked_mul_fraction(&self, fraction: &Fraction) -> Result<Self, ConversionError> {
        <Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*fraction.numerator()).into())
                .ok_or(ConversionError::Overflow)?,
            &(*fraction.denominator()).into(),
        )
        .ok_or(ConversionError::DivByZero)
    }

    /// Checked integer / [`Fraction`] = integer
    ///
    /// Returns truncated integer
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, prelude::*};
    /// #
    /// assert_eq!(8_u32.checked_div_fraction(&Fraction::new(1,2)), Ok(16_u32));
    ///
    /// // the result is not rounded, but truncated (8/3=2.66)
    /// assert_eq!(8_u32.checked_div_fraction(&Fraction::new(3,1)), Ok(2_u32));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
    // TODO: add example
    /// [`ConversionError::DivByZero`]
    // TODO: add example
    fn checked_div_fraction(&self, fraction: &Fraction) -> Result<Self, ConversionError> {
        <Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*fraction.denominator()).into())
                .ok_or(ConversionError::Overflow)?,
            &(*fraction.numerator()).into(),
        )
        .ok_or(ConversionError::DivByZero)
    }
}

impl TimeInt for u32 {}
impl TimeInt for u64 {}
