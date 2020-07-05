use crate::{Error, Period, TimeError};
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
    + TryFrom<u32>
    + TryInto<u32>
    + TryFrom<u64>
    + TryInto<u64>
    + Into<u64>
    + TryFrom<u128>
    + fmt::Display
    + fmt::Debug
{
    /// A checked multiplication with a [`Period`]
    ///
    /// Returns truncated integer
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Period, traits::*};
    /// assert_eq!(8_u32.checked_mul_period(&<Period>::new(1,2)), Some(4_u32));
    ///
    /// // the result is not rounded, but truncated
    /// assert_eq!(8_u32.checked_mul_period(&<Period>::new(1,3)), Some(2_u32));
    /// ```
    fn checked_mul_period<E: Error>(&self, period: &Period) -> Result<Self, TimeError<E>> {
        <Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*period.numerator()).into())
                .ok_or(TimeError::WouldOverflow)?,
            &(*period.denominator()).into(),
        )
        .ok_or(TimeError::WouldDivByZero)
    }

    /// A checked division with a [`Period`]
    ///
    /// Returns truncated integer
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Period, traits::*};
    /// assert_eq!(8_u32.checked_div_period(&<Period>::new(1,2)), Some(16_u32));
    /// assert_eq!(8_u32.checked_div_period(&<Period>::new(3,2)), Some(5_u32));
    /// ```
    fn checked_div_period<E: Error>(&self, period: &Period) -> Result<Self, TimeError<E>> {
        <Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*period.denominator()).into())
                .ok_or(TimeError::WouldOverflow)?,
            &(*period.numerator()).into(),
        )
        .ok_or(TimeError::WouldDivByZero)
    }
}

impl TimeInt for u32 {}

impl TimeInt for u64 {}
