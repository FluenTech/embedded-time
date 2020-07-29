use crate::{ConversionError, Fraction};
use core::{fmt, ops};

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
    + ops::Mul<Fraction, Output = Self>
    + ops::Div<Fraction, Output = Self>
    + fmt::Display
    + fmt::Debug
{
    /// Checked integer × [`Fraction`] = integer
    ///
    /// Returns truncated integer
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

    // /// Panicky integer × [`Fraction`] = integer
    // ///
    // /// Returns truncated integer
    // fn mul_fraction(&self, fraction: &Fraction) -> Self {
    //     fraction.integer_mul(*self)
    //     // *self / (*fraction.denominator()).into() * (*fraction.numerator()).into()
    // }
    //
    // /// Panicky integer / [`Fraction`] = integer
    // ///
    // /// Returns truncated integer
    // fn div_fraction(&self, fraction: &Fraction) -> Self {
    //     *self * (*fraction.denominator()).into() / (*fraction.numerator()).into()
    // }
}

impl TimeInt for u32 {}
impl TimeInt for u64 {}

pub trait Widen {
    type Output;
    fn widen(&self) -> Self::Output;
}

impl Widen for u32 {
    type Output = u64;

    fn widen(&self) -> Self::Output {
        self.clone().into()
    }
}

impl Widen for u64 {
    type Output = u128;

    fn widen(&self) -> Self::Output {
        self.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Fraction, TimeInt};

    #[test]
    fn checked_integer_mul_fraction() {
        assert_eq!(8_u32.checked_mul_fraction(&Fraction::new(1, 2)), Ok(4_u32));

        // the result is not rounded, but truncated (8×(1/3)=2.66)
        assert_eq!(8_u32.checked_mul_fraction(&Fraction::new(1, 3)), Ok(2_u32));
    }

    #[test]
    fn checked_integer_div_fraction() {
        assert_eq!(8_u32.checked_div_fraction(&Fraction::new(1, 2)), Ok(16_u32));

        // the result is not rounded, but truncated (8/3=2.66)
        assert_eq!(8_u32.checked_div_fraction(&Fraction::new(3, 1)), Ok(2_u32));
    }
}
