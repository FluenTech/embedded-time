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
/// # use embedded_time::{prelude::*, units::*};
/// assert_eq!(5.nanoseconds(), Nanoseconds(5));
/// assert_eq!(5.microseconds(), Microseconds(5));
/// assert_eq!(5.milliseconds(), Milliseconds(5));
/// assert_eq!(5.seconds(), Seconds(5));
/// assert_eq!(5.minutes(), Minutes(5));
/// assert_eq!(5.hours(), Hours(5));
/// assert_eq!(5.hertz(), Hertz(5));
/// ```
///
/// Signed integers work as well!
/// ```rust
/// # use embedded_time::{prelude::*, units::*};
/// assert_eq!((-5).nanoseconds(), Nanoseconds(-5));
/// assert_eq!((-5).microseconds(), Microseconds(-5));
/// assert_eq!((-5).milliseconds(), Milliseconds(-5));
/// assert_eq!((-5).seconds(), Seconds(-5));
/// assert_eq!((-5).minutes(), Minutes(-5));
/// assert_eq!((-5).hours(), Hours(-5));
/// ```
pub trait TimeInt:
    Copy
    + num::Integer
    + num::Bounded
    + num::traits::WrappingAdd
    + num::traits::WrappingSub
    + num::CheckedMul
    + num::CheckedDiv
    + From<i32>
    + TryInto<i32>
    + TryFrom<i64>
    + Into<i64>
    + fmt::Display
    + fmt::Debug
{
    fn nanoseconds(self) -> Nanoseconds<Self>;
    fn microseconds(self) -> Microseconds<Self>;
    fn milliseconds(self) -> Milliseconds<Self>;
    fn seconds(self) -> Seconds<Self>;
    fn minutes(self) -> Minutes<Self>;
    fn hours(self) -> Hours<Self>;

    fn hertz(self) -> Hertz<Self>;

    /// ```rust
    /// # use embedded_time::{Period, prelude::*};
    /// assert_eq!(8_i32.checked_mul_period(&Period::new(1,2)), Some(4_i32));
    ///
    /// // the result is not rounded, but truncated
    /// assert_eq!(8_i32.checked_mul_period(&Period::new(1,3)), Some(2_i32));
    /// ```
    fn checked_mul_period(&self, period: &Period) -> Option<Self> {
        Some(<Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*period.numerator()).into())?,
            &(*period.denominator()).into(),
        )?)
    }

    /// ```rust
    /// # use embedded_time::{Period, prelude::*};
    /// assert_eq!(8_i32.checked_div_period(&Period::new(1,2)), Some(16_i32));
    /// assert_eq!(8_i32.checked_div_period(&Period::new(3,2)), Some(5_i32));
    /// ```
    fn checked_div_period(&self, period: &Period) -> Option<Self> {
        Some(<Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*period.denominator()).into())?,
            &(*period.numerator()).into(),
        )?)
    }
}

macro_rules! impl_time_ints {
    ($($type:ty),* $(,)?) => {
        $(
            impl TimeInt for $type {
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

impl_time_ints![i32, i64];
