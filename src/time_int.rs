use crate::fraction::Fraction;
use core::convert::TryInto;
use core::{fmt, ops};

/// The core inner-type trait for time-related types
#[doc(hidden)]
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
    /// Returns truncated (rounded toward `0`) integer or [`None`] upon failure
    fn checked_mul_fraction(&self, fraction: &Fraction) -> Option<Self> {
        <Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*fraction.numerator()).into())?,
            &(*fraction.denominator()).into(),
        )
    }

    /// Checked integer / [`Fraction`] = integer
    ///
    /// Returns truncated (rounded toward `0`) integer or [`None`] upon failure
    fn checked_div_fraction(&self, fraction: &Fraction) -> Option<Self> {
        <Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*fraction.denominator()).into())?,
            &(*fraction.numerator()).into(),
        )
    }
}

#[doc(hidden)]
impl TimeInt for u32 {
    fn checked_mul_fraction(&self, fraction: &Fraction) -> Option<Self> {
        if fraction.numerator() == &1 {
            self.checked_div(*fraction.denominator())
        } else {
            let integer = self.widen();
            integer
                .checked_mul((*fraction.numerator()).into())?
                .checked_div((*fraction.denominator()).into())?
                .try_into()
                .ok()
        }
    }

    fn checked_div_fraction(&self, fraction: &Fraction) -> Option<Self> {
        if fraction.denominator() == &1 {
            self.checked_div(*fraction.numerator())
        } else {
            let integer = self.widen();
            integer
                .checked_mul((*fraction.denominator()).into())?
                .checked_div((*fraction.numerator()).into())?
                .try_into()
                .ok()
        }
    }
}
#[doc(hidden)]
impl TimeInt for u64 {
    fn checked_mul_fraction(&self, fraction: &Fraction) -> Option<Self> {
        if fraction.numerator() == &1 {
            self.checked_div((*fraction.denominator()).into())
        } else {
            let integer = self.widen();
            integer
                .checked_mul((*fraction.numerator()).into())?
                .checked_div((*fraction.denominator()).into())?
                .try_into()
                .ok()
        }
    }

    fn checked_div_fraction(&self, fraction: &Fraction) -> Option<Self> {
        if fraction.denominator() == &1 {
            self.checked_div((*fraction.numerator()).into())
        } else {
            let integer = self.widen();
            integer
                .checked_mul((*fraction.denominator()).into())?
                .checked_div((*fraction.numerator()).into())?
                .try_into()
                .ok()
        }
    }
}

#[doc(hidden)]
pub trait Widen {
    type Output;
    fn widen(&self) -> Self::Output;
}

#[doc(hidden)]
impl Widen for u32 {
    type Output = u64;

    fn widen(&self) -> Self::Output {
        self.clone().into()
    }
}

#[doc(hidden)]
impl Widen for u64 {
    type Output = u128;

    fn widen(&self) -> Self::Output {
        self.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{fraction::Fraction, time_int::TimeInt};

    #[test]
    fn checked_integer_mul_fraction() {
        assert_eq!(
            8_u32.checked_mul_fraction(&Fraction::new(1, 2)),
            Some(4_u32)
        );

        // the result is not rounded, but truncated (8×(1/3)=2.66)
        assert_eq!(
            8_u32.checked_mul_fraction(&Fraction::new(1, 3)),
            Some(2_u32)
        );
    }

    #[test]
    fn checked_integer_div_fraction() {
        assert_eq!(
            8_u32.checked_div_fraction(&Fraction::new(1, 2)),
            Some(16_u32)
        );

        // the result is not rounded, but truncated (8/3=2.66)
        assert_eq!(
            8_u32.checked_div_fraction(&Fraction::new(3, 1)),
            Some(2_u32)
        );
    }
}
