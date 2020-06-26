use crate::frequency::units::Hertz;
use crate::{duration::units::*, Period};
use core::{convert::TryFrom, convert::TryInto, fmt};

/// Create time-based values from primitive and core numeric types.
///
/// This trait can be imported with `use embedded-time::prelude::*`.
///
/// # Examples
/// Basic construction of time-based values.
/// ```rust
/// # use embedded_time::{traits::*, units::*};
/// assert_eq!(5_u32.nanoseconds(), Nanoseconds(5_u32));
/// assert_eq!(5_u32.microseconds(), Microseconds(5_u32));
/// assert_eq!(5_u32.milliseconds(), Milliseconds(5_u32));
/// assert_eq!(5_u32.seconds(), Seconds(5_u32));
/// assert_eq!(5_u32.minutes(), Minutes(5_u32));
/// assert_eq!(5_u32.hours(), Hours(5_u32));
///
/// assert_eq!(5_u32.hertz(), Hertz(5_u32));
/// ```
pub trait TimeInt:
    Copy
    + num::Integer
    + num::Bounded
    + num::traits::WrappingAdd
    + num::traits::WrappingSub
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
    /// ```rust
    /// # use embedded_time::{Period, traits::*};
    /// assert_eq!(8_u32.checked_mul_period(&Period::new(1,2)), Some(4_u32));
    ///
    /// // the result is not rounded, but truncated
    /// assert_eq!(8_u32.checked_mul_period(&Period::new(1,3)), Some(2_u32));
    /// ```
    fn checked_mul_period(&self, period: &Period) -> Option<Self> {
        Some(<Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*period.numerator()).into())?,
            &(*period.denominator()).into(),
        )?)
    }

    /// ```rust
    /// # use embedded_time::{Period, traits::*};
    /// assert_eq!(8_u32.checked_div_period(&Period::new(1,2)), Some(16_u32));
    /// assert_eq!(8_u32.checked_div_period(&Period::new(3,2)), Some(5_u32));
    /// ```
    fn checked_div_period(&self, period: &Period) -> Option<Self> {
        Some(<Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*period.denominator()).into())?,
            &(*period.numerator()).into(),
        )?)
    }
}

/// Construct a time-related type with suffix units
pub trait NumericConstructor: TimeInt {
    /// Construct the [`Duration`](crate::duration::Duration) implementation
    fn nanoseconds(self) -> Nanoseconds<Self>;
    /// Construct the [`Duration`](crate::duration::Duration) implementation
    fn microseconds(self) -> Microseconds<Self>;
    /// Construct the [`Duration`](crate::duration::Duration) implementation
    fn milliseconds(self) -> Milliseconds<Self>;
    /// Construct the [`Duration`](crate::duration::Duration) implementation
    fn seconds(self) -> Seconds<Self>;
    /// Construct the [`Duration`](crate::duration::Duration) implementation
    fn minutes(self) -> Minutes<Self>;
    /// Construct the [`Duration`](crate::duration::Duration) implementation
    fn hours(self) -> Hours<Self>;

    /// Construct the frequency type
    fn hertz(self) -> Hertz<Self>;
}

macro_rules! impl_numeric_constructors {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericConstructor for $type {
                #[inline(always)]
                fn nanoseconds(self) -> Nanoseconds<$type> {
                    Nanoseconds(self)
                }

                #[inline(always)]
                fn microseconds(self) -> Microseconds<$type> {
                    Microseconds(self)
                }

                #[inline(always)]
                fn milliseconds(self) -> Milliseconds<$type> {
                    Milliseconds(self)
                }

                #[inline(always)]
                fn seconds(self) -> Seconds<$type> {
                    Seconds(self)
                }

                #[inline(always)]
                fn minutes(self) -> Minutes<$type> {
                    Minutes(self)
                }

                #[inline(always)]
                fn hours(self) -> Hours<$type> {
                    Hours(self)
                }

                #[inline(always)]
                fn hertz(self) -> Hertz<$type> {
                    Hertz(self)
                }
            }
        )*
    };
}

impl_numeric_constructors![u32, u64];

impl TimeInt for u32 {}

impl TimeInt for u64 {}
