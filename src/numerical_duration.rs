use crate::duration::time_units::*;
use crate::Ratio;
use core::{convert::TryFrom, convert::TryInto, fmt};

/// Create `Duration`s from primitive and core numeric types.
///
/// This trait can be imported with `use embedded-time::prelude::*`.
///
/// # Examples
/// Basic construction of `Duration`s.
/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// assert_eq!(5.nanoseconds(), Nanoseconds(5));
/// assert_eq!(5.microseconds(), Microseconds(5));
/// assert_eq!(5.milliseconds(), Milliseconds(5));
/// assert_eq!(5.seconds(), Seconds(5));
/// assert_eq!(5.minutes(), Minutes(5));
/// assert_eq!(5.hours(), Hours(5));
/// ```
///
/// Signed integers work as well!
/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// assert_eq!((-5).nanoseconds(), Nanoseconds(-5));
/// assert_eq!((-5).microseconds(), Microseconds(-5));
/// assert_eq!((-5).milliseconds(), Milliseconds(-5));
/// assert_eq!((-5).seconds(), Seconds(-5));
/// assert_eq!((-5).minutes(), Minutes(-5));
/// assert_eq!((-5).hours(), Hours(-5));
/// ```
pub trait TimeRep:
    num::PrimInt
    + num::traits::WrappingAdd
    + num::traits::WrappingSub
    + From<i32>
    + TryInto<i32>
    + TryFrom<i64>
    + Into<i64>
    + fmt::Display
    + fmt::Debug
{
    /// Create a `Duration` from the number of nanoseconds.
    fn nanoseconds(self) -> Nanoseconds<Self>;
    /// Create a `Duration` from the number of microseconds.
    fn microseconds(self) -> Microseconds<Self>;
    /// Create a `Duration` from the number of milliseconds.
    fn milliseconds(self) -> Milliseconds<Self>;
    /// Create a `Duration` from the number of seconds.
    fn seconds(self) -> Seconds<Self>;
    /// Create a `Duration` from the number of minutes.
    fn minutes(self) -> Minutes<Self>;
    /// Create a `Duration` from the number of hours.
    fn hours(self) -> Hours<Self>;

    fn checked_mul(&self, ratio: &Ratio<i32>) -> Option<Self> {
        Some(<Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*ratio.numer()).into())?,
            &(*ratio.denom()).into(),
        )?)
    }

    fn checked_div(&self, ratio: &Ratio<i32>) -> Option<Self> {
        Some(<Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*ratio.denom()).into())?,
            &(*ratio.numer()).into(),
        )?)
    }
}

macro_rules! impl_numerical_duration {
    ($($type:ty),* $(,)?) => {
        $(
            /// Create a duration from a primitive integer type
            impl TimeRep for $type {
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
            }
        )*
    };
}

impl_numerical_duration![i32, i64];
